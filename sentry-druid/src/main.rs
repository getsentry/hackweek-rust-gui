use druid::{AppLauncher, PlatformError, WindowDesc};

use crate::state::AppState;

mod actions;
mod layout;
mod state;
mod theme;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(layout::main_window());
    AppLauncher::with_window(main_window)
        .localization_resources(vec!["crashreporter.ftl".into()], "./resources/l18n".into())
        .configure_env(theme::configure)
        .launch(AppState::default())
}
