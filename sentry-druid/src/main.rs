use druid::{AppLauncher, PlatformError, WindowDesc};

use crate::state::AppState;

mod actions;
mod layout;
mod state;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(layout::main_window());
    AppLauncher::with_window(main_window)
        .localization_resources(vec!["crashreporter.ftl".into()], "./resources/l18n".into())
        .launch(AppState::default())
}
