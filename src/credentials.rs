
#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

impl Credentials {

    // Attempts to read from ...
    pub fn from_file(file: &str) -> Credentials {
        let mut settings = config::Config::new();
        settings.merge(config::File::with_name(file)).unwrap();
        println!("{}", settings.get_str("project_id").unwrap());
        settings.try_into().unwrap()
    }
}
