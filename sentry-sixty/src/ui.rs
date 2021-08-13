use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use sentry_core::types::Uuid;
use sentry_uploader::Envelope as RawEnvelope;
use sentry_uploader::{Uploader, UserFeedback};
use sixtyfps::{ModelHandle, VecModel};

use crate::config::Branding;
use crate::localizations::localize;

sixtyfps::include_modules!();

#[derive(Debug, Clone)]
enum UiState {
    SubmitEnvelope { raw_envelope: RawEnvelope },
    SubmitFeedback { event_id: Uuid },
}

// TODO: make it possible to use this as a library, and call the functions
// starting the UI from crashpad/your own app/some kind of wrapper.
pub fn start_crash_reporter_window(
    branding: Branding,
    uploader: Uploader,
    raw_envelope: RawEnvelope,
) {
    let uploader = Arc::new(uploader);

    // we move one ref into the submit callback (for now, possibly more callbacks soon)
    let uploader = Arc::clone(&uploader);

    let attachments = raw_envelope
        .items()
        .map(|item| Attachment {
            name: item.file_name().into(),
            contents: "Foo bar".into(),
        })
        .collect::<Vec<_>>();

    let ui_state = Arc::new(Mutex::new(UiState::SubmitEnvelope { raw_envelope }));

    let main_window = MainWindow::new();
    let main_window_weak = main_window.as_weak();
    localize(&main_window, &branding);

    main_window.set_attachments(ModelHandle::new(Rc::new(VecModel::from(attachments))));

    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.on_submit_clicked(move || {
        // make borrow checker happy
        let uploader = uploader.as_ref();
        let mut ui_state = ui_state.lock().unwrap();
        match *ui_state {
            UiState::SubmitEnvelope {
                ref mut raw_envelope,
            } => {
                let raw_envelope = std::mem::take(raw_envelope);
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

// TODO: maybe these callbacks/controllers should live outside the actual UI?
fn submit_envelope(uploader: &Uploader, raw_envelope: RawEnvelope) -> anyhow::Result<Uuid> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        uploader
            .send_envelope(raw_envelope.into_bytes().to_vec())
            .await
    })
}

fn submit_feedback(
    uploader: &Uploader,
    event_id: Uuid,
    feedback: &UserFeedback,
) -> anyhow::Result<bool> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move { uploader.send_user_feedback(event_id, feedback).await })
}
