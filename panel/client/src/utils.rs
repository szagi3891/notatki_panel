#[derive(PartialEq)]
pub enum Resource<T: PartialEq> {
    Loading,
    Ready(T),
    Error(String),
}