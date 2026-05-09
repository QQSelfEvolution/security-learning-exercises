use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Local;
use clap::{ArgAction, Parser, ValueHint};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(name = "port_scanner")]
#[command(about = "Fast port scanner with multi-threaded scanning", long_about = None)]
struct Args {
    /// Target IP address or hostname
    #[arg(value_hint = ValueHint::Hostname)]
    target: String,

    /// Port range or specific ports (e.g., 1-1000, 22,80,443, or top-100)
    #[arg(short, long, default_value = "1-1024")]
    ports: String,

    /// Number of concurrent threads
    #[arg(short, long, default_value_t = 50)]
    threads: usize,

    /// Connection timeout in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    timeout: u64,

    /// Show service names for known ports
    #[arg(short, long, action = ArgAction::SetTrue)]
    show_services: bool,

    /// Output in JSON format
    #[arg(short, long, action = ArgAction::SetTrue)]
    json: bool,

    /// Verbose output
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ScanResult {
    target: String,
    port: u16,
    state: String,
    service: Option<String>,
    response_time_ms: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
struct ScanSummary {
    target: String,
    start_time: String,
    end_time: String,
    duration_ms: u64,
    total_ports_scanned: usize,
    open_ports: usize,
    closed_ports: usize,
    filtered_ports: usize,
    results: Vec<ScanResult>,
}

// Common port to service mapping
fn get_service_name(port: u16) -> Option<String> {
    let services: HashMap<u16, &str> = [
        (21, "FTP"),
        (22, "SSH"),
        (23, "Telnet"),
        (25, "SMTP"),
        (53, "DNS"),
        (80, "HTTP"),
        (110, "POP3"),
        (143, "IMAP"),
        (443, "HTTPS"),
        (465, "SMTPS"),
        (587, "SMTP-SUB"),
        (993, "IMAPS"),
        (995, "POP3S"),
        (3306, "MySQL"),
        (3389, "RDP"),
        (5432, "PostgreSQL"),
        (5900, "VNC"),
        (6379, "Redis"),
        (8080, "HTTP-ALT"),
        (8443, "HTTPS-ALT"),
        (27017, "MongoDB"),
    ]
    .into_iter()
    .collect();

    services.get(&port).map(|s| s.to_string())
}

// Parse port string into list of ports
fn parse_ports(port_str: &str) -> Vec<u16> {
    if port_str.to_lowercase() == "top-100" {
        return vec![
            7, 20, 21, 22, 23, 25, 37, 42, 43, 49, 53, 67, 68, 69, 70, 79, 80, 81, 88, 102,
            110, 113, 119, 123, 135, 137, 138, 139, 143, 161, 162, 177, 179, 194, 201, 264,
            318, 381, 383, 389, 411, 412, 443, 445, 464, 465, 497, 500, 512, 513, 514, 515,
            520, 521, 540, 548, 554, 587, 591, 593, 631, 636, 639, 646, 691, 860, 873, 902,
            989, 990, 993, 995, 1025, 1026, 1027, 1028, 1029, 1080, 1110, 1433, 1434, 1723,
            1755, 1900, 2000, 2001, 2049, 2121, 2717, 3000, 3128, 3306, 3389, 3690, 4000,
            4333, 4443, 4444, 5000, 5060, 5190, 5432, 5631, 5632, 5800, 5900, 6000, 6001,
            6112, 6667, 6697, 7000, 7070, 7100, 8000, 8008, 8009, 8080, 8081, 8443, 8888,
            9090, 9100, 9418, 10000, 17000, 27017,
        ];
    }

    let mut ports = Vec::new();
    let parts: Vec<&str> = port_str.split(',').collect();

    for part in parts {
        let part = part.trim();
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                if let (Ok(start), Ok(end)) = (range[0].parse::<u16>(), range[1].parse::<u16>()) {
                    for port in start..=end {
                        ports.push(port);
                    }
                }
            }
        } else if let Ok(port) = part.parse::<u16>() {
            ports.push(port);
        }
    }

    ports.sort();
    ports.dedup();
    ports
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║           Rust Port Scanner v1.0                       ║");
    println!("╠════════════════════════════════════════════════════════╣");
    println!("║ Target:  {}",
        format!("{:<47}", &args.target));
    println!("║ Ports:   {}",
        format!("{:<47}", &args.ports));
    println!("║ Threads: {}",
        format!("{:<47}", args.threads));
    println!("║ Timeout: {}ms",
        format!("{:<46}", args.timeout));
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    let ports = parse_ports(&args.ports);
    let total_ports = ports.len();
    let target = args.target.clone();

