// app/src/routes/admin/dashboard.rs

// dependencies
use crate::authorization::{CurrentUser, require_admin};
use crate::errors::ApiError;
use pavex::{get, Response, response::body::Html};
use pavex_tera_template::{Context, TemplateEngine};


// handler which returns the admin dashboard, if the user has the proper role
#[get(path = "/admin")]
pub async fn admin_dashboard(user: &CurrentUser, template: &TemplateEngine) -> Result<Response, ApiError> {
    require_admin(user)?;
    let mut context = Context::new();
    context.insert("title", "Admin");
    context.insert("username", &user.username);
    let body: Html = template.render("admin/index.html", &context)?.into();

    Ok(Response::ok().set_typed_body(body))
}

