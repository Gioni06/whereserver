use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Importing deduplication utilities.
mod deduplicate;
// Importing utilities to fetch PID of a port.
mod pid_utils;
// Importing utilities related to HTTP requests.
mod http_utils;

/// # Whereserver
/// Find running http servers on your machine
fn main() {
    // Commonly used ports for various applications and services.
    let common_ports = vec![80, 443, 8000];
    let vite_ports: Vec<u16> = (5000..=5499).collect();
    let webpack_ports: Vec<u16> = (8080..=8999).collect();
    let create_react_app_ports: Vec<u16> = (9000..=9999).collect();
    let svelte_ports: Vec<u16> = (5500..=5999).collect();
    let system_ports: Vec<u16> = (1024..=49151).collect();

    // Concatenating all port ranges and duplicating them.
    let all_ports = [
        system_ports,
        common_ports,
        vite_ports,
        webpack_ports,
        create_react_app_ports,
        svelte_ports,
    ]
    .concat();
    let all_ports: Vec<u16> = deduplicate::vec_u16(all_ports);

    let timeout = Duration::from_millis(100);

    println!("| Port | PID   | URL                                    |");
    println!("|------|-------|----------------------------------------|");

    let stdout_mutex = Arc::new(Mutex::new(())); // Mutex to lock stdout

    let handles: Vec<_> = all_ports
        .into_iter()
        .map(|port| {
            let stdout_mutex = Arc::clone(&stdout_mutex);
            thread::spawn(move || {
                let address = format!("127.0.0.1:{}", port);
                if let Ok(_) = TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
                    let protocol = if port == 443 { "https" } else { "http" };
                    let url = format!("{}://{}", protocol, address);

                    if http_utils::is_serving_content(&url) {
                        let pid = pid_utils::get_pid(port).unwrap_or_else(|| "N/A".to_string());
                        let clickable = format!("\x1B]8;;{}\x07{:38}\x1B]8;;\x07", url, url);

                        let _lock = stdout_mutex.lock().unwrap();
                        println!("| {:4} | {:5} | {} |", port, pid, clickable.trim_end());
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        match handle.join() {
            Ok(_) => (),
            Err(err) => eprintln!("A thread panicked: {:?}", err),
        }
    }
}
