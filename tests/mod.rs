#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    // Existing test for Params
    #[test]
    fn test_params() {
        let params_0 = Params(1);
        assert_eq!(params_0.0, 1);

        let params_1 = Params([0, 1]);
        assert_eq!(params_1.0, [0, 1]);

        let params_2 = Params("hello");
        assert_eq!(params_2.0, "hello");
    }

    // Existing test for APIKey
    #[test]
    fn test_api_key() {
        let api_key = APIKey::new("API_KEY", "abcdef12345678");
        assert_eq!(api_key.as_header(), "API_KEY: abcdef12345678");
    }

    // Enhanced test for ServiceAddress
    #[test]
    fn test_service_address() {
        let addr = ServiceAddress::new("localhost:8080/", "/api/");
        assert_eq!(addr.url, "localhost:8080");
        assert_eq!(addr.endpoint, "api");
        assert_eq!(addr.full_path(), "localhost:8080/api");

        let addr2 = ServiceAddress::new("http://example.com", "rpc");
        assert_eq!(addr2.full_path(), "http://example.com/rpc");
    }

    // Updated test with error handling (assuming a running server at 127.0.0.1:8082)
    #[test]
    fn test_request_with_api_key() {
        let api_key = APIKey::new("X-API-KEY", "abcdef12345678");
        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        let req = Request::new("mul", Params([2.5, 3.5]), "0");

        match req.send(&service_address, Some(&api_key)) {
            Ok(response) => {
                assert_eq!(response["jsonrpc"], "2.0");
                assert_eq!(response["result"], 8.75);
                assert_eq!(response["id"], "0");
                assert_eq!(response["error"], Value::Null);
            }
            Err(e) => panic!("Request failed: {}", e),
        }
    }

    // Updated test with error handling
    #[test]
    fn test_request_without_api_key() {
        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        let req = Request::new("mul", Params([2.5, 3.5]), "0");

        match req.send(&service_address, None) {
            Ok(response) => {
                assert_eq!(response["jsonrpc"], "2.0");
                assert_eq!(response["result"], 8.75);
                assert_eq!(response["id"], "0");
                assert_eq!(response["error"], Value::Null);
            }
            Err(e) => panic!("Request failed: {}", e),
        }
    }

    // Updated test with error handling
    #[test]
    fn test_request_error_response() {
        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        let req = Request::new("mul", Params([2.5, 3.5, 3.0]), "0");

        match req.send(&service_address, None) {
            Ok(response) => {
                assert_eq!(response["jsonrpc"], "2.0");
                assert_eq!(response["result"], Value::Null);
                assert_eq!(response["id"], "0");
                assert_eq!(response["error"]["code"], -32602);
                assert_eq!(response["error"]["message"], "Invalid params");
            }
            Err(e) => panic!("Request failed: {}", e),
        }
    }

    // New test for invalid connection
    #[test]
    fn test_request_connection_failure() {
        let service_address = ServiceAddress::new("127.0.0.1:9999", "/api"); // Assuming no server at 9999
        let req = Request::new("mul", Params([2.5, 3.5]), "0");

        match req.send(&service_address, None) {
            Ok(_) => panic!("Expected connection failure"),
            Err(e) => match e {
                JsonRpcError::ConnectionError(_) => assert!(true),
                _ => panic!("Expected ConnectionError, got: {}", e),
            },
        }
    }

    // New test for serialization
    #[test]
    fn test_request_serialization() {
        let req = Request::new("test", Params(vec![1, 2, 3]), "1");
        let json = serde_json::to_string(&req).unwrap();
        let expected = r#"{"jsonrpc":"2.0","method":"test","params":[1,2,3],"id":"1"}"#;
        assert_eq!(json, expected);
    }

    // New async test
    #[async_std::test]
    async fn test_request_async() {
        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        let req = Request::new("mul", Params([2.5, 3.5]), "0");

        match req.send_async(&service_address, None).await {
            Ok(response) => {
                assert_eq!(response["jsonrpc"], "2.0");
                assert_eq!(response["result"], 8.75);
                assert_eq!(response["id"], "0");
                assert_eq!(response["error"], Value::Null);
            }
            Err(e) => panic!("Async request failed: {}", e),
        }
    }

    // New test for client config
    #[test]
    fn test_client_config() {
        let config = ClientConfig::default();
        assert_eq!(config.timeout, Some(std::time::Duration::from_secs(30)));
        assert_eq!(config.max_buffer_size, 4 * 1024 * 1024);

        let custom_config = ClientConfig {
            timeout: None,
            max_buffer_size: 1024,
        };
        assert_eq!(custom_config.timeout, None);
        assert_eq!(custom_config.max_buffer_size, 1024);
    }
}
