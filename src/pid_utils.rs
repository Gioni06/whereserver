use std::str;
use tokio::task;

pub async fn get_pid(port: u16) -> Option<String> {
    let output = task::spawn_blocking(move || {
        match std::process::Command::new("lsof")
            .arg("-i")
            .arg(format!(":{}", port))
            .output()
        {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    })
    .await
    .unwrap_or(None);

    match output {
        Some(output_data) => {
            let output_str = std::str::from_utf8(&output_data.stdout).unwrap_or("");
            for line in output_str.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    return Some(parts[1].to_string());
                }
            }
            None
        }
        None => None,
    }
}
