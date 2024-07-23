# Mycelium 

A simple C2 server written in rust. 
This project just started, so it is barebones right now.

All endpoints of the API are configurable via the settings.toml file.
A rudimentary implant is also available to test the API.

The API needs a postgresql database, the migrations used to set it up during development are in the `/migrations` folder.

All endpoints of the API need authentication, and only a registered operator can add a new account,
so a test user need to be added manually to the database.

The operators first login via the `/login` endpoint and are then authentified via a JWT token.

THERE IS NO GUI YET. All interactions with the server is done via the API.

I'm working on writing documentation for the API using Postman, it should be coming soon :)


 