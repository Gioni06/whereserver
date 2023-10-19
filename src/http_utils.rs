use reqwest;
use std::str;

pub fn is_serving_content(url: &str) -> bool {
    match reqwest::blocking::get(url) {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}
