use crate::types;
use actix_web::{get, web};
use std::sync::Arc;

#[derive(serde::Deserialize, Debug)]
pub struct Query {
    id: String,
}
#[get("/")]
pub async fn index(
    template: web::Data<Arc<tera::Tera>>,
) -> crate::types::TeraPage {
    let context = tera::Context::new();
    let tera_page = template.render("index.html", &context);
    types::TeraPage(tera_page, Arc::clone(&template))
}
#[get("/{page}")]
pub async fn home(
    template: web::Data<Arc<tera::Tera>>,
    page: web::Path<String>,
) -> crate::types::TeraPage {
    let context = tera::Context::new();
    let tera_page = template.render(&(page.to_string() + ".html"), &context);
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

// #[get("/admin/{page}")]
// pub async fn admin(
//     template: web::Data<Arc<tera::Tera>>,
//     page: web::Path<String>,
//     // query: web::Query<Query>,
// ) -> crate::types::TeraPage {
//     println!("{}", page);
//     let context = tera::Context::new();
//     let tera_page = template.render(("/admin/" + page.to_string() + ".html"), &context);
//     types::TeraPage(tera_page, Arc::clone(&template))
// }
