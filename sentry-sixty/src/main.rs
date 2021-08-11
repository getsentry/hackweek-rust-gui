use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::DesktopLanguageRequester;
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../resources/l18n"] // path to the compiled localization resources
struct Localizations;

sixtyfps::include_modules!();

pub mod config;

fn make_mock_envelope() -> sentry_core::Envelope {
    let mut event = sentry_core::protocol::Event::new();
    event.message = Some(String::from("Wow, something bad happened"));
    let mut envelope: sentry_core::Envelope = event.into();
    envelope.add_item(sentry_core::protocol::EnvelopeItem::Attachment(
        sentry_core::protocol::Attachment {
            buffer: include_bytes!("main.rs").to_vec(),
            filename: String::from("main.rs"),
            ty: None,
        },
    ));

    envelope
}

// TODO: make it possible to use this as a library, and call the functions
// starting the UI from crashpad/your own app/some kind of wrapper.
pub fn start_crash_reporter_windows(config: config::Config) {
    let main_window = MainWindow::new();

    let dsn = config.sentry.dsn.parse().unwrap();

    let uploader_config = sentry_uploader::Config::new(
        dsn,
        config.sentry.org_slug.clone(),
        config.sentry.project_slug.clone(),
    );
    let uploader = sentry_uploader::Uploader::new(uploader_config);

    let envelope = make_mock_envelope();

    localize(&main_window, &config.branding);
    main_window.on_close_clicked(sixtyfps::quit_event_loop);
    main_window.on_submit_clicked(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        // make borrow checker happy
        let envelope = &envelope;
        let uploader = &uploader;
        rt.block_on(async move {
            uploader.send_envelope(envelope).await?;

            let event_id = envelope.uuid().unwrap().to_owned();
            let mut user_feedback = sentry_uploader::UserFeedback::default();
            // TODO: take the values from the UI, and only send user feedback if all
            // the required fields are provided
            user_feedback.name = String::from("Bender");
            user_feedback.email = String::from("bender@planetexpress.earth");
            user_feedback.comment = String::from("oh noes! its a me, user feedbackaaaa!");

            uploader
                .send_user_feedback(event_id, &user_feedback)
                .await?;

            Ok::<_, Box<dyn std::error::Error>>(())
        })
        .unwrap();
        sixtyfps::quit_event_loop();
    });

    main_window.run();
}

fn main() {
    start_crash_reporter_windows(config::Config {
        branding: config::Branding {
            app_name: String::from("Sentry"),
        },
        sentry: config::Sentry {
            dsn: String::from(
                "https://afb46d25eecb4cdb8653f859aa980181@o19635.ingest.sentry.io/1041156",
            ),
            org_slug: String::from("sentry-test"),
            project_slug: String::from("rust"),
        },
    });
}

fn localize(main_window: &MainWindow, branding: &config::Branding) {
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
