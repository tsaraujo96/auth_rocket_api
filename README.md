
# Auth Rocket Api

Testing simple api template using diesel with JWT authentication.
## Install

To start the application, you must be using the nightly version of rust.

Because the application Cors (rocket-cors)  is not yet stable.

```bash
  rustup default nightly
```

After intall nightly version, you need up postgresql database on docker, browser to the deploy folder and run this command:


```bash
  docker-compose up -d
```

Now you can run your application using, remember to go back to the project initial folder. This command will compile your application and run it.

```bash
  cargo run
```
    
## API documentation

#### Returns all registered users - *need authentication*

```http
  GET /get_users
```

#### Create an user  - *dont need authentication*

```http
  POST /create_user
```

#### Body request 

| Parameter   | Type       | Describe                                   |
| :---------- | :--------- | :------------------------------------------ |
| `email`      | `string` | **Mandatory**. User email of your choice (does not have a validator)  |
| `password`      | `string` | **Mandatory**. Password of your choice (does not have a validator)  |


#### Login 

```http
  POST /login
```

#### Form-data request 

| Parameter   | Type       |
| :---------- | :--------- | 
| `email`      | `string` | 
| `password`      | `string` |

Return **Bearer Token using JWT**



