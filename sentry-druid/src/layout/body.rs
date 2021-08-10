use druid::widget::{CrossAxisAlignment, Flex, Label, TextBox};
use druid::{LocalizedString, Widget, WidgetExt};

use crate::AppState;

pub fn body() -> impl Widget<AppState> {
    let comment_label_text = LocalizedString::new("form-comment-label");
    let comment_label = Label::new(comment_label_text);

    // TODO: not sure how to make this work yet, the textbox does not support
    // newlines on enter, it does not enter text when you hold a key down, and it
    // does not support select-all via cmd/ctrl+a
    let comment_textbox = TextBox::multiline()
        // TODO: the `with_placeholder` method does not work with localized strings :-(
        //.with_placeholder(LocalizedString::new("form-comment-placeholder"))
        .with_line_wrapping(true)
        .lens(AppState::comment)
        .expand_width()
        .fix_height(200.0);

    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child(comment_label)
        .with_flex_child(comment_textbox, 1.0)
        .expand_width()
}
