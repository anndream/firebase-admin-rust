#[derive(Clone)]
pub struct ErrorInfo<'a> {
    code: &'a str,
    message: &'a str,
}

pub struct FirebaseArrayIndexError {
    index: i32,
    error: FirebaseError
}

trait FirebaseError: Err {

}