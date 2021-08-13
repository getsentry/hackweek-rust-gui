use sentry_core::protocol::{EnvelopeItem, Event};
use sentry_core::Envelope;
use sentry_uploader::{Envelope as RawEnvelope, Uploader};

mod config;
mod localizations;
mod ui;

fn make_mock_envelope() -> Vec<u8> {
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

    let mut raw_envelope = Vec::new();
    envelope.to_writer(&mut raw_envelope).unwrap();
    raw_envelope
}

fn mock_config() -> config::Config {
    config::Config {
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
    }
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next(); // self

    let config = args
        .next()
        .and_then(|path| {
            let file = std::fs::File::open(path).ok()?;
            let config: config::Config = serde_json::from_reader(file).ok()?;
            Some(config)
        })
        .unwrap_or_else(mock_config);

    let raw_envelope = args
        .next()
        .and_then(|path| std::fs::read(path).ok())
        .unwrap_or_else(make_mock_envelope);
    let raw_envelope = RawEnvelope::parse(raw_envelope);

    let dsn = config.sentry.dsn.parse().unwrap();

    let uploader_config = sentry_uploader::Config::new(
        dsn,
        config.sentry.org_slug.clone(),
        config.sentry.project_slug.clone(),
    );
    let uploader = Uploader::new(uploader_config);

    ui::start_crash_reporter_window(config.branding, uploader, raw_envelope);

    Ok(())
}
