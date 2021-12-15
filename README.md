# JSONRPC v2 Client

## Features

- logging
- async-std runtime
- APIKey option (sent as http header)

## Examples

``` rust
let service_address = jsonrpc_v2_client::ServiceAddress::new("127.0.0.1:8082", "/api");
let method = "add";
let params = jsonrpc_v2_client::Params([10.5, 20.5]);
let id = 0;
let request = jsonrpc_v2_client::Request::new(method, params, id);
// Without API KEY
let response = request.send(&service_address, None);
println!("{}", response);
// Or with API KEY
let api_key = jsonrpc_v2_client::APIKey::new("API-KEY", "abcdef123456");
let response = request.send(&service_address, Some(&api_key));
println!("{}", response);
// Access JSON field
println!("{}", response["result"]);
println!("{}", response["error"]);
println!("{}", response["id"]);
```
