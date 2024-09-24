# Retail SmartOps Backend

## Overview

Retail SmartOps Backend is a Rust-based backend service for managing user roles and user data in a retail application.
It uses Axum for the web framework and SQLx for database interactions.

## Features

- User role management
- User data management
- PostgreSQL database integration

## Getting Started

### Prerequisites

- Rust
- Cargo
- PostgreSQL

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/retail-smartops-backend.git
    cd retail-smartops-backend
    ```

2. Set up the PostgreSQL database and update the database URL in the `.env` file.

3. Run the application:
    ```sh
    cargo run
    ```

## Usage

- The backend provides APIs for managing users and user roles.
- Refer to the `src/repositories/user.rs` and `src/repositories/user_role.rs` for implementation details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

MIT License

Copyright (c) 2023 Revanth Shalon Raj

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.