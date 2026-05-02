use crate::api::auth::types::{AuthResponse, UserResponse};
use crate::state::GoogleOidcConfig;

pub(super) fn frontend_callback_url(config: &GoogleOidcConfig, auth: AuthResponse) -> String {
    let user: UserResponse = auth.user;
    let mut fragment = url::form_urlencoded::Serializer::new(String::new());
    fragment
        .append_pair("token", &auth.token)
        .append_pair("token_type", &auth.token_type)
        .append_pair("expires_in", &auth.expires_in.to_string())
        .append_pair("user_id", &user.id)
        .append_pair("user_name", &user.name)
        .append_pair("user_created_at", &user.created_at);

    if let Some(email) = user.email {
        fragment.append_pair("user_email", &email);
    }

    if let Some(account_number) = user.account_number {
        fragment.append_pair("user_account_number", &account_number);
    }

    format!("{}#{}", config.success_redirect_path, fragment.finish())
}
