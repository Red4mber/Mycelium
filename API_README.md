# Project: Mycelium API
Mycelium is a Command & Control framework written in rust. This API is the center part of the team server, allowing creation and management of Operators accounts and agents.
# 📁 Collection: Public Endpoints 


## End-point: Login
Log-In Mycelium API using an email and a password.

The response from the API contains a JWT token associated with your account.
### Method: POST
>```
>http://127.0.0.1:3000/login
>```
### Body (**raw**)

```json
{
    "email": "melusine@mycelium.com",
    "password": "TransRights!"
}
```

### 🔑 Authentication noauth

|Param|value|Type|
|---|---|---|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Health Check
This endpoint will always reply with a success message. It's quick way to assess the availability of the API.
### Method: GET
>```
>http://127.0.0.1:3000/healthcheck
>```
### Body (**raw**)

```json

```

### 🔑 Authentication noauth

|Param|value|Type|
|---|---|---|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Ping
You can "Ping" the API by sending any data via a POST request to the `healthcheck` endpoint, which will send it back to you.
### Method: POST
>```
>http://127.0.0.1:3000/healthcheck
>```
### 🔑 Authentication noauth

|Param|value|Type|
|---|---|---|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃
# 📁 Collection: Operator Endpoints 


## End-point: Whoami
A simple GET request on the /operator endpoint will return information about the account used for the request.
### Method: GET
>```
>http://127.0.0.1:3000/operator
>```
### 🔑 Authentication bearer

|Param|value|Type|
|---|---|---|
|token|{{accessToken}}|string|


