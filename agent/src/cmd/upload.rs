use std::error::Error;
use std::fs::read;
use std::env::{set_var, remove_var};
use aws_config::{meta::region::RegionProviderChain, from_env};
use aws_smithy_http::byte_stream::{ByteStream, AggregatedBytes};
use aws_smithy_http::body::SdkBody;
use crate::cmd::CommandArgs;
use crate::config::ConfigOptions;
use aws_sdk_s3::{Client, Error as S3Error, model::ObjectCannedAcl};
use crate::cmd::notion_out;
use litcrypt::lc;


/// Uploads a file
pub async fn handle(cmd_args: &mut CommandArgs, config_options: &mut ConfigOptions) -> Result<String, Box<dyn Error>> {

    let aws_access_key_id: String;
    let aws_secret_access_key: String;
    let bucket_name: String;
    let upload_file: String;

    if let Some(aaki) = cmd_args.nth(0) {
        println!("AWS Access Key ID: {aaki}");
        aws_access_key_id = aaki;
    } else {
        return notion_out!(("Missing AWS Access ID"));
    }

    if let Some(asak) = cmd_args.nth(0) {
        println!("AWS Access Secret: {asak}");
        aws_secret_access_key = asak;
    } else {
        return notion_out!(("Missing AWS Secret Access Key"));
    }

    if let Some(bn) = cmd_args.nth(0) {
        println!("Bucket Name: {bn}");
        bucket_name = bn;
    } else {
        return notion_out!(("Missing bucket name"));
    }

    if let Some(uf) = cmd_args.nth(0) {
        println!("Bucket Name: {uf}");
        upload_file = uf;
    } else {
        return notion_out!(("Missing upload file"));
    }

    set_var("AWS_ACCESS_KEY_ID", aws_access_key_id);
    set_var("AWS_SECRET_ACCESS_KEY", aws_secret_access_key);

    let region_provider = RegionProviderChain::default_provider().or_else("us-west-1");
    let config = aws_config::from_env()
    .region(region_provider).load().await;
    let client = Client::new(&config);

    // let resp = client.list_buckets().send().await.unwrap();
    // println!("Buckets:");

    let file = read(&upload_file).unwrap();
    let stream = ByteStream::new(SdkBody::from(file));
    let region_string = config.region().unwrap().to_string();

    let res = client.put_object()
        .bucket(&bucket_name)
        .acl(ObjectCannedAcl::PublicRead)
        .key(&upload_file)
        .body(stream)
        .send()
        .await
        .unwrap();

    remove_var("AWS_ACCESS_KEY_ID");
    remove_var("AWS_SECRET_ACCESS_KEY");

    notion_out!("File uploaded: ", &format!("https://{}.s3.{}.amazonaws.com/{}", bucket_name, region_string, upload_file))

}