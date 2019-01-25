# Image Upload to AWS S3

This application is used to upload an image on AWS S3 Bucket. It uses Rust programming language and generates a binary file which can be run on any machine like Windows, Linux, Raspberry Pi or Tinkerboard.

### Intro

An interface towards AWS S3 which supports simple put function to put the image on Bucket.

It takes the following parameters
1. AWS Access Key
2. AWS Secret Key
3. S3 Bucket Name
4. S3 Bucket Region
5. Local file path on System
6. File path on Bucket

### Configuration

Put function uses AWS Credentialss like access key, secret key, bucket name, bucket region, local file path on system and file path on bucket.

### Usage

*In your Cargo.toml*

```
[dependencies]
rust-s3 = "0.11.0"
```
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

## Acknowledgments

* https://doc.rust-lang.org/


