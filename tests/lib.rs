extern crate firebase_admin;

#[cfg(test)]
mod tests {
    use firebase_admin::admin::{Admin};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn initialize_default_app() {
        match Admin::initialize_app(None, None) {
            Ok(app) => assert_eq!(app.name, "[DEFAULT]"),
            Err(err_string) => panic!(err_string)
        };
        assert_eq!(3, 3)
    }

}
