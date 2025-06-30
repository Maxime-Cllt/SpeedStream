<div align="center">
    <h1>SpeedStream</h1>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Version-1.0.0-informational?style=for-the-badge" alt="Version" />
</div>

## 📖 Overview

API made with Rust and Axum, designed to handle speed data from sensor and store it in a PostgreSQL database. It
provides endpoints to retrieve and submit speed data, making it suitable for applications that require real-time speed
monitoring.

## ✨ Key Features

## 💻 Platform Support

<div align="center">
  <a href="#macos">
    <img src="https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white&labelColor=gray" alt="macOS" />
  </a>
  <a href="#linux">
    <img src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black&labelColor=gray" alt="Linux" />
  </a>
  <a href="#windows">  
    <img src="https://img.shields.io/badge/Windows-0078D4?style=for-the-badge&logo=windows&logoColor=white&labelColor=gray" alt="Windows" />
  </a>
</div>

## 📋 Prerequisites

- **Rust Compiler** (Install via [Rustup](https://rustup.rs/))
- **Cargo Package Manager** (Installed with Rust)

## 🚀️ Endpoints

- **GET /health**: Check the health status of the API.
- **POST /api/create-speed**: Submit new speed data into the database.
- **GET /api/get-speed**: Retrieve speed data from the database.
- **GET /api/get-speed?limit=500**: Retrieve speed data with a limit on the number of records returned.

## Example Usage

### 1. Insert Speed Data

```bash
curl -X POST http://localhost:3000/api/create-speed \
  -H "Content-Type: application/json" \
  -d '{
    "speed":145
  }'
```

### 2. Retrieve Speed Data

```bash
curl -X GET http://localhost:3000/api/get-speed?limit=500
```

## 🤝 Contributing

Contributions are welcome! To contribute:

- **Fork the Repository**
- **Create a Feature Branch**:
  ```bash
  git checkout -b feature/your-feature-name
    ```

<p align="center">
  Made with 🦀
</p>