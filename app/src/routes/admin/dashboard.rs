// app/src/routes/admin/dashboard.rs

// dependencies
use pavex::get;
use crate::authorization::{CurrentUser, require_admin};
use crate::{errors::ApiError, response::ApiResponse};

// handler which returns the admin dashboard, if the user has the proper role
#[get(path = "/admin")]
pub async fn admin_dashboard(user: &CurrentUser) -> Result<ApiResponse<&'static str>, ApiError> {
    require_admin(user)?;
    Ok(ApiResponse::ok("ok"))
}

