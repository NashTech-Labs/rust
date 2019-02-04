use crate::constants::constant;
use s3::{bucket::Bucket, credentials::Credentials, region::Region};

///This method connection() establishes connection with S3 bucket and returns the bucket instance
pub fn connection(
    bucket_name: &str,
    access_key: Option<String>,
    secret_key: Option<String>,
) -> Bucket {
    let region: Region = constant::REGION;
    ///AWS access credentials: access key, secret key, and optional token
    let credentials: Credentials =
        Credentials::new(access_key.clone(), secret_key.clone(), None, None);
    ///Instantiate a new `Bucket`
    Bucket::new(bucket_name, region, credentials)
}
