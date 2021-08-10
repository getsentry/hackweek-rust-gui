use druid::widget::{CrossAxisAlignment, Flex, Label};
use druid::{LocalizedString, Widget, WidgetExt};

use crate::{theme, AppState};

pub fn header() -> impl Widget<AppState> {
    let title_text = LocalizedString::new("app-header-title")
        .with_arg("app-name", |_data: &AppState, _env| "TODO".into());
    let title_label = Label::new(title_text)
        .with_text_size(22.)
        .with_text_color(theme::TEXT_PRIMARY)
        .fix_height(28.);

    let subtitle_text = LocalizedString::new("app-header-subtitle")
        .with_arg("app-name", |_data: &AppState, _env| "TODO".into());
    let subtitle_label = Label::new(subtitle_text)
        .with_text_size(14.)
        .with_text_color(theme::TEXT_SECONDARY)
        .fix_height(21.);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(title_label)
        .with_spacer(4.)
        .with_child(subtitle_label)
        .expand_width()
        .padding(theme::SECTION_PADDING)
    //.border(theme::BORDER_COLOR, 1.0)
}
