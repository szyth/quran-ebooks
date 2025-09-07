use once_cell::sync::OnceCell;
use reqwest::Client;

pub static HTTP_CLIENT: OnceCell<Client> = OnceCell::new();
pub static ACCESS_TOKEN: OnceCell<String> = OnceCell::new();
