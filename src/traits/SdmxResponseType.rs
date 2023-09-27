use serde::de::DeserializeOwned;

pub trait ResponseType {
    type Response: DeserializeOwned;
}