### Response: 200
```json
{
    "id": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
    "name": "Melusine",
    "email": "melusine@fakemail.lol",
    "password": "$2b$12$AlzNYI/5W98RB4fjtJ9ZfeWfs1ikQPKvs2MGfh0ER3SmUoRJyei7u",
    "role": "Guest",
    "created_by": "00000000-0000-0000-0000-000000000000",
    "created_at": "2024-07-18T02:12:51.577358Z",
    "last_login": "2024-07-20T05:15:14.845965Z"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Query all accounts
Requests a list of every operator accounts on the Mycelium server.
### Method: GET
>```
>http://127.0.0.1:3000/operator/all
>```
### Response: 200
```json
{
    "result": [
        {
            "created_at": "2024-07-18T02:12:51.577358Z",
            "created_by": "00000000-0000-0000-0000-000000000000",
            "id": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
            "name": "Melusine",
            "role": "Guest"
        },
        {
            "created_at": "2024-07-20T06:46:23.702183Z",
            "created_by": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
            "id": "93d04524-575c-4184-8335-0bb54435036f",
            "name": "Example",
            "role": "Guest"
        }
    ],
    "status": "ok"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Query all agents
Requests a list of every operator accounts on the Mycelium server.
### Method: GET
>```
>http://127.0.0.1:3000/operator/all
>```
### Response: 200
```json
{
    "result": [
        {
            "created_at": "2024-07-18T02:12:51.577358Z",
            "created_by": "00000000-0000-0000-0000-000000000000",
            "id": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
            "name": "Melusine",
            "role": "Guest"
        },
        {
            "created_at": "2024-07-20T06:46:23.702183Z",
            "created_by": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
            "id": "93d04524-575c-4184-8335-0bb54435036f",
            "name": "Example",
            "role": "Guest"
        }
    ],
    "status": "ok"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Lookup a specific operator
Allow searching for an operator's account information using the UUID of their account.

If the Operator request their own data, all the data is returned, else the output will be filtered.
### Method: GET
>```
>http://127.0.0.1:3000/operator/{{operator_id}}
>```
### Response: 200
```json
{
    "result": {
        "created_at": "2024-07-18T02:12:51.577358Z",
        "created_by": "00000000-0000-0000-0000-000000000000",
        "email": "melusine@fakemail.lol",
        "id": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
        "last_login": "2024-07-20T05:15:14.845965Z",
        "name": "Melusine",
        "password": "$2b$12$AlzNYI/5W98RB4fjtJ9ZfeWfs1ikQPKvs2MGfh0ER3SmUoRJyei7u",
        "role": "Guest"
    },
    "status": "ok"
}
```

### Response: 200
```json
{
    "Result": "Operator not found"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Lookup a specific agent
Allow searching for an Implant's information using its UUID.
### Method: GET
>```
>http://127.0.0.1:3000/agent/{{agent_id}}
>```
### Response: 200
```json
{
    "result": {
        "created_at": "2024-07-18T02:12:51.577358Z",
        "created_by": "00000000-0000-0000-0000-000000000000",
        "email": "melusine@fakemail.lol",
        "id": "78f95d87-4eeb-44ca-badb-916953d8a9b7",
        "last_login": "2024-07-20T05:15:14.845965Z",
        "name": "Melusine",
        "password": "$2b$12$AlzNYI/5W98RB4fjtJ9ZfeWfs1ikQPKvs2MGfh0ER3SmUoRJyei7u",
        "role": "Guest"
    },
    "status": "ok"
}
```

### Response: 200
```json
{
    "Result": "Operator not found"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Create new account
This endpoint is used to create a new Operator account.

Checks are in place to make sure you cannot create an account more privileged than yours. In addition, _**Guests cannot create accounts**_. The email provided must be unique, as it's the only identifier used to log-in, and a password over 8 characters is necessary just for added security.
### Method: POST
>```
>http://127.0.0.1:3000/operator
>```
### Body (**raw**)

```json
{
	"email": "new.operator@example.com",
	"name": "Example",
	"password": "Choose_A-Str0ng_Passw0rd",
	"role": "Operator"
}
```

### Response: 200
```json
{
    "Result": "Account Example was created successfully"
}
```

### Response: 401
```json
{
    "Error": "Access Denied"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: Delete account
This endpoint is used to delete Operators account.

_**Guests cannot delete accounts and**_ _**Admin accounts cannot be deleted**_**.**

The account to be deleted must be specified using the UUID of said account.
### Method: DELETE
>```
>http://127.0.0.1:3000/operator/dd5498a9-8540-44e2-8a60-a474494c298b
>```
### Response: 200
```json
{
    "Result": "Account `dd5498a9-8540-44e2-8a60-a474494c298b` was successfully deleted"
}
```

### Response: 405
```json
{
    "Error": "Admins cannot be deleted"
}
```


⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃
# 📁 Collection: Agents Endpoints 


## End-point: Basic Beacon
### Method: POST
>```
>http://127.0.0.1:3000/beacon
>```
### Headers

|Content-Type|Value|
|---|---|
|Authorization|c6fb70b3-6d40-47ed-920c-1f205bc0f232|


### Body (**raw**)

```json
{
	"hostname": "DESKTOP-F4K3PC",
	"username": "Melusine",
	"tmpdir": "C:\\Windows\\Temp",
	"appdata": "C:\\Users\\Melusine\\AppData\\Roaming",
	"windir": "C:\\Windows",
	"cwd": "C:\\Users\\Melusine\\Download",
	"cmdline": "LegitInstaller.msi",
	"pid": 12345,
	"env": [
		"ENV_VAR=Veluevalue",
		"TOO_LAZY=to-do-more"
	]
}
```

### 🔑 Authentication noauth

|Param|value|Type|
|---|---|---|



⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃

## End-point: File Upload
This endpoint allows Agents to upload files to the server.

The files will be uploaded to the agent's own directory, which will be created under the general upload directory if it doesn't already exist.

As per the other endpoint, all connections to it will be logged
### Method: GET
>```
>undefined
>```

⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃ ⁃
_________________________________________________
Powered By: [postman-to-markdown](https://github.com/bautistaj/postman-to-markdown/)
