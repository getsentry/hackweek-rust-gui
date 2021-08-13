use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::DesktopLanguageRequester;
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

use crate::config::Branding;
use crate::ui::MainWindow;

#[derive(RustEmbed)]
#[folder = "../resources/l18n"] // path to the compiled localization resources
struct Localizations;

pub fn localize(main_window: &MainWindow, branding: &Branding) {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    let requested_languages = DesktopLanguageRequester::requested_languages();
    let _result = i18n_embed::select(&loader, &Localizations, &requested_languages);

    main_window.set_header_title(
        fl!(
            loader,
            "app-header-title",
            appname = branding.app_name.clone()
        )
        .into(),
    );
    main_window.set_header_subtitle(
        fl!(
            loader,
            "app-header-subtitle",
            appname = branding.app_name.clone()
        )
        .into(),
    );
    main_window.set_body_comment_label_text(fl!(loader, "form-comment-label").into());
    main_window.set_body_comment_placeholder(fl!(loader, "form-comment-placeholder").into());
    main_window.set_body_files_label_text(fl!(loader, "app-body-files-label").into());
    main_window.set_body_show_button_text(fl!(loader, "app-body-toggle-show-file").into());
    main_window.set_body_hide_button_text(fl!(loader, "app-body-toggle-hide-file").into());
    main_window.set_footer_show_text(fl!(loader, "app-footer-toggle-show-files").into());
    main_window.set_footer_close_text(fl!(loader, "app-footer-close").into());
    main_window.set_footer_submit_text(fl!(loader, "app-footer-submit").into());
}
