use std::net::TcpStream;
use std::time::Duration;
mod deduplicate;
mod pid_utils;
mod http_utils;

fn main() {
    let common_ports = vec![80, 443, 8000];
    let vite_ports: Vec<u16> = (5000..=5499).collect();
    let webpack_ports: Vec<u16> = (8080..=8999).collect();
    let create_react_app_ports: Vec<u16> = (9000..=9999).collect();
    let svelte_ports: Vec<u16> = (5500..=5999).collect();
    let system_ports: Vec<u16> = (1024..=49151).collect();
    let all_ports = [system_ports, common_ports, vite_ports, webpack_ports, create_react_app_ports, svelte_ports].concat();

    // deduplicate all_ports
    let all_ports: Vec<u16> = deduplicate::vec_u16(all_ports);
    
    let timeout = Duration::from_millis(100);

    println!("| Port | PID   | URL                                    |");
    println!("|------|-------|----------------------------------------|");

    for &port in &all_ports {
        let address = format!("127.0.0.1:{}", port);
        if let Ok(_) = TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
            let protocol = if port == 443 { "https" } else { "http" };
            let url = format!("{}://{}", protocol, address);

            if http_utils::is_serving_content(&url) {
                let pid = pid_utils::get_pid(port).unwrap_or_else(|| "N/A".to_string());
                let clickable = format!("\x1B]8;;{}\x07{:38}\x1B]8;;\x07", url, url);
                println!("| {:4} | {:5} | {} |", port, pid, clickable.trim_end());
            }
        }
    }
}
