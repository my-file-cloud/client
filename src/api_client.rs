use std::fmt;
use std::fmt::Formatter;
use reqwest::Response;
use std::path::PathBuf;
use async_trait::async_trait;
use dioxus::prelude::*;
use http::{Method, StatusCode};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use my_file_cloud_api::ApiErrorBody;
use my_file_cloud_api::route::auth::login::LoginBody;
use my_file_cloud_api::route::auth::register::RegisterBody;
use my_file_cloud_api::route::browse::BrowseResponseDTO;
use my_file_cloud_api::route::dashboard::DashboardResponse;
use web_sys::{Request, RequestCredentials, RequestInit};


#[async_trait(?Send)]
pub trait MapIntoApiClientError: Sized {
    async fn map_response_to_api_client_error(self) -> Result<Self, ApiClientError>;
    async fn map_response_to_api_client_error_with_body<T: DeserializeOwned>(self) -> Result<T, ApiClientError>;
}

#[async_trait(?Send)]
impl MapIntoApiClientError for Response {
    async fn map_response_to_api_client_error(self) -> Result<Self, ApiClientError>
    {
        if self.status().is_success() {
            Ok(self)
        } else {
            Err(ApiClientError::Api(
                self.json()
                    .await.unwrap()
            ))
        }
    }

    async fn map_response_to_api_client_error_with_body<T: DeserializeOwned>(self) -> Result<T, ApiClientError> {
        let response = self.map_response_to_api_client_error().await?;
        
        response.json::<T>().await
            .map_err(|err|
                ApiClientError::Api(ApiErrorBody{
                    message: format!("Could not parse expected body: {err}"),
                })
            )
    }
}

#[derive(Debug)]
pub enum ApiClientError {
    Request(reqwest::Error),
    Api(ApiErrorBody),
}
impl std::error::Error for ApiClientError {}
impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(err) => write!(f, "Failed to send request: {err}"),
            Self::Api(body) => write!(f, "{}", body.message),
        }
    }
}

#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    pub base_url: String,
    pub is_authenticated: Signal<bool>,
}
impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: build_http_client(),
            base_url,
            is_authenticated: use_signal(|| false),
        }
    }

    pub fn request_builder(&self, method: Method, sub_url: impl ToString, with_credentials: bool) -> RequestBuilder {
        let mut builder = self.client.request(
            method,
            format!("{}{}", self.base_url, sub_url.to_string())
        );

        #[cfg(target_arch = "wasm32")]
        {
            if with_credentials {
                builder = builder.fetch_credentials_include();
            }
        }

        builder
    }
    
    pub async fn register(&self, body: RegisterBody) -> Result<(), ApiClientError> {
        self.request_builder(Method::POST, "/auth/register", false)
            .json(&body)
            .send().await
            .map_err(ApiClientError::Request)?
            .map_response_to_api_client_error().await?;
        
        Ok(())
    }
    pub async fn login(&mut self, body: LoginBody) -> Result<(), ApiClientError> {
        let response = self.request_builder(Method::POST, "/auth/login", true)
            .json(&body)
            .send().await
            .map_err(ApiClientError::Request)?;
        
        match response.map_response_to_api_client_error().await {
            Err(err) => Err(err),
            Ok(_) => {
                self.is_authenticated.set(true);
                Ok(())
            }
        }
    }
    pub async fn refresh(&mut self) -> Result<(), ApiClientError> {
        let response = self.request_builder(Method::POST, "/auth/refresh", true)
            .send().await
            .map_err(ApiClientError::Request)?;

        match response.map_response_to_api_client_error().await {
            Err(err) => Err(err),
            Ok(_) => {
                self.is_authenticated.set(true);
                Ok(())
            }
        }
    }
    pub async fn logout(&mut self) -> Result<(), ApiClientError> {
        let response = self.request_builder(Method::POST, "/auth/logout", true)
            .send().await
            .map_err(ApiClientError::Request)?;
        
        self.is_authenticated.set(false);
        
        response.map_response_to_api_client_error().await
            .map(|_| ())
    }
    
    pub async fn dashboard(&self) -> Result<DashboardResponse, ApiClientError> {
        let response = self.request_builder(Method::GET, "/dashboard", true)
            .send().await
            .map_err(ApiClientError::Request)?;
        
        response.map_response_to_api_client_error_with_body().await
    }

    pub async fn browse(&self, dir: String) -> Result<BrowseResponseDTO, ApiClientError> {
        let response = self.request_builder(Method::GET, format!("/browse{dir}"), true)
            .send().await
            .map_err(ApiClientError::Request)?;

        match response.map_response_to_api_client_error_with_body().await {
            Err(err) => Err(err),
            Ok(mut response) => {
                if let BrowseResponseDTO::Directory(ref mut items) = response {
                    items.sort_by(|a, b| a.storage_content_type.cmp(&b.storage_content_type));
                }
                
                Ok(response)
            }
        }
    }
    pub async fn delete(&self, path: String) -> Result<(), ApiClientError> {
        self.request_builder(Method::DELETE, format!("/browse{path}"), true)
            .send().await
            .map_err(ApiClientError::Request)?
            .map_response_to_api_client_error().await?;
        
        Ok(())
    }

    pub async fn create_directory(&self, path: String) -> Result<(), ApiClientError> {
        self.request_builder(Method::POST, format!("/create-directory{path}"), true)
            .send().await
            .map_err(ApiClientError::Request)?
            .map_response_to_api_client_error().await?;
        
        Ok(())
    }

    pub async fn upload(&self, path: PathBuf, file: gloo_file::File) -> Result<(), ApiClientError> {
        let path = path.to_string_lossy().as_ref().to_string();

        let form_data = web_sys::FormData::new().expect("Failed to create FormData");
        form_data.append_with_blob("file", file.as_ref()).expect("Failed to append file");

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&form_data);
        opts.set_credentials(RequestCredentials::Include);

        let request_url = &self.base_url;
        let request = Request::new_with_str_and_init(&format!("{request_url}/upload{path}"), &opts).expect("Failed to create API Request");

        request.headers().set("Accept", "application/json").expect("Failed to append Accept Header");

        let window = web_sys::window().expect("No web_sys::window available");
        let res: web_sys::Response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await.expect("Failed to create JsFuture")
            .dyn_into().expect("Failed convert JsFuture to Response");

        match StatusCode::from_u16(res.status()) {
            Err(err) => error!("{err}"),
            Ok(status) => {
                if !status.is_success() {
                    error!("failed to upload ({status}): {}", res.text().unwrap().to_string());
                    return Err(ApiClientError::Api(ApiErrorBody{
                        message: status.to_string()
                    }));
                }
            }
        }

        Ok(())
    }
}

fn build_http_client() -> reqwest::Client {
    let builder = reqwest::Client::builder();
    
    #[cfg(not(target_arch = "wasm32"))]
    let builder = builder.cookie_store(true);
    
    builder
        .build()
        .expect("failed to build reqwest client")
}