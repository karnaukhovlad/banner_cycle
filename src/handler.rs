use crate::source::ImageCounter;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use std::collections::HashMap;

pub async fn get_banner(
    req: HttpRequest,
    banner: web::Data<HashMap<String, Vec<ImageCounter>>>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    req.q
    let context = {
        let mut ctx = tera::Context::new();
        ctx.insert("image_url", "CHECK");
        ctx
    };
    let html = tmpl
        .render("banner.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").finish())
}
