mod request;  pub use request::*;
mod response; pub use response::*;


#[cfg(feature="utils")]
#[cfg(test)] #[allow(unused)] mod __ {
    use crate::{Status, typed::ResponseBody, IntoResponse, Response};

    async fn handler_1() -> Status {
        Status::NoContent
    }

    #[derive(::serde::Serialize)]
    struct Length {
        value: usize
    } const _: () = {
        impl ResponseBody for Length {
            fn into_response_with(self, status: Status) -> crate::Response {
                Response::with(status).json(self)
            }
        }
    };
    impl Length {
        fn new() -> Result<Self, LengthError> {
            Ok(Self { value: 42 })
        }
    }

    enum LengthError {
        TODO,
    } const _: () = {
        impl IntoResponse for LengthError {
            fn into_response(self) -> crate::Response {
                match self {
                    Self::TODO => Status::NotImplemented.into_response()
                }
            }
        }
    };

    async fn handler_2() -> Result<Length, LengthError> {
        let length = Length::new()?;
        Ok(length)
    }
}
