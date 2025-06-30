use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryLimit {
    pub limit: Option<u16>,
}
