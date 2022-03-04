use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize, PartialEq, Debug)]
pub struct ErrorResponse {
    pub error_code: u16,
    pub error_message: String,
}

#[derive(Clone, CandidType, Deserialize, PartialEq, Debug)]
pub struct SuccessResponse<T> {
    pub code: u16,
    pub data: Option<T>,
}

#[derive(Clone, CandidType, Deserialize, PartialEq, Debug)]
pub enum Response<T> {
    Success(SuccessResponse<T>),
    Error(ErrorResponse),
}

// this is the type to use if there is no data for SuccessResponse to return.
// it should be something that is candid-typed, and there're no nulls in rust.
pub type NoResponseData = u8;
