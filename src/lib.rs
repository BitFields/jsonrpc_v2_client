use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// JSONRPC version 2.0 compatible client library
/// [JSONRPC v2.0 specification](https://www.jsonrpc.org/specification)
pub const JSONRPC_VERSION: &str = "2.0";

/// Custom error type for JSON-RPC operations
#[derive(Debug)]
pub enum JsonRpcError {
    ConnectionError(String),
    SerializationError(String),
    ResponseError(String),
    InvalidResponse,
}

impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::ResponseError(msg) => write!(f, "Response error: {}", msg),
            Self::InvalidResponse => write!(f, "Invalid response format"),
        }
    }
}

impl Error for JsonRpcError {}

/// Request parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params<T: Serialize>(pub T);

/// API Key container
#[derive(Debug, Serialize)]
pub struct APIKey {
    key: String,
    value: String,
}

impl APIKey {
    pub fn new(key: &str, value: &str) -> Self {
        APIKey {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    pub fn as_header(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

/// Service address container
#[derive(Clone, Debug)]
pub struct ServiceAddress {
    pub url: String,
    pub endpoint: String,
}

impl ServiceAddress {
    pub fn new(url: &str, endpoint: &str) -> Self {
        ServiceAddress {
            url: url.trim_end_matches('/').to_owned(),
            endpoint: endpoint.trim_start_matches('/').to_owned(),
        }
    }

    pub fn full_path(&self) -> String {
        format!("{}/{}", self.url, self.endpoint)
    }
}

/// JSON RPC Request
#[derive(Clone, Debug, Serialize)]
pub struct Request<T: Serialize> {
    jsonrpc: String,
    pub method: String,
    pub params: Params<T>,
    pub id: String,
}

impl<T: Serialize> Request<T> {
    pub fn new(method: &str, params: Params<T>, id: &str) -> Self {
        Request {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            method: method.to_owned(),
            params,
            id: id.to_owned(),
        }
    }

    /// Sends the request asynchronously and returns a Result with the response
    pub async fn send_async(
        &self,
        service_address: &ServiceAddress,
        api_key: Option<&APIKey>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        let mut client = TcpStream::connect(&service_address.url)
            .await
            .map_err(|e| JsonRpcError::ConnectionError(e.to_string()))?;

        let json = serde_json::to_string(&self)
            .map_err(|e| JsonRpcError::SerializationError(e.to_string()))?;
        let content_length = json.len();

        let request = match api_key {
            Some(key) => format!(
                "POST /{} HTTP/1.1\r\n\
                Host: {}\r\n\
                Content-Type: application/json\r\n\
                User-Agent: jsonrpc_v2_client/0.1.0\r\n\
                Accept: application/json\r\n\
                {}\r\n\
                Content-Length: {}\r\n\r\n\
                {}",
                service_address.endpoint,
                service_address.url,
                key.as_header(),
                content_length,
                json
            ),
            None => format!(
                "POST /{} HTTP/1.1\r\n\
                Host: {}\r\n\
                Content-Type: application/json\r\n\
                User-Agent: jsonrpc_v2_client/0.1.0\r\n\
                Accept: application/json\r\n\
                Content-Length: {}\r\n\r\n\
                {}",
                service_address.endpoint,
                service_address.url,
                content_length,
                json
            ),
        };

        log::debug!("Sending request: {}", request);

        client
            .write_all(request.as_bytes())
            .await
            .map_err(|e| JsonRpcError::ConnectionError(e.to_string()))?;

        let mut buffer = Vec::new();
        client
            .read_to_end(&mut buffer)
            .await
            .map_err(|e| JsonRpcError::ResponseError(e.to_string()))?;

        let response_str = String::from_utf8_lossy(&buffer);
        let body = response_str
            .split("\r\n\r\n")
            .nth(1)
            .ok_or(JsonRpcError::InvalidResponse)?;

        serde_json::from_str(body).map_err(|e| JsonRpcError::SerializationError(e.to_string()))
    }

    /// Synchronous version of send
    pub fn send(
        &self,
        service_address: &ServiceAddress,
        api_key: Option<&APIKey>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        task::block_on(self.send_async(service_address, api_key))
    }
}

/// Configuration for the JSON-RPC client
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub timeout: Option<std::time::Duration>,
    pub max_buffer_size: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            timeout: Some(std::time::Duration::from_secs(30)),
            max_buffer_size: 4 * 1024 * 1024, // 4MB default
        }
    }
}
