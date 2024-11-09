use std::env;
pub struct ApiConfig {
    pub dev_id: i32,
    pub key: String,
}

impl ApiConfig {
    pub fn build() -> Result<ApiConfig, &'static str> {

        let dev_id = env::var("PTV_DEV_ID").map_err(|_| "Expected environment variable PTV_DEV_ID set")?;

        let dev_id = dev_id.parse().map_err(|_| "Could not parse PTV_DEV_ID to number")?;

        let key = env::var("PTV_API_KEY").map_err(|_| "Expected environment variable PTV_API_KEY set")?;

        Ok(ApiConfig {
            dev_id,
            key,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    fn cleanup_env() {
        env::remove_var("PTV_DEV_ID");
        env::remove_var("PTV_API_KEY");
    }

    #[test]
    fn build_with_valid_config() {
        let ptv_dev_id = 12345;
        let ptv_api_key = "my_api_key";

        env::set_var("PTV_DEV_ID", ptv_dev_id.to_string());
        env::set_var("PTV_API_KEY", ptv_api_key);

        let config = ApiConfig::build();

        let config = config.unwrap();

        assert_eq!(config.dev_id, ptv_dev_id);
        assert_eq!(config.key, ptv_api_key);

        cleanup_env();
    }

    #[test]
    fn build_with_missing_dev_id_returns_error() {
        let ptv_api_key = "my_api_key";

        env::set_var("PTV_API_KEY", ptv_api_key);

        let config = ApiConfig::build();

        assert!(config.is_err());

        if let Err(e) = config {
            assert_eq!(e, "Expected environment variable PTV_DEV_ID set");
        };

        cleanup_env();
    }

    #[test]
    fn build_with_missing_api_key_returns_error() {
        let ptv_dev_id = 12345;

        env::set_var("PTV_DEV_ID", ptv_dev_id.to_string());

        let config = ApiConfig::build();

        assert!(config.is_err());

        if let Err(e) = config {
            assert_eq!(e, "Expected environment variable PTV_API_KEY set");
        };

        cleanup_env();
    }

    #[test]
    fn build_with_invalid_dev_id_returns_error() {
        let ptv_dev_id = "this should fail";
        let ptv_api_key = "my_api_key";

        env::set_var("PTV_DEV_ID", ptv_dev_id);
        env::set_var("PTV_API_KEY", ptv_api_key);

        let config = ApiConfig::build();

        assert!(config.is_err());

        if let Err(e) = config {
            assert_eq!(e, "Could not parse PTV_DEV_ID to number");
        };

        cleanup_env();
    }
}