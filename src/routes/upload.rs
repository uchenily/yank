use crate::models::paste_id::PasteId;
use crate::Args;
use clap::Parser;
use rocket::data::{Data, ToByteUnit};
// use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use std::fmt;
use std::path::Path;

pub struct Filename<'r>(Option<&'r str>);

impl<'a> fmt::Display for Filename<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(filename) = self.0 {
            write!(f, "{}", filename)
        } else {
            write!(f, "{}", "default")
        }
    }
}

// #[derive(Debug)]
// pub enum FilenameError {
//     Missing,
//     // Invalid,
// }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Filename<'r> {
    // type Error = FilenameError;
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-Filename") {
            None => {
                // Outcome::Error((Status::BadRequest, FilenameError::Missing))
                Outcome::Success(Filename(None))
            }
            Some(key) => Outcome::Success(Filename(Some(key))),
        }
    }
}

// curl --data-binary @test.file  --url localhost:6162 -H "X-Filename: test.file"
#[post("/", data = "<paste>")]
pub async fn upload(
    paste: Data<'_>,
    filename: Filename<'_>,
) -> Result<String, std::io::Error> {
    let args = Args::parse();

    let id = match filename.0 {
        Some(filename) => filename.to_string(),
        None => {
            let id = PasteId::new(8);
            id.to_string()
        }
    };

    let filepath = Path::new(&args.upload).join(format!("{}", id));

    paste
        .open(args.binary_upload_limit.mebibytes())
        .into_file(&filepath)
        .await?;

    let url = match tree_magic::from_filepath(&filepath)
        .as_str()
        .contains("text")
    {
        true => format!("/h/{id}", id = id), // never be true?
        false => format!("/{id}", id = id),
    };

    Ok(url)
}
