extern crate firebase_admin;

mod tests {

    use firebase_admin as fb;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn deserialize_into_credentials() {
        let settings = fb::credentials::Credentials::from_file("tests/service");
        
        assert_eq!("https://accounts.google.com/o/oauth2/auth", settings.auth_uri);
        assert_eq!("-----BEGIN PRIVATE KEY-----\n[PRIVATE-KEY]\n-----END PRIVATE KEY-----\n", settings.private_key);
    }

    #[test]
    fn build_and_initialize_app() {
        let app = fb::admin::Admin::app_builder()
            .with_credentials("tests/service")
            .build_and_initialize();
        assert_eq!(app.unwrap().name, "[DEFAULT]");
        
    }

}
