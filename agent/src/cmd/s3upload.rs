use std::error::Error;
use std::fs::read;
use std::env::{set_var, remove_var};
use aws_config::{meta::region::{RegionProviderChain}, from_env};
use aws_smithy_http::byte_stream::{ByteStream};
use aws_smithy_http::body::SdkBody;
use crate::cmd::CommandArgs;
use crate::config::ConfigOptions;
use crate::logger::{Logger, log_out };
use aws_sdk_s3::{
    Client, 
    model::ObjectCannedAcl, 
};
use aws_types::region::Region;
use crate::cmd::command_out;
use litcrypt::lc;

/// Uploads a file to S3 Storage.
/// 
/// Usage: `s3upload aws_access_key_id aws_secret_access_key region bucket_name filename`
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {

    let mut aws_access_key_id = String::from("");
    let mut aws_secret_access_key = String::from("");
    let mut region = String::from("");
    let mut bucket_name = String::from("");
    let mut upload_file = String::from("");

    let arg_vals = vec![
        (&mut aws_access_key_id, "AWS Access Key ID"),
        (&mut aws_secret_access_key, "AWS Secret Access Key"),
        (&mut region, "Region"),
        (&mut bucket_name, "Bucket Name"),
        (&mut upload_file, "Upload File")
    ];

    for (v, m) in arg_vals {
        if let Some(a) = cmd_args.nth(0) {
            logger.debug(log_out!("Arg:", m, ":", &a));
            *v = a;
        } else {
            return command_out!(("Missing ", m));
        }

    }

    set_var("AWS_ACCESS_KEY_ID", &aws_access_key_id);
    set_var("AWS_SECRET_ACCESS_KEY", &aws_secret_access_key);

    let region_provider = RegionProviderChain::first_try(Region::new(region.to_owned()));
    let config = from_env()
    .region(region_provider).load().await;
    let client = Client::new(&config);

    let file: Vec<u8> = match read(&upload_file) {
        Ok(v) => v,
        Err(e) => { return Ok(e.to_string()); }
    };

    let stream = ByteStream::new(SdkBody::from(file));

    let res = client.put_object()
        .bucket(&bucket_name)
        .acl(ObjectCannedAcl::PublicRead)
        .key(&upload_file)
        .body(stream)
        .send()
        .await;

    remove_var("AWS_ACCESS_KEY_ID");
    remove_var("AWS_SECRET_ACCESS_KEY");

    match res {
        Ok(_) => command_out!("File uploaded: ", &format!("https://{}.s3.{}.amazonaws.com/{}", bucket_name, region, upload_file)),
        _ => command_out!("Upload Error")
    }

}