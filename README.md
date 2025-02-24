# CS 361 Microservice: Account Tracking

## Getting Started
### Prerequisites
- **[Rust](https://www.rust-lang.org/tools/install):** Compiling/running the microservice
- **[MySQL](https://www.mysql.com/downloads/):** Microservice dependency for connecting to
    a MySQL database
- **[Docker](https://docs.docker.com/engine/install/):** Hosting a database for local 
    development

### Installation
1. Clone the repo
1. Build the microservice with `cargo build --release`

### Database Setup
To develop locally, [set up a MySQL database](/docs/docker.md) using Docker. To deploy for
production, set up a MySQL database server and note its credentials for configuring this 
microservice.

### Configuration
Create a file named `.env` in the root of this project. Add the following lines to it:
```env
API_KEY={api_key}
DATABASE_URL={database_url}
```
- `{api_key}` should be a long, cryptographically-secure random string used to 
    authenticate to the microservice. If you start the microservice without one defined, 
        it will generate one that you can add to `./env`
- `{database_url}` should be a connection string of the form: 
    ```
    mysql://{username}:{password}@{hostname}:{port}/accounts
    ```
    - `{username}` and `{password}` should be replaced with the username and password you 
        chose when setting up the database
    - `{hostname}` is the domain name or IP address where the database server is hosted. 
        For local development, it should be `127.0.0.1`
    - `{port}` is the port the database server is listening on. For local development, it 
        should be `3306`

### Local Development
1. If the Docker container for the database is stopped, start it with 
    `docker start accounts`.
1. Start the project with `cargo run --release`. This will start the microservice server 
    on port 3000.

## Usage
All API endpoints are [documented](/docs/api_schema.yaml) using the OpenAPI v3.0 format. 
This format can be pretty-printed using [Swagger](https://editor.swagger.io).

### Requests
Every endpoint authenticates requests by checking for an `X-API-Key` header whose value 
matches the API key added to this project's configuration. Failing to include this header 
will cause the request to be rejected. Requests with a body must encode the body using the
JSON format and include a `Content-Type: application/json` header.

#### Example: Account Creation
```py
response = requests.post(
    f'{BASE_URL}/users',
    headers = {
        'Content-Type': 'application/json',
        'X-API-Key': os.environ['API_KEY']
    },
    json = {
        'username': 'test',
        'password': 'password'
    }
)
```

### Responses
Every endpoint will provide a basic status with the HTTP status code. In addition, 
responses for successful requests will include a JSON-encoded response body. Responses for
failed requests will include a plain-text body explaining what error occurred.

#### Example: Retrieving Account ID
```py
if response.status_code == 409:
    print(response.text)
elif response.status_code == 200:
    result = response.json()
    print('Account successfully created!')
    print(f'The user can be consistently identified with the ID {result['id']} in other microservices')
```

### Example Use Sequence
![Sequence diagram showing account creation and session validation](/docs/example_sequence.png)

