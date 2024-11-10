use std::env;
use sha1::Sha1;
use hmac::{Hmac, KeyInit, Mac};
use hex;

type HmacSha1 = Hmac<Sha1>;
const BASE_URL: &str = "http://timetableapi.ptv.vic.gov.au";

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
            key
        })
    }

    pub fn generate_url(&self, request: &str ) -> Result<String, &'static str> {
        let separator = if request.contains('?') {'&'} else {'?'};

        let mut mac = HmacSha1::new_from_slice(self.key.as_bytes()).map_err(|_| "Error creating HMAC")?;

        let unsigned_request = format!("{}{}devid={}", request, separator, self.dev_id );
        mac.update(unsigned_request.as_bytes());

        let signature = mac.finalize().into_bytes();
        let signature = hex::encode(signature);

        Ok(format!("{BASE_URL}{unsigned_request}&signature={signature}"))
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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

    #[test]
    #[ignore]
    fn generate_url() {
        let ptv_dev_id = 12345;
        let ptv_api_key = "my_api_key";

        env::set_var("PTV_DEV_ID", ptv_dev_id.to_string());
        env::set_var("PTV_API_KEY", ptv_api_key);

        let config = ApiConfig::build();

        let config = config.unwrap();

        assert_eq!(config.dev_id, ptv_dev_id);
        assert_eq!(config.key, ptv_api_key);

        let url = config.generate_url("my_url");

        assert!(url.is_ok());

        cleanup_env();
    }
}