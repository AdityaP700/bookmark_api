Frontend (or Postman) → HTTP Request (POST / GET / DELETE)
           |
           v
    Actix Web Server (main.rs)
           |
     ┌──────────────┐
     │   Router     │  => decides which handler to call
     └──────────────┘
           |
           v
     ┌──────────────┐
     │  Handlers    │  => logic for each route
     └──────────────┘
           |
           v
     ┌──────────────┐
     │   Models     │  => Bookmark struct
     └──────────────┘
           |
           v
     ┌──────────────┐
     │   Storage    │  => in-memory HashMap (for now)
     └──────────────┘
