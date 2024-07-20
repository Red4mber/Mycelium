# Mycelium 

A simple C2 server written in rust.

So far it is just an API, with JWT Authentication, to manage users and implants.
This project barely started, so it is super barebones right now, but the rest is coming :)

### JWT key generation

You can generate a PEM file containing an ECDSA key using the following command : 
```bash
openssl ecparam -genkey -noout -name prime256v1 | openssl pkcs8 -topk8 -nocrypt -out ec-private.pem
```