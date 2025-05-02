use crate::defs::FIELD_LEN_LIMIT;

/// Check if a field is valid (not empty and within length limit)
#[inline]
pub fn is_valid_field(s: &str) -> bool {
    !s.is_empty() && s.len() < FIELD_LEN_LIMIT
}

/// Check if a key field is valid (not empty, within length limit, and doesn't contain colons)
#[inline]
pub fn is_valid_key_field(s: &str) -> bool {
    !s.is_empty() && s.len() < FIELD_LEN_LIMIT && !s.contains(':')
}

/// Get the current timestamp in seconds
#[inline]
pub fn now_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
