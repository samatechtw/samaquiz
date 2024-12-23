use crate::error::api_error::ApiError;
use reqwest::Url;
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::time::Duration;
use tracing::warn;

#[derive(Clone)]
pub struct S3Client {
    credentials: Credentials,
    pub site_asset_bucket: Bucket,
}

impl S3Client {
    pub fn new(s3_url: String, s3_access_key_id: String, s3_secret_access_key: String) -> S3Client {
        if s3_url.is_empty() || s3_secret_access_key.is_empty() || s3_access_key_id.is_empty() {
            warn!("Missing S3 configuration, please check the following information is provided: S3_URL, S3_ACCESS_KEY_ID, S3_SECRET_ACCESS_KEY")
        }
        let endpoint: Url = s3_url.parse().expect("s3 endpoint is invalid");
        let path_style = UrlStyle::Path;
        let site_asset_name = "site-assets";
        let region = "auto";
        let site_asset_bucket = Bucket::new(endpoint.clone(), path_style, site_asset_name, region)
            .expect("site-asset bucket url is invalid");

        let credentials = Credentials::new(s3_access_key_id, s3_secret_access_key);

        S3Client {
            credentials,
            site_asset_bucket,
        }
    }

    fn presign_put(
        &self,
        bucket: &Bucket,
        filename: &str,
        expires: u64,
        content_type: &str,
        size: String,
    ) -> Result<Url, ApiError> {
        // Sign a request
        let presigned_url_duration = Duration::from_secs(expires);
        let mut action = bucket.put_object(Some(&self.credentials), filename);

        let query = action.query_mut();
        query.insert("Content-Type", content_type);
        query.insert("Content-Length", size);

        Ok(action.sign(presigned_url_duration))
    }

    async fn delete_asset(&self, bucket: &Bucket, object_key: &str) -> Result<(), ApiError> {
        let delete_object = bucket.delete_object(Some(&self.credentials), &object_key);

        let expires_in = Duration::from_secs(600);
        let url = delete_object.sign(expires_in);

        match reqwest::Client::new().delete(url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    let error_message = match res.text().await {
                        Ok(text) => {
                            format!("Delete asset fail: {}, Response: {}", object_key, text)
                        }
                        Err(_) => format!("Delete asset fail: {}", object_key),
                    };

                    Err(ApiError::internal_error().message(error_message))
                }
            }
            Err(err) => Err(ApiError::internal_error().message(format!(
                "Failed to send DELETE request: {}, {}",
                object_key, err
            ))),
        }
    }

    pub fn presign_put_site_asset(
        &self,
        filename: &str,
        expires: u64,
        content_type: &str,
        size: i64,
    ) -> Result<Url, ApiError> {
        self.presign_put(
            &self.site_asset_bucket,
            filename,
            expires,
            content_type,
            size.to_string(),
        )
    }

    pub async fn verify_site_asset(&self, object_key: &str) -> Result<bool, ApiError> {
        let head_object = self
            .site_asset_bucket
            .head_object(Some(&self.credentials), &object_key);

        let expires_in = Duration::from_secs(600);
        let url = head_object.sign(expires_in);

        match reqwest::Client::new().head(url).send().await {
            Ok(res) => Ok(res.status().is_success()),
            Err(err) => Err(ApiError::internal_error()
                .message("Failed to send HEAD request".to_string() + &err.to_string())),
        }
    }

    pub async fn delete_site_asset(&self, object_key: &str) -> Result<(), ApiError> {
        self.delete_asset(&self.site_asset_bucket, object_key).await
    }
}
