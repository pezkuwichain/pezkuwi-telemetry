# Pezkuwichain Telemetry

## Overview

This repository contains the Telemetry server for Pezkuwichain nodes. It includes:

- **Backend**: Rust-based ingestion server (`telemetry_shard` and `telemetry_core`)
- **Frontend**: React/TypeScript dashboard for monitoring network nodes

Pezkuwichain nodes can connect to Telemetry using the `--telemetry-url` flag.

### Messages

Nodes send telemetry messages depending on the verbosity level:

**Verbosity 0 (default):**
```
system.connected
system.interval
block.import
notify.finalized
```

**Verbosity 1 (includes validator address):**
```
afg.authority_set
```

## Getting Started

### Prerequisites

- Rust (via rustup): https://rustup.rs/
- Node.js (LTS version)

### Backend

```bash
cd backend
cargo build --release

# Terminal 1 - Core server
./target/release/telemetry_core

# Terminal 2 - Shard server
./target/release/telemetry_shard
```

Default ports:
- `telemetry_core`: 127.0.0.1:8000
- `telemetry_shard`: 127.0.0.1:8001

### Frontend

```bash
cd frontend
npm install
npm run start
```

Access the UI at http://localhost:3000

### Connect a Node

```bash
pezkuwi --dev --telemetry-url 'ws://localhost:8001/submit 0'
```

Note: The "0" is the verbosity level (0-9).

## Docker

### Build Images

```bash
# Backend
cd backend
docker build -t pezkuwichain/telemetry-backend .

# Frontend
cd frontend
docker build -t pezkuwichain/telemetry-frontend .
```

### Docker Compose

```bash
docker-compose up
```

### Manual Docker Setup

```bash
# Create network
docker network create telemetry

# Backend core
docker run -d --network=telemetry --name backend-core \
  -p 8000:8000 --read-only \
  pezkuwichain/telemetry-backend \
  telemetry_core -l 0.0.0.0:8000

# Backend shard
docker run -d --network=telemetry --name backend-shard \
  -p 8001:8001 --read-only \
  pezkuwichain/telemetry-backend \
  telemetry_shard -l 0.0.0.0:8001 -c http://backend-core:8000/shard_submit

# Frontend
docker run -d --network=telemetry --name frontend \
  -p 3000:8000 \
  -e SUBSTRATE_TELEMETRY_URL=ws://localhost:8000/feed \
  pezkuwichain/telemetry-frontend
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SUBSTRATE_TELEMETRY_URL` | WebSocket URL for frontend to connect | `ws://localhost:8000/feed` |

## Links

- **Website**: https://pezkuwichain.io
- **Telemetry**: https://telemetry.pezkuwichain.io
- **Discord**: https://discord.gg/Y3VyEC6h8W

## License

GPL-3.0

## Credits

Based on [substrate-telemetry](https://github.com/paritytech/substrate-telemetry) by Parity Technologies.

Maintained by Dijital Kurdistan Tech Institute.
