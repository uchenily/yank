use crate::models::response_wrapper::ResponseWrapper;
use crate::Args;
use clap::Parser;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub async fn index() -> ResponseWrapper<Template> {
    let mut map = HashMap::new();

    // whether to include `/client` info
    let client_desc = match Args::parse().client_desc {
        true => "placeholder",
        false => "",
    };

    map.insert("title", "bin");
    map.insert("client_desc", client_desc);

    ResponseWrapper::meta_response(Template::render("index.html", &map))
}
