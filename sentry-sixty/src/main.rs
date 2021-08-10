use fluent::FluentArgs;

use crate::translations::TranslationContext;

sixtyfps::include_modules!();

mod translations;

fn main() {
    let main_window = MainWindow::new();

    let locale = druid_shell::Application::get_locale().parse().unwrap();
    let translations = TranslationContext::new(locale);

    let mut args = FluentArgs::new();
    args.set("app-name", "TODO");
    main_window.set_header_title(
        translations
            .translate("app-header-title", Some(&args))
            .into(),
    );
    main_window.set_header_subtitle(
        translations
            .translate("app-header-subtitle", Some(&args))
            .into(),
    );
    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.run();
}
