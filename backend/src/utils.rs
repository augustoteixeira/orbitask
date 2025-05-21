use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use base64::{engine::general_purpose, Engine as _};
use std::fs;

type IP = String;

pub struct RateLimiter {
    attempts: Mutex<HashMap<IP, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            attempts: Mutex::new(HashMap::new()),
        }
    }

    pub fn too_many_attempts(
        &self,
        ip: &str,
        limit: usize,
        window: Duration,
    ) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        let now = Instant::now();
        let timestamps = attempts.entry(ip.to_string()).or_default();

        // Remove expired attempts
        timestamps.retain(|&t| now.duration_since(t) < window);

        // Record this attempt
        timestamps.push(now);

        // Return whether we're over the limit
        timestamps.len() > limit
    }
}

pub fn is_password_valid(pw: &str) -> bool {
    !pw.is_empty() && pw.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn load_or_generate_secret() -> String {
    const KEY_FILE: &str = "data/secret.key";

    match fs::read_to_string(KEY_FILE) {
        Ok(s) => s.trim().to_owned(),
        Err(_) => {
            println!("Generating secret key...");
            let key: [u8; 32] = rand::random();
            let key_encoded = general_purpose::STANDARD.encode(key);
            fs::write(KEY_FILE, &key_encoded)
                .expect("Failed to write secret key");
            key_encoded
        }
    }
}
