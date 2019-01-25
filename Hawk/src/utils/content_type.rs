use std::ffi::OsStr;
use std::path::Path;

/**
 * Get the mime type for a local file
*/
pub fn local_file_get_mime(file: String) -> String {
    let extension = Path::new(&file).extension().and_then(OsStr::to_str);

    match extension {
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("jpg") => "image/jpeg",
        _ => "only image required",
    }
    .to_owned()
}
