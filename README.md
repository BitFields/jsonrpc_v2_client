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
let id = "0";
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

## Logging

log target name is: jsonrpc_v2_client  
example using log4rs

file: config.yaml  

``` yaml
# Scan this file for changes every x seconds
refresh_rate: 30 seconds

appenders:
  stdout:
    kind: file
    path: "log/logs/stdout.log"
    encoder:
      pattern: "{d} - {m}{n}"

  jsonrpc_v2_client:
    kind: file
    path: "log/logs/jsonrpc_v2_client.log"
    encoder:
      pattern: "{d} - {m}{n}"

root:
  level: info
  appenders:
    - stdout

  jsonrpc_v2_client:
    level: trace
    appenders:
      - jsonrpc_v2_client
    additive: false
```

log output example:

``` log
2021-12-15T14:58:35.293410700+01:00 - [jsonrpc_v2_client: request as string]
POST /api HTTP/1.1
Content-Type: application/json
User-Agent: jsonrpc_v2_client
Accept: application/json
X-API-KEY: 1q2w3e4r5t
Content-Length: 100

{
  "jsonrpc": "2.0",
  "method": "mul",
  "params": [
    10.3,
    10.1,
    12.2
  ],
  "id": "0"
}
2021-12-15T14:58:35.294005600+01:00 - [jsonrpc_v2_client: sending request]
2021-12-15T14:58:35.294470100+01:00 - [jsonrpc_v2_client: request successfully sent]
2021-12-15T14:58:35.294679+01:00 - [jsonrpc_v2_client: reading response]
2021-12-15T14:58:35.297373900+01:00 - [jsonrpc_v2_client: received response of len = 183]
2021-12-15T14:58:35.300850900+01:00 - [jsonrpc_v2_client: request as string]
```
