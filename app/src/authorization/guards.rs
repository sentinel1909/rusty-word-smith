// app/src/authorization/guards.rs

// dependencies
use crate::authorization::CurrentUser;
use crate::errors::ApiError;
use crate::models::UserRole;

// guard funciton which takes the current user as input, checks their role
pub fn require_admin(user: &CurrentUser) -> Result<(), ApiError> {
    if matches!(user.role, UserRole::Admin) {
        Ok(())
    } else {
        Err(ApiError::Forbidden("Admin access required".into()))
    }
}

pub fn require_roles(user: &CurrentUser, allowed: &[UserRole]) -> Result<(), ApiError> {
    if allowed.contains(&user.role) {
        Ok(())
    } else {
        Err(ApiError::Forbidden("Insufficient permissions".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::{require_admin, require_roles};
    use crate::authorization::CurrentUser;
    use crate::errors::ApiError;
    use crate::models::UserRole;
    use uuid::Uuid;

    fn user_with(role: UserRole) -> CurrentUser {
        CurrentUser {
            id: Uuid::new_v4(),
            username: "test".to_string(),
            role,
        }
    }

    #[test]
    fn require_admin_allows_admin() {
        let user = user_with(UserRole::Admin);
        assert!(require_admin(&user).is_ok());
    }

    #[test]
    fn require_admin_rejects_non_admins() {
        for role in [
            UserRole::Editor,
            UserRole::Author,
            UserRole::Contributor,
            UserRole::Subscriber,
        ] {
            let user = user_with(role);
            let err = require_admin(&user).unwrap_err();
            match err {
                ApiError::Forbidden(_) => {}
                _ => panic!("expected Forbidden for role {:?}", role),
            }
        }
    }

    #[test]
    fn require_roles_allows_allowed_role() {
        let user = user_with(UserRole::Editor);
        let allowed = [UserRole::Admin, UserRole::Editor];
        assert!(require_roles(&user, &allowed).is_ok());
    }

    #[test]
    fn require_roles_rejects_disallowed_role() {
        let user = user_with(UserRole::Author);
        let allowed = [UserRole::Admin, UserRole::Editor];
        let err = require_roles(&user, &allowed).unwrap_err();
        matches!(err, ApiError::Forbidden(_));
    }
}
