use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use serde::Serialize;


/// version of protocol
pub const JSONRPC_VERSION: &str = "2.0";

/// Request parameters
/// 
/// Enable sending of serializable parameters with request
/// 
/// # Examples
/// 
/// ```
/// let str_params = jsonrpc_v2_client::Params("hello");
/// let f32_params = jsonrpc_v2_client::Params(3.14);
/// let u32_params = jsonrpc_v2_client::Params(1024);
/// let str_list_params = jsonrpc_v2_client::Params(["hello", "world"]);
/// let u32_list_params = jsonrpc_v2_client::Params([120_000, 20_000]);
/// ```
#[derive(Debug, Serialize)]
pub struct Params<T: Serialize>(pub T);

/// API Key container
/// 
/// Store api key and output it in several formats: query string, http header, cookie
/// 
/// # Examples
/// 
/// ```
/// let api_key = jsonrpc_v2_client::ApiKey::new("API-KEY", "abcdef12345");
/// println!("{}", api_key.as_header());
/// // API-KEY: abcdef12345
/// 
/// println!("{}", api_key.as_query_str());
/// // API-KEY=abcdef12345
/// ```
#[derive(Debug, Serialize)]
pub struct ApiKey(String, String);
impl ApiKey {
    pub fn new(key: &str, value: &str) -> ApiKey {
        ApiKey(key.to_string(), value.to_string())
    }

    pub fn as_query_str(&self) -> String {
        format!("{}={}", self.0, self.1)
    }

    pub fn as_header(&self) -> String {
        format!("{}: {}", self.0, self.1)
    }

    pub fn as_cookie(&self) -> String {
        format!("Cookie: {}={}", self.0, self.1)
    }
}

/// JSON RPC Request
/// 
/// Contains request data and is able to send a remote call
/// 
/// # Examples
/// 
/// ``` no_run
/// let math_service_url = "http://localhost:8082/math-api";
/// let method = "add";
/// let params = jsonrpc_v2_client::Params([10.5, 20.5]);
/// let id = 0;
/// let request = jsonrpc_v2_client::Request::new(method, params, id);
/// let response = request.send(math_service_url).unwrap();
/// println!("{}", response);
/// ```
#[derive(Debug, Serialize)]
pub struct Request<T: Serialize> {
    jsonrpc: String,
    pub method: String,
    pub params: Params<T>,
    pub id: u64,
}
impl<T: Serialize> Request<T> {
    pub fn new(method: &str, params: Params<T>, id: u64) -> Request<T> {
        Request {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.to_string(),
            params: params,
            id: id,
        }
    }
    pub fn send(self, url: &str) -> std::result::Result<std::string::String, std::io::Error> {
        task::block_on(async {
            let mut client = TcpStream::connect(url).await.unwrap();
            client.write_all(
                format!(
                    "POST / HTTP/1.1\r\nContent-Type: application/json\r\nUser-Agent: jsonrpc_v2_client\r\n\n{}",
                    serde_json::to_string(&self).unwrap()
                ).as_bytes()
            ).await?;

            let mut buffer = [0u8; 4 * 1024];
            let buffer_size = client.read(&mut buffer).await.unwrap();

            Ok(String::from_utf8_lossy(&buffer[..buffer_size]).into_owned())
        })   
    }
}
