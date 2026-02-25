use crate::repositories::Registry;

#[derive(Clone)]
pub struct AppState {
    pub repo: Registry
}