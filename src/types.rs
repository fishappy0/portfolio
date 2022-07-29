use actix_web::{HttpResponse, Responder};

pub struct TeraPage(
    pub Result<String, tera::Error>,
    pub std::sync::Arc<tera::Tera>,
);
impl Responder for TeraPage {
    type Body = String;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let custom_404 = self.1.render("404.html", &tera::Context::new()).unwrap();
        match self.0 {
            Ok(body) => HttpResponse::Ok()
                .content_type("text/html")
                .finish()
                .set_body(body),
            Err(_) => HttpResponse::NotFound().finish().set_body(custom_404),
        }
    }
}
