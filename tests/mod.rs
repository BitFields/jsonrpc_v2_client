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
		use jsonrpc_v2_client::ApiKey;

		let api_key = ApiKey::new("API_KEY", "my-api-key.xxx.yyy.zzz");

		assert_eq!(api_key.as_query_str(), "API_KEY=my-api-key.xxx.yyy.zzz");
		assert_eq!(api_key.as_header(), "API_KEY: my-api-key.xxx.yyy.zzz");
		assert_eq!(api_key.as_cookie(), "Cookie: API_KEY=my-api-key.xxx.yyy.zzz");
	}

}