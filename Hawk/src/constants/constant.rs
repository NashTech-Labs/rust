use s3::region::Region;

///REGION describes the region of the bucket
pub const REGION: Region = Region::ApSouth1;

///CONTENT_TYPE_IMAGE describes the type of the file
pub const CONTENT_TYPE: &str = "image/jpeg";

///OK_STATUS_CODE describes the success code used
pub const STATUS_CODE: u32 = 200;

///ARGUMENTS_LENGTH describes the length of valid number of cmd line arguments
pub const ARGUMENTS_LENGTH: usize = 6;

///ACCESS_KEY_LENGTH describes the valid length of access key
pub const ACCESS_KEY_LENGTH: usize = 20;

///SECRET_KEY_LENGTH describes the valid length of secret key
pub const SECRET_KEY_LENGTH: usize = 40;

pub static DEBUG_LEVEL_KEY: &str = "RUST_LOG";

pub static DEBUG_LEVEL_VALUE: &str = "hawk=debug";
