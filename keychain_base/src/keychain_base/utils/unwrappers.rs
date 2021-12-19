pub use crate::types::response_types::ErrorResponse;

pub enum UnwrapOrThrowResult<T> {
    Result(T),
    Error(ErrorResponse),
}

pub fn unwrap_or_throw_500<T>(
    data: Option<T>,
    debug_message: String,
    response_message: String,
) -> UnwrapOrThrowResult<T> {
    match data {
        Some(val) => return UnwrapOrThrowResult::Result(val),
        None => {
            eprintln!("{}", &debug_message);
            return UnwrapOrThrowResult::Error(ErrorResponse {
                error_code: 500,
                error_message: response_message,
            });
        }
    };
}
