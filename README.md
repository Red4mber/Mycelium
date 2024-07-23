# Mycelium

Mycelium is a lightweight Command and Control (C2) server written in Rust. This project is in its early stages and currently offers basic functionality.

## Features

- Configurable API endpoints via `settings.toml` 
    
- Basic implant for API testing with rudimentary enumeration using [Thermite](https://github.com/Red4mber/Thermite), my malware development rust library.
- PostgreSQL database integration.
- JWT-based authentication for operators

## Getting Started

### Prerequisites

- Docker and Docker Compose 
    > (or your own PostgreSQL database, but then, you would probably know what you're doing)

### Building and running Mycelium C2

1. Clone the repository:

```shell
git clone https://github.com/Red4mber/Mycelium.git
cd Mycelium
```
2. Build and run the server:

To build and run Mycelium, start the server by running:
```shell
docker compose up --build
```

The initial build may take 1-2 minutes. Once complete, the server will be available at http://localhost:3000.

### Database

- Migrations for setting up the database are located in the `/migrations` folder.
- The database is pre-populated with dummy values for development and testing.

## API Documentation

[![Run in Postman](https://run.pstmn.io/button.svg)](https://god.gw.postman.com/run-collection/37113998-de34f7fc-c4f1-493e-b835-4a2ab3d4298c?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D37113998-de34f7fc-c4f1-493e-b835-4a2ab3d4298c%26entityType%3Dcollection%26workspaceId%3D8aee9eed-9f7c-4150-aca2-844b29f39ac7)


The documentation is only valid with the configuration provided. If you change the endpoints in your own installation, i suggest forking the postman collection to make your own documentation.

Some endpoints are still missing from the documentation, such as `/beacon`. See [settings.toml](settings.toml) to see all the endpoints.


## Authentication

- All API endpoints require authentication.
- Operators must first login via the `/login` endpoint to receive a JWT token for authentication.
- Implant only authenticate using their UUID so far, their authentication mecanism being more complex I still need time to work on it.
- Only registered operators can add new accounts.
- A default user "Melusine" is pre-configured in the database with the default password `TransRights!` for initial access. 

## Current Limitations

- No GUI available yet; all interactions are currently done through the API.
- Project is in very early development; features may be limited or subject to change.

## Contributing

We welcome contributions! Please feel free to submit pull requests or open issues for any bugs or feature requests.

