use crate::db::{get_shortened_url_by_codes, insert_shortened_url};
use axum::extract::Json;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

pub async fn shorten_handler(Json(request): Json<ShortenRequest>) -> String {
    let short_code = generate_short_code(&request.url).await;

    // Insert the URL into the database
    if let Err(_) = insert_shortened_url(&request.url, &short_code).await {
        // If insertion fails, return error code
        return "ERROR".to_string();
    }

    short_code
}

async fn generate_short_code(url: &str) -> String {
    // Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    // Create 10 slices of size 10 from the hash
    let mut candidates = Vec::new();
    for i in 0..10 {
        let start = i * 6; // Use 6 characters to get enough variety
        let end = (start + 10).min(hash_hex.len());
        if end - start >= 10 {
            candidates.push(hash_hex[start..start + 10].to_string());
        }
    }

    // If we don't have enough from the hash, pad with additional slices
    while candidates.len() < 10 {
        let base = candidates.len();
        let start = (base * 3) % hash_hex.len();
        let end = (start + 10).min(hash_hex.len());
        if end - start >= 10 {
            candidates.push(hash_hex[start..start + 10].to_string());
        } else {
            // Create a new hash to get more material
            let mut new_hasher = Sha256::new();
            new_hasher.update(format!("{}{}", url, base).as_bytes());
            let new_result = new_hasher.finalize();
            let new_hash = format!("{:x}", new_result);
            candidates.push(new_hash[0..10].to_string());
        }
    }

    // Convert to string refs for the database query
    let candidate_refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();

    // Query database for existing codes
    let existing_codes = get_shortened_url_by_codes(candidate_refs).await;
    let existing_set: std::collections::HashSet<String> = existing_codes
        .iter()
        .map(|url| url.short_code.clone())
        .collect();

    // Find first non-existing code
    for candidate in candidates {
        if !existing_set.contains(&candidate) {
            return candidate;
        }
    }

    // Fallback - should rarely happen
    format!("{:10}", hash_hex)[0..10].to_string()
}
