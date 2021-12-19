mod auth;
mod client;
mod errors;
mod object;
mod utils;

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::object::ObjectAPI;

    #[tokio::test]
    async fn test_upload_object() {
        let cli = client::COS::new(
            "tmp_secret_id",
            "tmp_secret_key",
            "",
            "https://xxxx.cos.ap-guangzhou.myqcloud.com",
        );
        let res = cli
            .put_object_from_file("README.md".to_string(), "/README.md".to_string(), None)
            .await;
        assert!(res.is_ok());
    }
}
