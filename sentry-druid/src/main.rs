use druid::widget::{Button, Flex, Label, TextBox, ViewSwitcher};
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
}

fn main_window() -> impl Widget<AppState> {
    Flex::column()
        .with_child(app_header())
        .with_child(app_footer())
}

fn app_header() -> impl Widget<AppState> {
    let title_text = LocalizedString::new("app-header-title")
        .with_arg("app-name", |_data: &AppState, _env| "TODO".into());
    let title_label = Label::new(title_text);

    let subtitle_text = LocalizedString::new("app-header-subtitle")
        .with_arg("app-name", |_data: &AppState, _env| "TODO".into());
    let subtitle_label = Label::new(subtitle_text);

    Flex::column()
        .with_child(title_label)
        .with_child(subtitle_label)
}

fn app_footer() -> impl Widget<AppState> {
    // TODO: is there a simpler way to achieve this?
    // It seems a bit excessive to allocate a new box each time I want to change
    // the label
    let toggle_files = ViewSwitcher::new(
        |data: &AppState, _env| data.files_open,
        |is_open, _data, _env| {
            if *is_open {
                Box::new(
                    Button::new(LocalizedString::new("app-footer-toggle-show-files"))
                        .on_click(toggle_files),
                )
            } else {
                Box::new(
                    Button::new(LocalizedString::new("app-footer-toggle-hide-files"))
                        .on_click(toggle_files),
                )
            }
        },
    );

    let close_text = LocalizedString::new("app-footer-close");
    let close = Button::new(close_text);

    let submit_text = LocalizedString::new("app-footer-submit");
    let submit = Button::new(submit_text);

    Flex::row()
        .with_child(toggle_files)
        .with_default_spacer()
        .with_child(close)
        .with_child(submit)
}

fn toggle_files(_event: &mut EventCtx, data: &mut AppState, _env: &Env) {
    data.files_open = !data.files_open;
}
