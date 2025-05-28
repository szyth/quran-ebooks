pub fn check_envs() -> Option<bool> {
    let envs = vec![
        "QURAN_DOT_COM_API_URL",
        "QURAN_DOT_COM_AUTH_URL",
        "QURAN_DOT_COM_CLIENT_ID",
        "QURAN_DOT_COM_CLIENT_SECRET",
    ];
    for env in envs {
        let env_var = std::env::var(env);
        if env_var.is_err() || env_var.unwrap().is_empty() {
            return None;
        }
    }
    Some(true)
}

pub fn api_url() -> Option<String> {
    std::env::var("QURAN_DOT_COM_API_URL").ok()
}

pub fn auth_url() -> Option<String> {
    std::env::var("QURAN_DOT_COM_AUTH_URL").ok()
}
pub fn client_id() -> Option<String> {
    std::env::var("QURAN_DOT_COM_CLIENT_ID").ok()
}

pub fn client_secret() -> Option<String> {
    std::env::var("QURAN_DOT_COM_CLIENT_SECRET").ok()
}

pub fn access_token() -> Option<String> {
    std::env::var("QURAN_DOT_COM_ACCESS_TOKEN").ok()
}
