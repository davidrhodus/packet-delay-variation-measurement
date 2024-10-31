use std::net::UdpSocket;
use std::time::{Duration, SystemTime};
use std::convert::TryInto;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    println!("Server listening on port 8080");

    let mut buf = [0; 8];
    let mut last_arrival_time: Option<SystemTime> = None;
    let mut last_delay: Option<Duration> = None;

    loop {
        let (amt, _src) = socket.recv_from(&mut buf)?;
        if amt < 8 {
            eprintln!("Received incomplete data");
            continue;
        }

        // Decode the NTP timestamp from the client
        let timestamp_data = u64::from_le_bytes(buf.try_into().expect("Invalid data"));
        let ntp_seconds = (timestamp_data >> 32) as u64;
        let ntp_nanos = (timestamp_data & 0xFFFFFFFF) as u64;
        let client_time = Duration::new(ntp_seconds, ntp_nanos as u32);

        // Get the current server time
        let server_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time went backwards");

        // Calculate the one-way delay (OWD) by subtracting the client timestamp from the server's arrival time
        let delay = server_time.checked_sub(client_time)
            .unwrap_or(Duration::from_millis(0)); // Prevent negative durations

        // Calculate PDV as the difference between consecutive delays
        if let Some(last) = last_delay {
            let pdv = delay.checked_sub(last).unwrap_or(Duration::from_millis(0));
            println!("PDV: {:?}", pdv);
        }

        // Update the last values
        last_delay = Some(delay);
        last_arrival_time = Some(SystemTime::now());
    }
}
