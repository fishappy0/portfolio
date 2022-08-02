use crate::types;
use actix_web::{get, web};
use std::sync::Arc;

#[derive(serde::Deserialize, Debug)]
pub struct Query {
    id: String,
}
#[get("/admin")]
pub async fn admin_index(
    template: web::Data<Arc<tera::Tera>>,
) -> crate::types::TeraPage {
    let context = tera::Context::new();
    let tera_page = template.render("admin.html", &context);
    types::TeraPage(tera_page, Arc::clone(&template))
}
#[get("/admin/{page}")]
pub async fn admin_page(
    template: web::Data<Arc<tera::Tera>>,
    page: web::Path<String>,
) -> crate::types::TeraPage {
    let context = tera::Context::new();
    let tera_page = template.render(&("admin/".to_string() + &page.to_string() + ".html"), &context);
    types::TeraPage(tera_page, Arc::clone(&template))
}

#[get("/yes")]
pub async fn yes(
    template: web::Data<Arc<tera::Tera>>,
    query: web::Query<Query>,
) -> crate::types::TeraPage {
    println!("{:?}", query.id);
    let context = tera::Context::new();
    let tera_page = template.render("index.html", &context);
    types::TeraPage(tera_page, Arc::clone(&template))
}