use druid::commands::QUIT_APP;
use druid::widget::{Button, Either, Flex};
use druid::{LocalizedString, Widget, WidgetExt};

use crate::actions::toggle_files;
use crate::AppState;

pub fn footer() -> impl Widget<AppState> {
    let toggle_files = Either::new(
        |data: &AppState, _env| data.files_open,
        Button::new(LocalizedString::new("app-footer-toggle-hide-files")).on_click(toggle_files),
        Button::new(LocalizedString::new("app-footer-toggle-show-files")).on_click(toggle_files),
    );

    let close_text = LocalizedString::new("app-footer-close");
    let close = Button::new(close_text).on_click(|ev, _data, _env| ev.submit_command(QUIT_APP));

    let submit_text = LocalizedString::new("app-footer-submit");
    let submit = Button::new(submit_text);

    Flex::row()
        .must_fill_main_axis(true)
        .with_child(toggle_files)
        .with_flex_spacer(1.0)
        .with_child(close)
        .with_child(submit)
        .expand_width()
}
