# Wow Engine

Wow Engine is a high-performance, modular Rust-based bridging and routing service. It is designed to route multi-chain tokens into the Stellar network and facilitate instant fiat on-ramping and off-ramping via Stellar anchors.

This service acts as the shared transaction backend for the Whales of Wallstreet (WOW) ecosystem, including the Web App and Native App.

## How It Works

The Wow Engine coordinates cross-chain liquidity transfers and fiat gateway aggregation:

1. **Cross-Chain Routing**: Integrates with bridging protocols like Circle CCTP (Cross-Chain Transfer Protocol) and deBridge DLN to route assets from external networks such as Ethereum, Solana or Arbitrum into the Stellar network.
2. **Optimal Pathfinding**: The internal pathfinding router queries available bridge providers, evaluates estimated gas fees and completion times, and ranks the execution paths by highest output yield and lowest cost.
3. **Stellar Anchor On-Ramping and Off-Ramping**: The engine uses Stellar Ecosystem Proposals (SEP-24 for interactive deposits/withdrawals and SEP-38 for quotes) to bridge and exchange tokens between on-chain assets and local fiat currencies globally.

## Developer Integration

The engine is structured as an integratable on-ramp and off-ramp service, exposing a clean REST API so client applications do not need to implement complex cryptographic or cross-chain coordination logic locally. 

### API Endpoints

The server listens on port 8080 and provides the following endpoints:

- `GET /api/v1/health`: Checks the service status and version.
- `POST /api/v1/quote`: Evaluates and returns sorted, executable routes for cross-chain transfers.
- `POST /api/v1/anchor/deposit`: Sets up deposit transactions (on-ramp) using the SEP-24 interactive flow.
- `POST /api/v1/anchor/withdraw`: Sets up withdrawal transactions (off-ramp) using the SEP-24 interactive flow.
- `POST /api/v1/anchor/quote`: Returns price and rate quotes following the SEP-38 standard.

## Technical Details

- **Language**: Rust (ensures memory safety and predictable execution times)
- **Runtime**: Tokio (handles concurrent async operations)
- **Web Framework**: Axum (manages HTTP routing and request parsing)
- **HTTP Client**: Reqwest (manages outbound calls to anchors and bridge builders)

## Running Locally

1. Verify that the Rust toolchain is installed.
2. Change directory to the engine path.
3. Start the application:
   ```bash
   cargo run
   ```
4. The service will be active at `http://127.0.0.1:8080`.
