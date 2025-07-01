## Architecture

<details>
<summary>Click to view as text if diagram does not render</summary>

```
Frontend (or Postman)
        │
        ▼
  HTTP Request (POST / GET / DELETE)
        │
        ▼
  Actix Web Server (main.rs)
        │
        ▼
     Router (decides which handler to call)
        │
        ▼
     Handlers (logic for each route)
        │
        ▼
     Models (Bookmark struct)
        │
        ▼
     Storage (in-memory HashMap for now)
```
</details>

```mermaid
flowchart TD
    A[Frontend (or Postman)] --> B[HTTP Request (POST / GET / DELETE)]
    B --> C[Actix Web Server (main.rs)]
    C --> D[Router (decides which handler to call)]
    D --> E[Handlers (logic for each route)]
    E --> F[Models (Bookmark struct)]
    F --> G[Storage (in-memory HashMap for now)]
```