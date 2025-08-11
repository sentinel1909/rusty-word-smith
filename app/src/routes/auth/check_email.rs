use crate::errors::ApiError;
use pavex::{get, Response, response::body::Html};
use pavex_tera_template::{Context, TemplateEngine};

#[get(path = "/auth/check-email")]
pub fn check_email_page(template: &TemplateEngine) -> Result<Response, ApiError> {
    let mut context = Context::new();
    context.insert("title", "Check your email");
    let body: Html = template.render("auth/check_email.html", &context)?.into();
    Ok(Response::ok().set_typed_body(body))
}


