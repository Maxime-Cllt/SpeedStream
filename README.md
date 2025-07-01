<div align="center">
    <h1>SpeedStream</h1>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Axum-ff4f00?style=for-the-badge&logo=rust&logoColor=white" alt="Axum" />
    <img src="https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL" />
    <img src="https://img.shields.io/badge/Version-1.0.0-informational?style=for-the-badge" alt="Version" />
</div>

## 📖 Overview

SpeedStream is a blazingly fast REST API built with Rust and Axum, specifically designed for real-time speed data
collection and monitoring. Perfect for IoT sensors, vehicle tracking systems, and performance monitoring applications.

## ✨ Key Features

- ⚡ Ultra-Fast: Can handle thousands of requests per second with minimal latency
- 🔒 Memory Safe: Built with Rust's zero-cost abstractions and memory safety
- 🏗️ Production Ready: Comprehensive error handling, logging, and health checks
- 🌐 IoT Friendly: Optimized for Arduino, Raspberry Pi, and other embedded devices

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
    <a href="#docker">
        <img src="https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white&labelColor=gray" alt="Docker" />
    </a>
</div>

## 📋 Prerequisites

- **Rust Compiler** (Install via [Rustup](https://rustup.rs/))
- **Cargo Package Manager** (Installed with Rust)

## 🚀️ Endpoints

- **GET /health**: Check the health status of the API.
- **POST /api/create-speed**: Submit new speed data into the database.
- **GET /api/get-speed?limit=500**: Retrieve speed data with a limit on the number of records returned.
- **GET /api/get-speed/pagination?offset=0&limit=500**: Retrieve speed data with pagination support.
- **GET /api/get-speed/today**: Retrieve speed data for today with pagination support.

## 🧪 Example Usage

### 1. Insert Speed Data

```bash
curl -X POST http://localhost:3000/api/create-speed \
  -H "Content-Type: application/json" \
  -d '{
    "speed":145
  }'
```

### 2. Retrieve last n records of Speed Data

```bash
curl -X GET http://localhost:3000/api/get-speed?limit=500
```

### 3. Retrieve Speed Data with Pagination

```bash
curl -X GET http://localhost:3000/api/get-speed/pagination?offset=0&limit=500
```

## 📊 Architecture Diagram

```mermaid
graph TD
%% Nodes
  A[Arduino Sensor]
  B[Axum Server]
  C[PostgreSQL Database]

%% Edges
  A -->|Sends HTTP Request| B
  B -->|Queries Database| C
  C -->|Returns Data| B
  B -->|Sends HTTP Response| A

%% Styles
  classDef arduino fill:#00979D,stroke:#004d4d,stroke-width:2px,color:#fff,font-weight:bold;
  classDef axum fill:#dea584,stroke:#b36723,stroke-width:2px,color:#000,font-weight:bold;
  classDef postgres fill:#336791,stroke:#1f3d5d,stroke-width:2px,color:#fff,font-weight:bold;

  class A arduino;
  class B axum;
  class C postgres;
```

## 🤝 Contributing

Contributions are welcome! To contribute:

- **Fork the Repository**
- **Create a Feature Branch**:
  ```bash
  git checkout -b feature/your-feature-name
    ```