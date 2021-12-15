use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use serde::Serialize;

/// JSONRPC version 2.0 compatible client library
/// [JSONRPC v2.0 specification][1]
/// 
/// [1]: https://www.jsonrpc.org/specification

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
#[derive(Clone, Debug, Serialize)]
pub struct Params<T: Serialize>(pub T);

/// API Key container
///
/// # Examples
///
/// ```
/// let api_key = jsonrpc_v2_client::APIKey::new("API-KEY", "abcdef12345");
/// println!("{}", api_key.as_header());
/// // API-KEY: abcdef12345
/// ```
#[derive(Debug, Serialize)]
pub struct APIKey(String, String);

impl APIKey {

    pub fn new(key: &str, value: &str) -> APIKey {
        APIKey(key.to_string(), value.to_string())
    }

    pub fn as_header(&self) -> String {
        format!("{}: {}", self.0, self.1)
    }

}

/// Service address container containing `url` and `endpoint`
/// `url` is server host in the form of 127.0.0.1:8080
/// `endpoint` is service endpoint route like /api
/// So to access service at http://127.0.0.1:8082/api ServiceAddress
/// needs to be ServiceAddress::new("127.0.0.1:8082", "/api")
///
/// # Examples
/// 
/// ```
/// let math_service_address = jsonrpc_v2_client::ServiceAddress::new("127.0.0.1:8082", "/api");
/// println!("{:?}", math_service_address);
/// ```
/// 
#[derive(Clone, Debug)]
pub struct ServiceAddress {
    pub url: String,
    pub endpoint: String,
}

impl ServiceAddress {

    pub fn new(url: &str, endpoint: &str) -> ServiceAddress {

        ServiceAddress {
            url: url.to_string(),
            endpoint: endpoint.to_string(),
        }

    }

}

/// JSON RPC Request
///
/// Request object
///
/// # Examples
///
/// ``` no_run
/// let service_address = jsonrpc_v2_client::ServiceAddress::new("127.0.0.1:8082", "/api");
/// let method = "add";
/// let params = jsonrpc_v2_client::Params([10.5, 20.5]);
/// let id = 0;
/// let request = jsonrpc_v2_client::Request::new(method, params, id);
/// let response = request.send(&service_address, None);
/// println!("{}", response);
/// // Or with API KEY
/// let api_key = jsonrpc_v2_client::APIKey::new("API-KEY", "abcdef123456");
/// let response = request.send(&service_address, Some(&api_key));
/// println!("{}", response);
/// // Access JSON field
/// println!("{}", response["result"]);
/// println!("{}", response["error"]);
/// println!("{}", response["id"]);
/// ```
#[derive(Clone, Debug, Serialize)]
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

    pub fn send(
        &self,
        service_address: &ServiceAddress,
        api_key: Option<&APIKey>,
    ) -> serde_json::Value {

        task::block_on(async {
            let mut client = TcpStream::connect(&service_address.url).await.unwrap();
            let request: String;

            let json = serde_json::to_string_pretty(&self).unwrap();
            let content_length = json.len();

            let mut buffer = [0u8; 4 * 1024];
            let mut buffer_size: usize = 0;

            match api_key {

                Some(key_value) => {

                    request = format!(
                        "POST {} HTTP/1.1\r\n\
                        Content-Type: application/json\r\n\
                        User-Agent: jsonrpc_v2_client\r\n\
                        Accept: application/json\r\n\
                        {}\r\n\
                        Content-Length: {}\r\n\r\n\
                        {}",
                        service_address.endpoint,
                        key_value.as_header(),
                        content_length,
                        json,
                    );

                    log::trace!(
                        target: "jsonrpc_v2_client",
                        "[jsonrpc_v2_client: request as string]\r\n{}",
                        &request
                    );
                },
                None => {
                    request = format!(
                        "POST {} HTTP/1.1\r\n\
                        Content-Type: application/json\r\n\
                        User-Agent: jsonrpc_v2_client\r\n\
                        Accept: application/json\r\n\
                        Content-Length: {}\r\n\r\n\
                        {}",
                        service_address.endpoint,
                        content_length,
                        json
                    );

                    log::trace!(
                        target: "jsonrpc_v2_client",
                        "[jsonrpc_v2_client: request as string]\r\n{}",
                        &request
                    );
                }
            }

            log::trace!(
                target: "jsonrpc_v2_client",
                "[jsonrpc_v2_client: sending request]"
            );

            // send request to the server
            match client.write_all(request.as_bytes()).await {

                Ok(_) => {
                    log::info!(
                        target: "jsonrpc_v2_client",
                        "[jsonrpc_v2_client: request successfully sent]"
                    );
                },
                Err(error) => {
                    log::error!(
                        target: "jsonrpc_v2_client",
                        "[jsonrpc_v2_client: error]: {}",
                        error
                    );
                }
            }
            
            log::trace!(
                target: "jsonrpc_v2_client",
                "[jsonrpc_v2_client: reading response]"
            );
            
            // read the response
            match client.read(&mut buffer).await {
                Ok(size) => {
                    log::info!(
                        target: "jsonrpc_v2_client",
                        "[jsonrpc_v2_client: received response of len = {}]",
                        &size,
                    );
                    buffer_size = size;
                },
                Err(error) => {
                    log::error!(
                        target: "jsonrpc_v2_client",
                        "[jsonrpc_v2_client: error]\r\n{}",
                        error
                    );
                }
            }

            serde_json::from_str(
                String::from_utf8_lossy(
                    &buffer[..buffer_size]
                )
                .split(
                    "\r\n\r\n"
                ).collect::<Vec<&str>>()[1].trim()
            ).unwrap()
            
        })
    }
}
