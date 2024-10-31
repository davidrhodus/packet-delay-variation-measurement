# Packet Delay Variation (PDV) Measurement Tool

This project contains a Rust-based client-server application to measure Packet Delay Variation (PDV) between two servers over a network. The client fetches highly accurate timestamps from an NTP server and sends them to the server, which calculates the PDV based on packet arrival intervals. PDV, or jitter, measures the variability in packet arrival times and is essential in evaluating network performance for real-time applications like video streaming, gaming, and VoIP.

## Overview

The tool uses NTP to synchronize timestamps between the client and server for one-way delay (OWD) and PDV measurement. Using a private UDP connection, it calculates jitter as the variance in OWD across consecutive packets.

### Key Features
- **NTP Synchronized Timestamps**: Fetches precise timestamps from an NTP server, achieving higher accuracy in delay measurements.
- **One-Way Delay (OWD) Calculation**: Measures OWD by calculating the difference between the timestamp and arrival time.
- **Real-Time PDV Calculation**: Calculates PDV based on the variability of consecutive delays.

## Usage

### Running the Server

On the server machine:

Run the server program to start listening for incoming timestamped packets:

```bash
cargo run --bin server
```

### Running the Client

On the client machine:

Update `client.rs` with the server’s IP address:

```rust
let server_addr = "SERVER_IP:8080"; // Replace with actual server IP
```

```bash
cargo run --bin client
```

The server will calculate and log the PDV for each packet received.

The server will calculate and log the PDV for each packet received.

## Technical Details

- **NTP Protocol**: The client uses NTP to obtain accurate timestamps, ensuring synchronized time without relying on local clocks. This is critical for calculating OWD accurately.
- **One-Way Delay (OWD)**: By measuring the difference between the client's transmitted timestamp and the server’s reception time, we calculate the OWD.
- **PDV Calculation**: PDV is computed as the difference between consecutive OWD values, showing variability in delay (i.e., jitter).

## Formulas and Calculations

### 1. NTP to Unix Time Conversion

NTP time starts from January 1, 1900, while Unix time starts from January 1, 1970. The conversion from NTP to Unix time in seconds is:

`Unix Time = NTP Seconds - 2,208,988,800`

Where:

- `2,208,988,800` is the number of seconds between the NTP and Unix epochs.

### 2. One-Way Delay (OWD) Calculation

Once the server receives the packet, it calculates OWD by subtracting the NTP timestamp sent by the client from the time it received the packet:

`OWD = Arrival Time_server - Timestamp_client`

Where:

- `Arrival Time_server` is the server’s time upon packet reception.
- `Timestamp_client` is the timestamp from the NTP server.

### 3. Packet Delay Variation (PDV)

PDV is calculated as the difference between consecutive OWD values. This difference quantifies the jitter over the network path:

`PDV = |OWD_i+1 - OWD_i|`

Where:

- `OWD_i+1` and `OWD_i` are consecutive OWD measurements.

### 4. Exponential Moving Average (EMA) for PDV Smoothing

To smooth the PDV over time, we apply an Exponential Moving Average (EMA), which dampens short-term fluctuations and provides a stable trend line:

`EMA_(n+1) = α × PDV_n + (1 - α) × EMA_n`

Where:

- `α` is the smoothing factor, usually between 0.1 and 0.3 for stability.

## How It Works

1. **NTP Timestamp Request**: The client sends a manually constructed NTP request to an NTP server (e.g., time.google.com) and retrieves the response containing the timestamp.
2. **Timestamp Transmission**: The client then sends this NTP timestamp to the server.
3. **OWD Calculation**: The server records the arrival time and calculates OWD based on the timestamp.
4. **PDV Calculation**: By comparing consecutive OWD values, the server calculates the PDV, logging jitter for analysis.
