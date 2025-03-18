use crate::utils::url::url_decode;
use crate::{models::HTTP_DEFAULT_GROUP, Proxy};
use url::Url;

/// Parse an HTTP subscription link into a Proxy object
/// Matches C++ explodeHTTPSub implementation
pub fn explode_http_sub(link: &str, node: &mut Proxy) -> bool {
    // Parse the URL
    let url = match Url::parse(link) {
        Ok(u) => u,
        Err(_) => return false,
    };

    // Determine if TLS is enabled
    let is_https = url.scheme() == "https";

    // Initialize variables
    let mut group = String::new();
    let mut remarks = String::new();
    let mut server = String::new();
    let mut port = String::new();
    let mut username = String::new();
    let mut password = String::new();

    // Extract query parameters
    for (key, value) in url.query_pairs() {
        match key.as_ref() {
            "remarks" => remarks = url_decode(&value),
            "group" => group = url_decode(&value),
            _ => {}
        }
    }

    // Extract username and password
    username = url.username().to_string();
    if let Some(pass) = url.password() {
        password = pass.to_string();
    }

    // Extract hostname and port
    if let Some(host) = url.host_str() {
        server = host.to_string();
    } else {
        return false;
    }

    if let Some(p) = url.port() {
        port = p.to_string();
    } else {
        port = if is_https {
            "443".to_string()
        } else {
            "80".to_string()
        };
    }

    // Use default group if none specified
    if group.is_empty() {
        group = HTTP_DEFAULT_GROUP.to_string();
    }

    // Use server:port as remark if none specified
    if remarks.is_empty() {
        remarks = format!("{}:{}", server, port);
    }

    // Skip invalid port
    if port == "0" {
        return false;
    }

    // Parse port to u16
    let port_num = match port.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Create the proxy object
    *node = Proxy::http_construct(
        &group, &remarks, &server, port_num, &username, &password, is_https, None, None, None, "",
    );

    true
}
