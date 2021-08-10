use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::DesktopLanguageRequester;
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../resources/l18n"] // path to the compiled localization resources
struct Localizations;

sixtyfps::include_modules!();

fn main() {
    let main_window = MainWindow::new();

    localize(&main_window);
    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.run();
}

fn localize(main_window: &MainWindow) {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    let requested_languages = DesktopLanguageRequester::requested_languages();
    let _result = i18n_embed::select(&loader, &Localizations, &requested_languages);

    main_window.set_header_title(fl!(loader, "app-header-title", appname = "TODO").into());
    main_window.set_header_subtitle(fl!(loader, "app-header-subtitle", appname = "TODO").into());
    main_window.set_body_comment_label_text(fl!(loader, "form-comment-label").into());
    main_window.set_body_comment_placeholder(fl!(loader, "form-comment-placeholder").into());
    main_window.set_footer_show_text(fl!(loader, "app-footer-toggle-show-files").into());
    main_window.set_footer_close_text(fl!(loader, "app-footer-close").into());
    main_window.set_footer_submit_text(fl!(loader, "app-footer-submit").into());
}
