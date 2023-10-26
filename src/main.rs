use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task;

mod deduplicate;
mod http_utils;
mod pid_utils;

#[tokio::main]
async fn main() {
    let common_ports = vec![80, 443, 8000];
    let vite_ports: Vec<u16> = (5000..=5499).collect();
    let webpack_ports: Vec<u16> = (8080..=8999).collect();
    let create_react_app_ports: Vec<u16> = (9000..=9999).collect();
    let svelte_ports: Vec<u16> = (5500..=5999).collect();
    let system_ports: Vec<u16> = (1024..=49151).collect();

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

    let stdout_mutex = Arc::new(Mutex::new(()));

    let handles: Vec<_> = all_ports
        .into_iter()
        .map(|port| {
            let stdout_mutex = Arc::clone(&stdout_mutex);
            task::spawn(async move {
                let address = format!("127.0.0.1:{}", port);
                let result = tokio::time::timeout(timeout, TcpStream::connect(&address)).await;

                if let Ok(Ok(_)) = result {
                    let protocol = if port == 443 { "https" } else { "http" };
                    let url = format!("{}://{}", protocol, address);

                    if http_utils::is_serving_content(&url).await {
                        let pid = pid_utils::get_pid(port)
                            .await
                            .unwrap_or_else(|| "N/A".to_string());
                        let clickable = format!("\x1B]8;;{}\x07{:38}\x1B]8;;\x07", url, url);

                        let _lock = stdout_mutex.lock().await;
                        println!("| {:4} | {:5} | {} |", port, pid, clickable.trim_end());
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        match handle.await {
            Ok(_) => (),
            Err(err) => eprintln!("A task panicked: {:?}", err),
        }
    }
}
