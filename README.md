# wake-on-lan

A command-line tool to send Wake-on-LAN magic packets written in Rust.

## Usage

```sh
wake-on-lan --mac <MAC_ADDRESS>
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-m, --mac` | MAC address of the device to wake (required) | — |
| `-a, --address` | Broadcast address to send the packet to | `255.255.255.255` |
| `-p, --port` | UDP port to send the packet to | `9` |

### Examples

```sh
# Wake a device using the default broadcast address and port
wake-on-lan --mac b8:ae:ed:9c:c7:89

# Wake a device on a specific subnet
wake-on-lan --mac b8:ae:ed:9c:c7:89 --address 192.168.1.255

# Wake a device on a custom port
wake-on-lan --mac b8:ae:ed:9c:c7:89 --port 7
```

## Building

```sh
cargo build --release
```

## Running tests

```sh
cargo test
```
