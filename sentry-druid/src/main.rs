use druid::widget::{Button, Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc,
};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window)
        .localization_resources(vec!["crashreporter.ftl".into()], "./resources/l18n".into())
        .launch(HelloState::default())
}

#[derive(Clone, Default, Data, Lens)]
struct HelloState {
    counter: u32,
    name: String,
}

fn ui_builder() -> impl Widget<HelloState> {
    // The label text will be computed dynamically based on the current locale and count
    let text = LocalizedString::new("foo-bar")
        .with_arg("count", |data: &HelloState, _env| (*data).counter.into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut HelloState, _env| data.counter += 1)
        .padding(5.0);
    let textbox = TextBox::new()
        .with_placeholder("some placeholder")
        .lens(HelloState::name);

    Flex::column()
        .with_child(label)
        .with_child(button)
        .with_child(textbox)
}
