<div align="center">
    <h1>SpeedStream</h1>
</div>

<div align="center">
    <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Axum-ff4f00?style=for-the-badge&logo=rust&logoColor=white" alt="Axum" />
    <img src="https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL" />
    <img src="https://img.shields.io/badge/Redis-d82c20?style=for-the-badge&logo=redis&logoColor=white" alt="Redis" />
    <img src="https://img.shields.io/badge/Version-26.2.1-informational?style=for-the-badge" alt="Version" />
</div>

## 📖 Overview

SpeedStream is a fast REST API built with Rust and Axum, specifically designed for real-time speed data
collection and monitoring. Perfect for vehicle tracking speed systems and performance monitoring applications.

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

See the [API Documentation](./docs/ENDPOINTS.md) for detailed information on available endpoints.

## 📊 Architecture Diagram

```mermaid
graph TD
%% Nodes
    A[Arduino Sensor]
    B[Axum Server]
    D[Redis Cache]
    C[PostgreSQL Database]

%% Edges
    A -->|Sends HTTP Request| B
    B -->|Reads/Writes Cache| D
    D -->|Cache Miss| B
    B -->|Queries Database| C
    C -->|Returns Data| B
    B -->|Updates Cache| D
    B -->|Sends HTTP Response| A

%% Styles
    classDef arduino fill:#00979D,stroke:#004d4d,stroke-width:2px,color:#fff,font-weight:bold;
    classDef axum fill:#dea584,stroke:#b36723,stroke-width:2px,color:#000,font-weight:bold;
    classDef postgres fill:#336791,stroke:#1f3d5d,stroke-width:2px,color:#fff,font-weight:bold;
    classDef redis fill:#dc382d,stroke:#8b1d18,stroke-width:2px,color:#fff,font-weight:bold;

    class A arduino;
    class B axum;
    class C postgres;
    class D redis;

```

## 🛠 Code quality

### Unit Tests available

- **Run Tests**:
  ```bash
  cargo test
  ```
  
## 🤝 Contributing

Contributions are welcome! To contribute:

- **Fork the Repository**
- **Create a Feature Branch**:
  ```bash
  git checkout -b feature/your-feature-name
    ```
