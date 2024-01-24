# user-auth-rs
Welcome to `user-auth-rs`, a simple user authentication project built using the Actix web framework in Rust. This project leverages the Tokio runtime to provide efficient and scalable asynchronous I/O operations.

## Overview

This is my first Rust project, and I've chosen Actix due to its strong async I/O capabilities. The initial release includes basic functionality with three key endpoints:

- **`/register`**
    - Endpoint for user registration.
    - This allows users to create an account by providing necessary information.

- **`/login`**
    - Endpoint for user login.
    - Users can log in using their credentials to obtain access tokens.

- **`/get_user`**
    - Endpoint to retrieve user information.
    - This provides basic user details based on the provided authentication token.

## Future Development

I plan to expand this project by adding more features, including:

- **Authorization:** Enhance security by implementing token-based authorization mechanisms.

- **User Profile Management:** Allow users to update their profiles, change passwords, and manage account settings.

- **Password Recovery:** Implement a secure process for users to recover or reset their passwords.



## Table of Contents
- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Usage Examples](#usage-examples)


## Getting Started

Provide instructions on how to install, configure, and run the application.

```bash
# Clone the repository
git clone https://github.com/your-username/user-auth-rs.git

# Change into the project directory
cd user-auth-rs

# Create .env file and add below line 
BACKENDPORT=8080
MONGODB_URI=mongodb://localhost:27017/rusty
SECRET_KEY=user-auth-rs 

# Install dependencies
cargo build

# Run the application
cargo run

```

## Configurations 

- **MongoDB:** Config string of local MongoDB or Cloud 

- **SECRET_KEY:** Any string you can set 

## Usage Examples
Explore basic usage examples to interact with the provided endpoints using a tool like Postman:

1. Register a new user:
- Send a POST request to `http:localhost:BACKENDPORT/register` with a RAW body:
```json
{
  "username": "your_username",
  "password": "your_password"
}
```
2. Login and obtain access tokens:
- Send a POST request to `http://localhost:BACKENDPORT/login` with the registered credentials.
```json
{
  "username": "your_username",
  "password": "your_password"
}
```
3. Retrieve user information:
- Send a GET request to `http://localhost:BACKENDPORT/get_user` with the obtained access token in the request headers.