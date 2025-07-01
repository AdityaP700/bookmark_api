```mermaid
flowchart TD
    A[Frontend (or Postman)] --> B[HTTP Request (POST / GET / DELETE)]
    B --> C[Actix Web Server (main.rs)]
    C --> D[Router<br/>(decides which handler to call)]
    D --> E[Handlers<br/>(logic for each route)]
    E --> F[Models<br/>(Bookmark struct)]
    F --> G[Storage<br/>(in-memory HashMap for now)]
```