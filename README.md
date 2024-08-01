# Mycelium

Mycelium is a lightweight Command and Control (C2) server written in Rust. This project is in its early stages and currently offers basic functionality.
I just finished migrating the entire project from PostgreSQL to SurrealDB - It  took me an eternity, but it's done, I can finally get back working on meaningful changes to this

## Features

- Configurable API endpoints via `settings.toml`
- Basic implant for API testing with rudimentary enumeration using [Thermite](https://github.com/Red4mber/Thermite), my malware development rust library.
- SurrealDB database integration.
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

(Super outdated)
[<img src="https://run.pstmn.io/button.svg" alt="Run In Postman" style="width: 128px; height: 32px;">](https://god.gw.postman.com/run-collection/37113998-034e0471-7c27-49a1-bcbf-df7905ac0989?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D37113998-034e0471-7c27-49a1-bcbf-df7905ac0989%26entityType%3Dcollection%26workspaceId%3D8786ad64-fd37-4dcc-a897-dc9fefa82077#?env%5BPublic%5D=W3sia2V5IjoicGFzc3dvcmQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWUsInR5cGUiOiJzZWNyZXQifSx7ImtleSI6ImFjY2Vzc1Rva2VuIiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlLCJ0eXBlIjoiYW55In0seyJrZXkiOiJmaWxlbmFtZSIsInZhbHVlIjoiaW1hZ2UuanBnIiwiZW5hYmxlZCI6dHJ1ZX1d)

Took me what felt like years, but i finally cleaned Postman from everything it considers "private" so i can finally publish the documentation online.\
> Online API documentation : 
> https://documenter.getpostman.com/view/37113998/2sA3kXCzjz


## Authentication

- All API endpoints require authentication.
- Operators must first login via the `/login` endpoint to receive a JWT token for authentication.
- Implant only authenticate using their UUID so far, their authentication mecanism being more complex I still need time to work on it.
- Only registered operators can add new accounts.

## Current Limitations

- No GUI available yet; all interactions are currently done through the API.
- Project is in very early development; features may be limited or subject to change.

## Contributing

We welcome contributions! Please feel free to submit pull requests or open issues for any bugs or feature requests.

