# Rust Port Scanner

A fast port scanning tool written in Rust with multi-threaded concurrent scanning.

## Features

- TCP Connect scanning
- Multi-threaded concurrent scanning
- Output of open ports list
- Configurable port ranges
- Service detection for common ports
- Progress indication

## Installation

```bash
cargo build --release
```

The binary will be in `target/release/port_scanner`

## Usage

### Basic scan (common ports on localhost)

```bash
port_scanner 127.0.0.1
```

### Scan specific port range

```bash
port_scanner 192.168.1.1 --ports 1-1000
```

### Scan specific ports

```bash
port_scanner 192.168.1.1 --ports 22,80,443,8080
```

### Scan with multiple threads

```bash
port_scanner 10.0.0.1 --ports 1-65535 --threads 100
```

### Show service names

```bash
port_scanner 127.0.0.1 --show-services
```

### Verbose output

```bash
port_scanner 192.168.1.1 -v
```

### Export results to JSON

```bash
port_scanner 192.168.1.1 --json > results.json
```

## Options

- `target`: Target IP address or hostname (required)
- `--ports, -p`: Port range or specific ports [default: common ports (1-1024)]
- `--threads, -t`: Number of concurrent threads [default: 50]
- `--timeout, -o`: Connection timeout in milliseconds [default: 1000]
- `--show-services, -s`: Show service names for known ports
- `--json, -j`: Output in JSON format
- `--verbose, -v`: Verbose output
- `--help, -h`: Show help message

## Port Ranges

Use the following formats:
- Range: `1-1000`
- Specific: `22,80,443`
- Mixed: `22,80,443,1000-2000`

## Common Ports Reference

| Port | Service |
|------|---------|
| 21   | FTP |
| 22   | SSH |
| 23   | Telnet |
| 25   | SMTP |
| 53   | DNS |
| 80   | HTTP |
| 110  | POP3 |
| 143  | IMAP |
| 443  | HTTPS |
| 3306 | MySQL |
| 5432 | PostgreSQL |
| 6379 | Redis |
| 8080 | HTTP Proxy |

## Examples

### Quick scan of top ports

```bash
port_scanner 10.0.0.1 --ports top-100
```

### Full port scan (slow but thorough)

```bash
port_scanner 10.0.0.1 --ports 1-65535 --threads 200
```

### Scan localhost

```bash
port_scanner localhost --show-services
```

## Performance

The scanner uses Tokio for async I/O and can handle thousands of concurrent connections efficiently.

Typical scan times:
- Top 100 ports: < 5 seconds
- Port range 1-1000: < 15 seconds
- Full scan (1-65535): varies by network conditions
