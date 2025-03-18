//! Network utilities for IP address handling and validation

/// Checks if a string is a valid IPv4 address
///
/// # Arguments
///
/// * `s` - The string to check
///
/// # Returns
///
/// True if the string is a valid IPv4 address, false otherwise
pub fn is_ipv4(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();

    if parts.len() != 4 {
        return false;
    }

    for part in parts {
        // Check if part is a valid number between 0-255
        match part.parse::<u8>() {
            Ok(_) => continue,
            Err(_) => return false,
        }
    }

    true
}

/// Checks if a string is a valid IPv6 address
///
/// # Arguments
///
/// * `s` - The string to check
///
/// # Returns
///
/// True if the string is a valid IPv6 address, false otherwise
pub fn is_ipv6(s: &str) -> bool {
    // Basic implementation - placeholder
    // In a real implementation, we'd do proper IPv6 validation
    s.contains(':')
}

/// Checks if a string is a valid URL
pub fn is_link(link: &str) -> bool {
    link.starts_with("http://")
        || link.starts_with("https://")
        || link.starts_with("data:")
        || link.starts_with("content://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ipv4_valid() {
        assert!(is_ipv4("192.168.1.1"));
        assert!(is_ipv4("127.0.0.1"));
        assert!(is_ipv4("8.8.8.8"));
        assert!(is_ipv4("255.255.255.255"));
    }

    #[test]
    fn test_is_ipv4_invalid() {
        assert!(!is_ipv4("192.168.1"));
        assert!(!is_ipv4("192.168.1.256"));
        assert!(!is_ipv4("192.168.1.a"));
        assert!(!is_ipv4("192.168.1.1.1"));
        assert!(!is_ipv4("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    }

    #[test]
    fn test_is_ipv6_valid() {
        assert!(is_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(is_ipv6("::1"));
        assert!(is_ipv6("2001:db8::"));
    }

    #[test]
    fn test_is_ipv6_invalid() {
        assert!(!is_ipv6("192.168.1.1"));
        assert!(!is_ipv6("not an ip"));
    }
}
