use hawk::constants::constant::REGION;
use hawk::upload::image_upload::upload_image;
use hawk::utils::s3_connection::connection;
use s3::{bucket::Bucket, credentials::Credentials};
use hawk::utils::content_type::local_file_get_mime;

#[test]
fn check_connection_for_success() {
    let credentials: Credentials = Credentials::default();
    let access_key: Option<String> = Some(credentials.access_key.to_owned());
    let secret_key: Option<String> = Some(credentials.secret_key.to_owned());
    let bucket: Bucket = connection("hawk-image", access_key, secret_key);
    assert_eq!(&bucket.name, "hawk-image")
}

#[test]
fn check_image_upload_for_success() {
    let credentials: Credentials = Credentials::default();
    let access_key: Option<String> = Some(credentials.access_key.to_owned());
    let secret_key: Option<String> = Some(credentials.secret_key.to_owned());
    let bucket: Bucket = connection("hawk-image", access_key, secret_key);
    assert_eq!(
        upload_image("/home/knoldus/Downloads/new.png".to_string(),
                     "rust", &bucket),
        "Uploaded Successfully"
    )
}

#[test]
#[should_panic]
fn check_image_read_failure() {
    let credentials: Credentials = Credentials::default();
    let access_key: Option<String> = Some(credentials.access_key.to_owned());
    let secret_key: Option<String> = Some(credentials.secret_key.to_owned());
    let bucket: Bucket = connection("hawk-image", access_key, secret_key);
    upload_image("/wrong/path".to_string(), "rust", &bucket);
}

#[test]
#[should_panic]
fn check_image_upload_invalid_bucket() {
    let credentials: Credentials = Credentials::default();
    let access_key: Option<String> = Some(credentials.access_key.to_owned());
    let secret_key: Option<String> = Some(credentials.secret_key.to_owned());
    let bucket_name_invalid: &str = "wrongname";
    let credentials = Credentials::new(access_key, secret_key, None, None);
    let bucket: Bucket = Bucket::new(bucket_name_invalid, REGION, credentials);
    upload_image("/home/knoldus/Downloads/new.png".to_string(),
                            "rust", &bucket);
}

#[test]
#[should_panic]
fn check_image_upload_invalid_credentials() {
    let bad_access_key: Option<String> = Some("access".to_owned());
    let bad_secret_key: Option<String> = Some("secret".to_owned());

    let credentials = Credentials::new(bad_access_key, bad_secret_key, None,
                                       None);
    let bucket = Bucket::new("hawk-image", REGION, credentials);
    upload_image("/home/knoldus/Downloads/new.png".to_string(),
                     "rust", &bucket);
}

#[test]
fn test_local_file_get_mime() {
    assert_eq!(local_file_get_mime("/home/knoldus/Downloads/new.png".to_string()),"image/png"
        .to_string())
}
