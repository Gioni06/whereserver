use std::net::TcpStream;
use std::time::Duration;
use reqwest;
use std::process::Command;
use std::str;

fn get_pid(port: u16) -> Option<String> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}" , port))
        .output()
        .expect("Failed to execute lsof.");

    let output_str = str::from_utf8(&output.stdout).unwrap_or("");
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 1 {
            return Some(parts[1].to_string());
        }
    }
    None
}

fn is_serving_content(url: &str) -> bool {
    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status().is_success() {
                return true;
            }
        },
        Err(_) => {}
    }
    false
}

fn main() {
    let common_ports = vec![80, 443, 8000];
    let vite_ports: Vec<u16> = (5000..=5999).collect();
    let webpack_ports: Vec<u16> = (8080..=8999).collect();
    let create_react_app_ports: Vec<u16> = (9000..=9999).collect();
    let svelte_ports: Vec<u16> = (5000..=5999).collect();
    let all_ports = [common_ports, vite_ports, webpack_ports, create_react_app_ports, svelte_ports].concat();

    // deduplicate all_ports
    let all_ports: Vec<u16> = all_ports.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    
    let timeout = Duration::from_millis(100);

    println!("| Port | PID   | URL                                    |");
    println!("|------|-------|----------------------------------------|");

    for &port in &all_ports {
        let address = format!("127.0.0.1:{}", port);
        match TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
            Ok(_) => {
                let protocol = if port == 443 { "https" } else { "http" };
                let url = format!("{}://{}", protocol, address);

                if is_serving_content(&url) {
                    let pid = get_pid(port).unwrap_or_else(|| "N/A".to_string());
                    let clickable = format!("\x1B]8;;{}\x07{:38}\x1B]8;;\x07", url, url);
                    println!("| {:4} | {:5} | {} |", port, pid, clickable.trim_end());
                }
            }
            Err(_) => {
                // No webserver on this port
            }
        }
    }
}
