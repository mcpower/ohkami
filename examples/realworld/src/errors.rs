use ohkami::{IntoResponse, serde::Serialize, typed::ResponseBody};
use std::borrow::Cow;


#[derive(Debug)]
pub enum RealWorldError {
    Config(String),
    DB(sqlx::Error),
    Validation { body: String },
    NotFound(Cow<'static, str>),
    Unauthorized(Cow<'static, str>),
    FoundUnexpectedly(Cow<'static, str>),
} const _: () = {
    impl std::fmt::Display for RealWorldError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{self:?}"))
        }
    }
    impl std::error::Error for RealWorldError {}
};

#[ResponseBody(JSONS)]
struct ValidationErrorFormat {
    errors: ValidationError,
}
#[derive(Serialize, Debug)]
pub struct ValidationError {
    body: Vec<Cow<'static, str>>,
}

impl IntoResponse for RealWorldError {
    fn into_response(self) -> ohkami::Response {
        use ohkami::typed::*;
        
        match self {
            Self::Validation { body } => UnprocessableEntity(
                ValidationErrorFormat {
                    errors: ValidationError {
                        body: vec![body.into()],
                    },
                }
            ).into_response(),
            Self::Config(err_msg)       => InternalServerError(err_msg).into_response(),
            Self::DB(sqlx_err)          => InternalServerError(sqlx_err.to_string()).into_response(),
            Self::NotFound(nf)          => NotFound(nf).into_response(),
            Self::Unauthorized(msg)     => Unauthorized(msg).into_response(),
            Self::FoundUnexpectedly(fu) => BadRequest(fu).into_response(),
        }
    }
}
