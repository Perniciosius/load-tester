#[derive(Clone, PartialEq)]
pub enum ResponseType {
    Success,
    Redirect,
    ClientError,
    ServerError,
    Timeout,
}