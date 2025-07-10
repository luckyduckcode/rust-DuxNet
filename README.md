# DuxNet - Decentralized P2P Platform

A high-performance, decentralized peer-to-peer platform for service discovery, task processing, and secure payments without smart contracts.

## Features

- **Decentralized Service Registry**: Distributed Hash Table (DHT) for service discovery
- **P2P Network**: Pure peer-to-peer communication using libp2p
- **Decentralized Identity**: Self-sovereign identities using DIDs
- **Reputation System**: Trust-based scoring with peer attestations
- **Escrow System**: Multi-signature escrow for secure payments
- **Task Processing**: Distributed task execution and verification
- **Modern Web UI**: Beautiful, responsive interface

## Architecture

### Core Components

1. **DHT (Distributed Hash Table)**: Service discovery and metadata storage
2. **P2P Network**: Node communication using libp2p
3. **DID System**: Decentralized identity management
4. **Reputation System**: Trust scoring and attestations
5. **Escrow Manager**: Multi-signature payment escrow
6. **Task Engine**: Distributed task processing
7. **Web API**: RESTful API for frontend integration

### Technology Stack

- **Backend**: Rust with Tokio async runtime
- **P2P Networking**: libp2p (Kademlia DHT, Floodsub, mDNS)
- **Web Framework**: Axum
- **Frontend**: HTML5, CSS3, JavaScript
- **Cryptography**: ed25519-dalek for signatures
- **Serialization**: Serde JSON

## Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- Git

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd DuxNet-APIstore
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

The application will start:
- P2P node on port 8080
- Web API on port 8081
- Web interface at http://localhost:8081

### Usage

1. **Register a Service**:
   - Navigate to the web interface
   - Fill in service details (name, description, price)
   - Click "Register Service"

2. **Discover Services**:
   - Use the search functionality to find available services
   - View service details and provider reputation

3. **Submit Tasks**:
   - Enter service ID and task payload
   - Set resource requirements (CPU, memory, timeout)
   - Submit for processing

4. **Create Escrow**:
   - Set up secure payment escrow for services
   - Multi-signature protection for funds

## API Endpoints

### Node Status
- `GET /api/status` - Get node status and information

### Services
- `POST /api/services/register` - Register a new service
- `POST /api/services/search` - Search for services

### Tasks
- `POST /api/tasks/submit` - Submit a task for processing

### Escrow
- `POST /api/escrow/create` - Create a new escrow contract

### Reputation
- `GET /api/reputation/:did` - Get reputation score for a DID

### Statistics
- `GET /api/stats` - Get comprehensive network statistics

## Development

### Project Structure

```
src/
├── main.rs              # Application entry point
├── core/                # Core business logic
│   ├── mod.rs          # Core module
│   ├── data_structures.rs # Data structures
│   ├── dht.rs          # Distributed Hash Table
│   ├── identity.rs     # DID system
│   ├── reputation.rs   # Reputation system
│   ├── escrow.rs       # Escrow management
│   └── tasks.rs        # Task processing
├── network/            # P2P networking
│   └── mod.rs          # libp2p integration
├── api/                # Web API
│   └── mod.rs          # Axum routes
├── wallet/             # Wallet functionality
│   └── mod.rs          # Crypto wallet
└── frontend/           # Frontend integration
    └── mod.rs          # Static file serving

static/                 # Frontend assets
├── index.html          # Main HTML file
├── style.css           # CSS styles
└── script.js           # JavaScript functionality
```

### Building for Development

```bash
# Development build
cargo build

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check formatting
cargo fmt

# Lint code
cargo clippy
```

### Adding New Features

1. **New Service Type**: Extend `ServiceMetadata` in `data_structures.rs`
2. **New Task Type**: Add to `Task` enum and implement in `tasks.rs`
3. **New API Endpoint**: Add route in `api/mod.rs`
4. **Frontend Feature**: Update HTML/CSS/JS in `static/` directory

## Configuration

### Environment Variables

- `RUST_LOG`: Logging level (debug, info, warn, error)
- `P2P_PORT`: P2P network port (default: 8080)
- `API_PORT`: Web API port (default: 8081)

### Network Configuration

The P2P network supports:
- TCP transport with Noise encryption
- Kademlia DHT for service discovery
- Floodsub for pub/sub messaging
- mDNS for local peer discovery

## Security

- **Cryptographic Signatures**: All messages signed with ed25519
- **DID Authentication**: Self-sovereign identity verification
- **Multi-signature Escrow**: Secure payment protection
- **Reputation System**: Sybil attack resistance
- **P2P Encryption**: End-to-end encrypted communication

## Performance

- **Async Runtime**: Tokio for high-performance async operations
- **Memory Safety**: Rust's ownership system prevents memory issues
- **Concurrent Processing**: Multi-threaded task execution
- **Efficient DHT**: O(log n) lookup complexity

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Code Style

- Follow Rust conventions
- Use meaningful variable names
- Add documentation comments
- Include error handling
- Write unit tests

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Trust Wallet integration for crypto payments
- [ ] Advanced consensus mechanisms
- [ ] Mobile application
- [ ] Plugin system for custom services
- [ ] Advanced reputation algorithms
- [ ] Cross-chain compatibility
- [ ] Formal verification
- [ ] Performance optimizations

## Support

For questions and support:
- Create an issue on GitHub
- Check the documentation
- Join the community discussions

## Acknowledgments

- libp2p team for the excellent P2P networking library
- Rust community for the amazing ecosystem
- All contributors and supporters 