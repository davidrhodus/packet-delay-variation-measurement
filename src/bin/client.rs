use std::net::UdpSocket;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    let server_addr = "SERVER_IP:8080"; // Replace with your server's IP and port
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available local port
    socket.connect(server_addr)?;

    // NTP server address and port (standard NTP port is 123)
    let ntp_server = "time.google.com:123";
    let ntp_socket = UdpSocket::bind("0.0.0.0:0")?;
    ntp_socket.connect(ntp_server)?;

    // NTP packet with a single byte set to 0x1B (indicating a client request)
    let mut ntp_packet = [0; 48];
    ntp_packet[0] = 0x1B;

    loop {
        // Send NTP request
        ntp_socket.send(&ntp_packet)?;

        // Receive the NTP response
        let mut buffer = [0; 48];
        ntp_socket.recv(&mut buffer)?;

        // Extract the timestamp from the response (bytes 40-43 for seconds, 44-47 for fraction)
        let seconds = u32::from_be_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]) as u64;
        let fraction = u32::from_be_bytes([buffer[44], buffer[45], buffer[46], buffer[47]]) as u64;

        // Convert NTP time to Unix time (NTP epoch starts in 1900, Unix in 1970)
        let ntp_unix_difference = 2_208_988_800; // seconds between 1900 and 1970
        let timestamp = (seconds - ntp_unix_difference) * 1_000_000_000 + (fraction * 1_000_000_000 / (1 << 32));

        // Convert to bytes and send to server
        let time_bytes = timestamp.to_le_bytes();
        socket.send(&time_bytes)?;

        // Wait for the next interval
        sleep(Duration::from_millis(100)); // Adjust interval as needed
    }
}

