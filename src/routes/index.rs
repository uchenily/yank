use crate::models::response_wrapper::ResponseWrapper;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub async fn index() -> ResponseWrapper<Template> {
    let mut map = HashMap::new();

    map.insert("title", "bin");
    ResponseWrapper::meta_response(Template::render("index.html", &map))
}
