use std::net::TcpStream;
use std::time::Duration;

// Importing deduplication utilities.
mod deduplicate;
// Importing utilities to fetch PID of a port.
mod pid_utils;
// Importing utilities related to HTTP requests.
mod http_utils;

/// # Whereserver
/// Whereserver is a command line utility to find the PID and URL of a development server that's running on your machine.
/// I wrote this utility because I often forget which port a development server is running on. Also sometimes a server is 
/// running in some background process, and I have to manually find the PID of the process and kill it.
///
/// This program does the following:
/// - Defines commonly used ports for various development servers and systems.
/// - Deduplicates the list of all these ports.
/// - For each port in the list:
///   - Checks if it's active using a TCP stream.
///   - If active, checks if it's serving HTTP content.
///   - If serving content, fetches its PID and prints a table entry with port, PID, and a clickable URL.
fn main() {
    // Commonly used ports for various applications and services.
    let common_ports = vec![80, 443, 8000];
    let vite_ports: Vec<u16> = (5000..=5499).collect();
    let webpack_ports: Vec<u16> = (8080..=8999).collect();
    let create_react_app_ports: Vec<u16> = (9000..=9999).collect();
    let svelte_ports: Vec<u16> = (5500..=5999).collect();
    let system_ports: Vec<u16> = (1024..=49151).collect();

    // Concatenating all port ranges and deduplicating them.
    let all_ports = [system_ports, common_ports, vite_ports, webpack_ports, create_react_app_ports, svelte_ports].concat();
    let all_ports: Vec<u16> = deduplicate::vec_u16(all_ports);
    
    let timeout = Duration::from_millis(100);

    println!("| Port | PID   | URL                                    |");
    println!("|------|-------|----------------------------------------|");

    // Scanning each port in the list.
    for &port in &all_ports {
        let address = format!("127.0.0.1:{}", port);
        if let Ok(_) = TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
            let protocol = if port == 443 { "https" } else { "http" };
            let url = format!("{}://{}", protocol, address);

            // Checking if the port is serving HTTP content.
            if http_utils::is_serving_content(&url) {
                // Fetching the PID associated with the port.
                let pid = pid_utils::get_pid(port).unwrap_or_else(|| "N/A".to_string());
                let clickable = format!("\x1B]8;;{}\x07{:38}\x1B]8;;\x07", url, url);
                println!("| {:4} | {:5} | {} |", port, pid, clickable.trim_end());
            }
        }
    }
}
