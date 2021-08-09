use sentry_core::types::Dsn;

// TODO: provide a method to read the envelope file from disc
// TODO: should we have another method to upload minidump/messagepack event/breadcrumb
// files as they are being generated by crashpad at the moment?
// We could even add a proper parser that just converts these messagepack files to an envelope.

mod uploader;

pub use uploader::Uploader;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UserFeedback {
    pub name: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
// NOTE: The DSN can be extracted from each envelope since its an optional header property.
// However the org/project slugs are not, so they would need to be provided explicitly.
pub struct Config {
    pub dsn: Dsn,
    pub org_slug: String,
    pub project_slug: String,
}