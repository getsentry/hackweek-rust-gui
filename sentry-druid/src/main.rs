use druid::commands::QUIT_APP;
use druid::widget::{Button, CrossAxisAlignment, Either, Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, Env, EventCtx, Lens, LocalizedString, PlatformError, Widget, WidgetExt,
    WindowDesc,
};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(main_window());
    AppLauncher::with_window(main_window)
        .localization_resources(vec!["crashreporter.ftl".into()], "./resources/l18n".into())
        .launch(AppState::default())
}

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    files_open: bool,
    comment: String,
}

fn main_window() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(app_header())
        .with_child(app_form())
        .with_child(app_footer())
}

fn app_form() -> impl Widget<AppState> {
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

fn app_header() -> impl Widget<AppState> {
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

fn app_footer() -> impl Widget<AppState> {
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

fn toggle_files(_event: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.files_open = !data.files_open;
}
