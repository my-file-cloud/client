# my-file-cloud-client

Dioxus

## Routing

| Route          | Component | Description                                                                                                                                             |
|----------------|-----------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| /              | Index     | Initial Page. Decide if Session can be refreshed. Auto-Link to /dashboard or /login, depending on Auth status                                           |
| /login         | Login     | Login Page. User can enter Login credentials and receive a session. On login, route to /. User may choose to route to /register                         |
| /register      | Register  | Register Page. User can enter Register credentials and create an account. On account creation, route to /login. User may choose to route to /login      |
| /dashboard     | Dashboard | *TODO*                                                                                                                                                  |
| /browse        | Browse    | Default Browse page                                                                                                                                     |
| /browse/{path} | Browse    | Browse given path. (Just print out the path value). path may be following pattern: `my-path`, `my-path/`, `my-path/nested`, `my-path/s/o/f/a/r/nested/` |