    println!("Scanning {} ports...", total_ports);
    println!();

    let start_time = Instant::now();
    let start_timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Shared state for results
    let results: Arc<Mutex<Vec<ScanResult>>> = Arc::new(Mutex::new(Vec::new()));
    let semaphore = Arc::new(Semaphore::new(args.threads));
    let timeout_duration = Duration::from_millis(args.timeout);

    // Create tasks for each port
    let mut handles = Vec::new();

    for port in ports {
        let target = target.clone();
        let results = Arc::clone(&results);
        let sem = Arc::clone(&semaphore);
        let verbose = args.verbose;

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            let addr: SocketAddr = format!("{}:{}", target, port)
                .parse()
                .unwrap_or_else(|_| {
                    // If target is hostname, use 127.0.0.1 as fallback
                    format!("127.0.0.1:{}", port).parse().unwrap()
                });

            let port_start = Instant::now();
            let state: String;
            let response_time: Option<u64>;

            // Try to connect with timeout
            match timeout(timeout_duration, TcpStream::connect(&addr)).await {
                Ok(Ok(_stream)) => {
                    response_time = Some(port_start.elapsed().as_millis() as u64);
                    state = "open".to_string();
                }
                Ok(Err(_)) => {
                    response_time = None;
                    state = "closed".to_string();
                }
                Err(_) => {
                    response_time = None;
                    state = "filtered".to_string();
                }
            }

            let service = if args.show_services || verbose {
                get_service_name(port)
            } else {
                None
            };

            if verbose || state == "open" {
                let service_str = service
                    .as_ref()
                    .map(|s| format!(" ({})", s))
                    .unwrap_or_default();
                println!(
                    "[{}] Port {} is {}{}",
                    state.to_uppercase(),
                    port,
                    state,
                    service_str
                );
            }

            let result = ScanResult {
                target: target.clone(),
                port,
                state,
                service,
                response_time_ms: response_time,
            };

            let mut res = results.lock().await;
            res.push(result);
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }

    let duration = start_time.elapsed();
    let end_timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Collect results
    let all_results: Vec<ScanResult> = results.lock().await.clone();
    let open_count = all_results.iter().filter(|r| r.state == "open").count();
    let closed_count = all_results.iter().filter(|r| r.state == "closed").count();
    let filtered_count = all_results.iter().filter(|r| r.state == "filtered").count();

    // Output results
    if args.json {
        let summary = ScanSummary {
            target: args.target,
            start_time: start_timestamp,
            end_time: end_timestamp,
            duration_ms: duration.as_millis() as u64,
            total_ports_scanned: total_ports,
            open_ports: open_count,
            closed_ports: closed_count,
            filtered_ports: filtered_count,
            results: all_results,
        };

        let json = serde_json::to_string_pretty(&summary)?;
        println!("{}", json);
    } else {
        println!();
        println!("════════════════════════════════════════════════════════");
        println!("                      SCAN SUMMARY                         ");
        println!("════════════════════════════════════════════════════════");
        println!("  Target:       {}", args.target);
        println!("  Duration:     {:.2}s", duration.as_secs_f64());
        println!("  Ports scanned: {}", total_ports);
        println!();
        println!("  ┌──────────────┬─────────┐");
        println!("  │ State        │  Count  │");
        println!("  ├──────────────┼─────────┤");
        println!("  │ Open         │ {:>7} │", open_count);
        println!("  │ Closed       │ {:>7} │", closed_count);
        println!("  │ Filtered     │ {:>7} │", filtered_count);
        println!("  └──────────────┴─────────┘");
        println!();

        if open_count > 0 {
            println!("  Open Ports:");
            println!("  ┌──────┬──────────────────┬───────────┐");
            println!("  │ Port │ Service          │ Response  │");
            println!("  ├──────┼──────────────────┼───────────┤");
            for result in &all_results {
                if result.state == "open" {
                    let service = result.service.as_deref().unwrap_or("-");
                    let response = result
                        .response_time_ms
                        .map(|ms| format!("{}ms", ms))
                        .unwrap_or_else(|| "-".to_string());
                    println!(
                        "  │ {:>4} │ {:<16} │ {:>9} │",
                        result.port, service, response
                    );
                }
            }
            println!("  └──────┴──────────────────┴───────────┘");
        }
    }

    Ok(())
}
