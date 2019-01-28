use hawk::constants::constant::{ARGUMENTS_LENGTH, DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE};
use hawk::models::configuration::Configuration;
use hawk::upload::image_upload::upload_image;
use hawk::utils::s3_connection::connection;
use log::info;
use s3::bucket::Bucket;
use std::env::args;
use colored::*;

/**
 *This project is intended to get the file from the user and upload into the S3 bucket.
 *Here,we create connection with S3
 *Then instantiate the bucket
 *Takes local file
 *Uploads file to S3
 **/
#[cfg_attr(tarpaulin, skip)]
fn main() {
    std::env::set_var(DEBUG_LEVEL_KEY, DEBUG_LEVEL_VALUE);
    env_logger::init();

    if args().len() != ARGUMENTS_LENGTH {
        info!("{}", "\n Required user's input in this order:\n BUCKET_NAME ACCESS_KEY SECRET_KEY \
         LOCAL_FILE_PATH BUCKET_FILE_NAME".yellow());
        return;
    }
    let config: Configuration = Configuration::config();
    let bucket: Bucket = connection(
        config.bucket_name.as_str(),
        config.access_key,
        config.secret_key,
    );
    println!("1");
    info!("{}", upload_image(config.local_file_path,config.bucket_file_name.as_str(),
        &bucket).blue().bold());println!("1");
}
