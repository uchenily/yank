use crate::models::paste_id::PasteId;
use crate::Args;
use clap::Parser;
use rocket::data::{Data, ToByteUnit};
use std::path::Path;

#[post("/", data = "<paste>")]
pub async fn upload(paste: Data<'_>) -> Result<String, std::io::Error> {
    let args = Args::parse();
    let id = PasteId::new(6);

    let filepath = Path::new(&args.upload).join(format!("{id}", id = id));

    paste
        .open(args.binary_upload_limit.mebibytes())
        .into_file(&filepath)
        .await?;

    let url = match tree_magic::from_filepath(&filepath)
        .as_str()
        .contains("text")
    {
        true => format!("/p/{id}", id = id),
        false => format!("/{id}", id = id),
    };

    Ok(url)
}
