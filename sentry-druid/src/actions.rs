use druid::{Env, EventCtx};

use crate::state::AppState;

pub fn toggle_files(_event: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.files_open = !data.files_open;
}
