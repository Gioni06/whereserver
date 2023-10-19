use std::process::Command;
use std::str;

pub fn get_pid(port: u16) -> Option<String> {
    let output = match Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", port))
        .output()
    {
        Ok(res) => res,
        Err(_) => return None,
    };

    let output_str = str::from_utf8(&output.stdout).unwrap_or("");
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 1 {
            return Some(parts[1].to_string());
        }
    }
    None
}
