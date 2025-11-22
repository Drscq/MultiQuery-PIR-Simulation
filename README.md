# MultiQuery-PIR-Simulation

A simulation framework for Multi-Query Private Information Retrieval (MQPIR) implemented in Rust with Python automation scripts for running experiments.

## Overview

This repository contains:
- **MQPIR**: A Rust implementation of a Multi-Query Private Information Retrieval system with client-server architecture
- **Python Scripts**: Automation tools for running experiments and processing results

## Repository Structure

```
.
├── MQPIR/                          # Rust implementation of MQPIR
│   ├── src/
│   │   ├── main.rs                 # Entry point with client/server modes
│   │   ├── client.rs               # Client preprocessing logic
│   │   └── serverOffline.rs        # Server offline phase handler
│   ├── Cargo.toml                  # Rust dependencies
│   └── config.toml                 # Configuration for IP, ports, and commands
├── AtuoRunCSExperiments.py         # Automated client-server experiment runner
├── extract_necessary_information.py # Extract timing data from experiment logs
├── amortized_time_calculator.py    # Calculate amortized timing metrics
└── README.md                       # This file
```

## MQPIR Rust Implementation

The MQPIR system implements a client-server architecture for Private Information Retrieval with support for offline preprocessing.

### Features
- Client preprocessing with random hint generation
- Server offline phase for hint set processing
- TCP-based communication between client and server
- Configurable parameters (hints, block size, key size)

### Components

#### Client (`client.rs`)
- Generates random dummy keys for preprocessing
- Connects to server and sends hint sets
- Implements offline preprocessing phase

#### Server (`serverOffline.rs`)
- TCP listener on configurable port (default: 8080)
- Handles incoming client connections
- Processes hint sets in offline phase
- Multi-threaded client handling

#### Configuration (`config.toml`)
```toml
[ip_address]
address = "127.0.0.1"

[port]
value = "8080"

[cmd_offline_send_hints_set]
value = "offline_send_hints_set"

[cmd_offline_send_parities_set]
value = "offline_send_parities_set"

[cmd_receive_success]
value = "receive_success"
```

## Installation

### Prerequisites
- Rust (edition 2021 or later)
- Cargo
- Python 3.x (for automation scripts)

### Rust Dependencies
- `rand = "0.8.5"` - Random number generation
- `toml = "0.5.10"` - Configuration file parsing

### Setup

1. Clone the repository:
```bash
git clone https://github.com/Drscq/MultiQuery-PIR-Simulation.git
cd MultiQuery-PIR-Simulation
```

2. Build the Rust project:
```bash
cd MQPIR
cargo build --release
```

3. Install Python dependencies (for automation scripts):
```bash
pip install paramiko  # For SSH client-server experiments
```

## Usage

### Running MQPIR Locally

#### Start the Server (Offline Phase)
```bash
cd MQPIR
cargo run -- serverOffline
```

#### Run the Client (Preprocessing)
In a separate terminal:
```bash
cd MQPIR
cargo run -- client
```

### Python Automation Scripts

#### AtuoRunCSExperiments.py
Automated script for running client-server experiments with varying database configurations:
- Configurable database entry sizes: 4KB, 16KB, 64KB, 256KB
- Automatically adjusts database size parameters
- Supports remote server execution via SSH
- Sends email notifications with results

**Note**: Requires configuration of SSH credentials and email settings in the script.

#### extract_necessary_information.py
Extracts timing metrics from experiment output files:
- End-to-end amortized time
- Online phase amortized time
- Setup phase amortized time

Usage:
```bash
python extract_necessary_information.py
# Input: ./output.txt
# Output: ./extracted_data_updated.txt
```

#### amortized_time_calculator.py
Calculates and updates setup phase amortized times based on end-to-end and online phase measurements.

Usage:
```bash
python amortized_time_calculator.py
# Input: ./extracted_data_updated.txt
# Output: ./extracted_data_final.txt
```

## Parameters

### MQPIR Parameters (in `main.rs`)
- `num_of_hints`: Number of hints to generate (default: 10)
- `block_size`: Size of each block in bytes (default: 32)
- `key_size`: Size of cryptographic key in bytes (default: 16)

### Experiment Parameters (in `AtuoRunCSExperiments.py`)
Database entry sizes and powers:
- 4KB: 2^14 to 2^24 entries (step 2)
- 16KB: 2^12 to 2^22 entries (step 2)
- 64KB: 2^10 to 2^20 entries (step 2)
- 256KB: 2^8 to 2^18 entries (step 2)

## Development

### Building
```bash
cd MQPIR
cargo build
```

### Testing
```bash
cd MQPIR
cargo test
```

### Checking Code
```bash
cd MQPIR
cargo check
```

## Network Communication

The system uses TCP sockets for client-server communication:
- Default IP: `127.0.0.1`
- Default Port: `8080`
- Protocol: Command-based with acknowledgments

Communication flow:
1. Client connects to server
2. Client sends command (`offline_send_hints_set`)
3. Server acknowledges receipt
4. Client sends hint data
5. Server processes hints offline

## License

Please refer to the repository license file for terms and conditions.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.