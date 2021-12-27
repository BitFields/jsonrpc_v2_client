#[cfg(test)]
mod tests {

    #[test]
    fn test_params() {
        use jsonrpc_v2_client::Params;

        let params_0 = Params(1);
        assert_eq!(params_0.0, 1);

        let params_1 = Params([0, 1]);
        assert_eq!(params_1.0, [0, 1]);

        let params_2 = Params("hello");
        assert_eq!(params_2.0, "hello");
    }

    #[test]
    fn test_api_key() {
        use jsonrpc_v2_client::APIKey;

        let api_key = APIKey::new("API_KEY", "abcdef12345678");
        assert_eq!(api_key.as_header(), "API_KEY: abcdef12345678");
    }

    #[test]
    fn test_request_with_api_key() {
        use jsonrpc_v2_client::APIKey;
        use jsonrpc_v2_client::Params;
        use jsonrpc_v2_client::Request;
        use jsonrpc_v2_client::ServiceAddress;
        use serde_json::Value::Null;


        let api_key = APIKey::new("X-API-KEY", "abcdef12345678");
        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        let req = Request::new("mul", Params([2.5, 3.5]), 0);

        let response = req.send(&service_address, Some(&api_key));

        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["result"], 8.75);
        assert_eq!(response["id"], 0);
        assert_eq!(response["error"], Null);
    }

    #[test]
    fn test_request_without_api_key() {
        use jsonrpc_v2_client::Params;
        use jsonrpc_v2_client::Request;
        use jsonrpc_v2_client::ServiceAddress;
        use serde_json::Value::Null;

        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        let req = Request::new("mul", Params([2.5, 3.5]), "0");

        let response = req.send(&service_address, None);

        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["result"], 8.75);
        assert_eq!(response["id"], 0);
        assert_eq!(response["error"], Null);
    }

    #[test]
    fn test_request_error_response() {
        use jsonrpc_v2_client::Params;
        use jsonrpc_v2_client::Request;
        use jsonrpc_v2_client::ServiceAddress;
        use serde_json::Value::Null;

        let service_address = ServiceAddress::new("127.0.0.1:8082", "/api");
        // error produced here, this method takes only 2 params
        let req = Request::new("mul", Params([2.5, 3.5, 3.0]), "0");

        let response = req.send(&service_address, None);

        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["result"], Null);
        assert_eq!(response["id"], 0);
        assert_eq!(response["error"]["code"], -32602);
        assert_eq!(response["error"]["message"], "Invalid params");
    }
}
