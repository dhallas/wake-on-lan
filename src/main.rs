use clap::Parser;
use std::net::UdpSocket;
use std::process;

/// Program to send Wake-on-LAN packets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The MAC address of the device to wake up
    #[arg(short, long, value_parser = validate_mac)]
    mac: String,

    /// The broadcast address to send the packet to
    #[arg(short, long, default_value = "255.255.255.255")]
    address: String,

    /// The UDP port to send the packet to
    #[arg(short, long, default_value_t = 9)]
    port: u16,
}

fn validate_mac(mac: &str) -> Result<String, String> {
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() != 6 {
        return Err(String::from("Invalid MAC address format"));
    }
    for part in parts {
        if part.len() != 2 || !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(String::from("Invalid MAC address format"));
        }
    }
    Ok(mac.to_owned())
}

fn build_magic_packet(mac: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    // First add 6 bytes of 0xFF
    packet.extend_from_slice(&[0xFF; 6]);
    // Second add the MAC address repeated 16 times
    let mac_bytes: Vec<u8> = mac
        .split(':')
        .map(|part| u8::from_str_radix(part, 16).unwrap())
        .collect();
    for _ in 0..16 {
        packet.extend_from_slice(&mac_bytes);
    }
    packet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mac_valid() {
        assert!(validate_mac("00:11:22:33:44:55").is_ok());
        assert!(validate_mac("b8:ae:ed:9c:c7:89").is_ok());
        assert!(validate_mac("ff:ff:ff:ff:ff:ff").is_ok());
        assert!(validate_mac("AA:BB:CC:DD:EE:FF").is_ok());
    }

    #[test]
    fn test_validate_mac_too_few_octets() {
        assert!(validate_mac("00:11:22:33:44").is_err());
    }

    #[test]
    fn test_validate_mac_too_many_octets() {
        assert!(validate_mac("00:11:22:33:44:55:66").is_err());
    }

    #[test]
    fn test_validate_mac_invalid_hex() {
        assert!(validate_mac("00:11:22:33:44:GG").is_err());
    }

    #[test]
    fn test_validate_mac_wrong_delimiter() {
        assert!(validate_mac("00-11-22-33-44-55").is_err());
    }

    #[test]
    fn test_validate_mac_empty() {
        assert!(validate_mac("").is_err());
    }

    #[test]
    fn test_validate_mac_single_digit_octet() {
        assert!(validate_mac("0:1:2:3:4:5").is_err());
    }

    #[test]
    fn test_build_magic_packet_length() {
        let packet = build_magic_packet("00:11:22:33:44:55");
        // 6 bytes of 0xFF + 16 * 6 bytes of MAC = 102 bytes
        assert_eq!(packet.len(), 102);
    }

    #[test]
    fn test_build_magic_packet_header() {
        let packet = build_magic_packet("00:11:22:33:44:55");
        assert_eq!(&packet[0..6], &[0xFF; 6]);
    }

    #[test]
    fn test_build_magic_packet_mac_repetitions() {
        let packet = build_magic_packet("b8:ae:ed:9c:c7:89");
        let expected_mac = [0xb8, 0xae, 0xed, 0x9c, 0xc7, 0x89];
        for i in 0..16 {
            let offset = 6 + i * 6;
            assert_eq!(&packet[offset..offset + 6], &expected_mac);
        }
    }
}

fn main() {
    let args = Args::parse();
    let magic_packet = build_magic_packet(&args.mac);
    let dest = format!("{}:{}", args.address, args.port);

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: failed to bind socket: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = socket.set_broadcast(true) {
        eprintln!("Error: failed to enable broadcast: {e}");
        process::exit(1);
    }

    if let Err(e) = socket.send_to(&magic_packet, &dest) {
        eprintln!("Error: failed to send packet to {dest}: {e}");
        process::exit(1);
    }

    println!("Wake up packet sent to {}", args.mac);
}
