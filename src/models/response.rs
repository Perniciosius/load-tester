use crate::models::response_type::ResponseType;

#[derive(Clone)]
pub struct Response {
    pub res_type: ResponseType,
    pub path: String,
    pub res_time: u128,
}