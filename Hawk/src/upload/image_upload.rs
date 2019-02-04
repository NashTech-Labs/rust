use crate::constants::constant::STATUS_CODE;
use crate::utils::content_type::local_file_get_mime;
use log::error;
use s3::bucket::Bucket;
use std::fs;

/**
 *The method upload_image() is used to take image from the user and upload image to S3 bucket
 */
pub fn upload_image(
    local_file_path: String,
    bucket_file_name: &str,
    bucket: &Bucket,
) -> &'static str {
    let content_type: String = local_file_get_mime(local_file_path.clone());
    if content_type.eq("only image required") {
        error!("only image required")
    }
    ///Read the entire contents of a file into a bytes vector.
    match fs::read(local_file_path.as_str()) {
        ///Put into an S3 bucket.
        Ok(image) => match bucket.put(bucket_file_name, &image, content_type.as_str()) {
            Ok(response) => {
                assert_eq!(response.1, STATUS_CODE);
                "Uploaded Successfully"
            }
            Err(s3error) => panic!("{:?}", s3error),
        },
        Err(file_error) => panic!("{:?}", file_error),
    }
}
