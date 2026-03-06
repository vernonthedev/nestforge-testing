# Nestforge-Testing: Nest-like Rust Backend Service

[![Rust](https://img.shields.io/badge/rust-v1.75+-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/framework-Axum-blue.svg)](https://github.com/tokio-rs/axum)
[![NestForge](https://img.shields.io/badge/core-NestForge-green.svg)](https://github.com/vernonthedev/nestforge)

Nestforge-Testing is a robust backend service built with **Rust**, utilizing the **NestForge** framework to provide a modular, scalable architecture inspired by NestJS. It leverages **Axum** for high-performance routing and **Tokio** as the asynchronous runtime.

## Features

- **Modular Architecture**: Organized into distinct modules (e.g., `UsersModule`, `AppModule`) using controllers, services, and providers.
- **RESTful API**: Clean endpoint structure with a global `/api` prefix.
- **Environment Management**: Built-in support for `.env` configuration.
- **User Management**: Complete CRUD operations for handling user data.
- **Test Suite**: Included PowerShell scripts for easy API validation.
- **Health Checks**: Dedicated endpoint for monitoring service status.

## Project Structure

```text
src/
├── main.rs           # Application entry point & bootstrapping
├── app_module.rs     # Core module managing global imports & root controllers
├── users/            # Feature module for user-related logic
│   ├── controllers/  # API route handlers
│   ├── services/     # Business logic & data access
│   ├── dto/          # Data Transfer Objects for requests/responses
│   └── mod.rs        # Users module definition
├── controllers/      # Root/Shared controllers (App, Health)
├── services/         # Root/Shared services
└── dto/, guards/, interceptors/  # Shared utilities
test/                 # PowerShell scripts for API testing
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd nestforge-testing
   ```

2. Configure environment variables:

   ```bash
   cp .env.example .env
   ```

   _Edit `.env` as needed._

3. Run the application:
   ```bash
   cargo run
   ```
   The server will start at `http://localhost:3000/api`.

## API Endpoints

The service uses **versioned** RESTful endpoints. The global prefix is `/api`.

| Method   | Endpoint            | Version | Description           |
| :------- | :------------------ | :------ | :-------------------- |
| `GET`    | `/api/`             | -       | Welcome message       |
| `GET`    | `/api/health`       | -       | Service health status |
| `GET`    | `/api/v1/users`     | v1      | List all users        |
| `GET`    | `/api/v2/users/:id` | v2      | Get user by ID        |
| `POST`   | `/api/v2/users`     | v2      | Create a new user     |
| `PUT`    | `/api/v2/users/:id` | v2      | Update user details   |
| `DELETE` | `/api/v2/users/:id` | v2      | Remove a user         |

## Testing

The project includes a suite of PowerShell scripts in the `test/` directory to facilitate manual testing and API validation:

- `create.ps1`: Test user creation.
- `delete.ps1`: Test user deletion.
- `get_user.ps1`: Test fetching a specific user.
- `update.ps1`: Test user updates.

To run a test (example):

```powershell
./test/users/create.ps1
```

## Technologies Used

- **Framework**: [NestForge](https://github.com/vernonthedev/nestforge) (NestJS-inspired architecture)
- **Web Server**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Serialization**: [Serde](https://serde.rs/)
- **Error Handling**: [Anyhow](https://github.com/dtolnay/anyhow)
- **Environment**: [Dotenv](https://github.com/dotenv-rs/dotenv)

---

Developed using Nestforge.
