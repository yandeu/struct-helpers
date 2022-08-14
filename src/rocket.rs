#[cfg(feature = "rocket")]
pub mod guard {
    pub extern crate rocket;
    use crate::Helpers;
    use rocket::{
        data::{Data, FromData, Outcome as DataOutcome},
        http::Status,
        outcome::Outcome,
        request::Request,
        serde::json::Json,
    };
    use std::fmt::Debug;

    //  Struct used for Request Guards
    #[derive(Clone, Debug)]
    pub struct HelpersGuard<T>(pub T);

    //  Impl to get type T of `Json`
    impl<T> HelpersGuard<Json<T>> {
        pub fn into_deep_inner(self) -> T {
            self.0 .0
        }
    }

    #[rocket::async_trait]
    impl<'r, D> FromData<'r> for HelpersGuard<Json<D>>
    where
        D: Helpers + rocket::serde::Deserialize<'r>,
    {
        type Error = Result<String, rocket::serde::json::Error<'r>>;

        async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> DataOutcome<'r, Self> {
            let data_outcome = <Json<D> as FromData<'r>>::from_data(req, data).await;

            match data_outcome {
                Outcome::Failure((status, err)) => Outcome::Failure((status, Err(err))),
                Outcome::Forward(err) => Outcome::Forward(err),
                Outcome::Success(mut data) => {
                    let success = data.0.run_helpers();

                    if success {
                        Outcome::Success(HelpersGuard(data))
                    } else {
                        Outcome::Failure((
                            Status::BadRequest,
                            Ok("Something went wrong".to_string()),
                        ))
                    }
                }
            }
        }
    }
}
