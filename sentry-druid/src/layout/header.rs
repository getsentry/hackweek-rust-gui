use druid::widget::{CrossAxisAlignment, Flex, Label};
use druid::{LocalizedString, Widget, WidgetExt};

use crate::AppState;

pub fn header() -> impl Widget<AppState> {
    let title_text = LocalizedString::new("app-header-title")
        .with_arg("app-name", |_data: &AppState, _env| "TODO".into());
    let title_label = Label::new(title_text);

    let subtitle_text = LocalizedString::new("app-header-subtitle")
        .with_arg("app-name", |_data: &AppState, _env| "TODO".into());
    let subtitle_label = Label::new(subtitle_text);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(title_label)
        .with_child(subtitle_label)
        .expand_width()
}
