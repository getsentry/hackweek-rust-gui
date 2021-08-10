use druid::{Data, Lens};

#[derive(Clone, Default, Data, Lens)]
pub struct AppState {
    pub files_open: bool,
    pub comment: String,
}
