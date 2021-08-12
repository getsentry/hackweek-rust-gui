use std::sync::Arc;
use std::sync::Mutex;

use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::DesktopLanguageRequester;
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;
use sentry_core::protocol::{EnvelopeItem, Event};
use sentry_core::types::Uuid;
use sentry_core::Envelope;
use sentry_uploader::{Uploader, UserFeedback};

#[derive(RustEmbed)]
#[folder = "../resources/l18n"] // path to the compiled localization resources
struct Localizations;

sixtyfps::include_modules!();

use sentry_uploader::Envelope as RawEnvelope;

pub mod config;

#[derive(Debug, Clone)]
enum UiState {
    SubmitEnvelope { raw_envelope: RawEnvelope },
    SubmitFeedback { event_id: Uuid },
}

fn make_mock_envelope() -> Envelope {
    let mut event = Event::new();
    event.message = Some(String::from("Wow, something bad happened"));
    let mut envelope: Envelope = event.into();
    envelope.add_item(EnvelopeItem::Attachment(
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
    let dsn = config.sentry.dsn.parse().unwrap();

    let uploader_config = sentry_uploader::Config::new(
        dsn,
        config.sentry.org_slug.clone(),
        config.sentry.project_slug.clone(),
    );
    let uploader = Arc::new(Uploader::new(uploader_config));
    let envelope = Arc::new(make_mock_envelope());

    // we move one ref into the submit callback (for now, possibly more callbacks soon)
    let uploader = Arc::clone(&uploader);
    let envelope = Arc::clone(&envelope);

    let mut raw_envelope = Vec::new();
    envelope.to_writer(&mut raw_envelope).unwrap();
    let raw_envelope = RawEnvelope::parse(raw_envelope);

    let ui_state = Arc::new(Mutex::new(UiState::SubmitEnvelope { raw_envelope }));

    let main_window = MainWindow::new();
    let main_window_weak = main_window.as_weak();
    localize(&main_window, &config.branding);

    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.on_submit_clicked(move || {
        // make borrow checker happy
        let uploader = uploader.as_ref();
        let mut ui_state = ui_state.lock().unwrap();
        match *ui_state {
            UiState::SubmitEnvelope {
                ref mut raw_envelope,
            } => {
                let raw_envelope = std::mem::take(raw_envelope).into_inner();
                let event_id = submit_envelope(uploader, raw_envelope).unwrap();
                *ui_state = UiState::SubmitFeedback { event_id };
                main_window_weak.unwrap().set_step(2);
            }
            UiState::SubmitFeedback { event_id } => {
                let mut user_feedback = sentry_uploader::UserFeedback::default();
                // TODO: take the values from the UI, and only send user feedback if all
                // the required fields are provided
                user_feedback.name = String::from("Bender");
                user_feedback.email = String::from("bender@planetexpress.earth");
                user_feedback.comment = String::from("oh noes! its a me, user feedbackaaaa!");

                submit_feedback(uploader, event_id, &user_feedback).unwrap();

                sixtyfps::quit_event_loop()
            }
        }
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

fn submit_envelope(uploader: &Uploader, raw_envelope: Vec<u8>) -> anyhow::Result<Uuid> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move { uploader.send_envelope(raw_envelope).await })
}

fn submit_feedback(
    uploader: &Uploader,
    event_id: Uuid,
    feedback: &UserFeedback,
) -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move { uploader.send_user_feedback(event_id, feedback).await })?;
    Ok(())
}
