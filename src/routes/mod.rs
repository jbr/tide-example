pub mod articles;
use crate::templates::welcome::*;

pub async fn welcome(_request: crate::Request) -> tide::Result {
    Ok(WelcomeTemplate::new("Tide").into())
}
