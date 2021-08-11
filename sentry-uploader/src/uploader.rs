use sentry_core::types::Uuid;
use sentry_core::Envelope;

use crate::{Config, UserFeedback};

#[derive(Clone, Debug)]
pub struct Uploader {
    client: reqwest::Client,
    sentry_auth: String,
    dsn_auth: String,
    envelope_url: String,
    user_feedback_url: String,
}

impl Uploader {
    /// Creates a new Uploader given a [`Config`].
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::default(); // TODO: maybe allow configuring proxy, etc

        let auth = config.dsn.to_auth(None); // TODO: provide user agent?
        let sentry_auth = auth.to_string();
        let dsn_auth = format!("DSN {}", config.dsn);

        let envelope_url = config.dsn.envelope_api_url().to_string();
        let user_feedback_url = format!(
            "https://sentry.io/api/0/projects/{}/{}/user-feedback/",
            // TODO: infer the API url from the dsn
            //config.dsn.scheme(),
            //config.dsn.host(),
            //config.dsn.port(),
            config.org_slug,
            config.project_slug
        );

        Self {
            client,
            sentry_auth,
            dsn_auth,
            envelope_url,
            user_feedback_url,
        }
    }

    /// Send the given envelope to the configured DSN.
    pub async fn send_envelope(&self, envelope: &Envelope) -> anyhow::Result<()> {
        // TODO: maybe not worth it, but can we actually write the envelope onto the body stream directly, instead
        // of first allocating a byte vector ahead of time?
        // TODO: can we gzip the body?
        let mut body = Vec::new();
        envelope.to_writer(&mut body).unwrap();
        let request = self
            .client
            .post(&self.envelope_url)
            .header("X-Sentry-Auth", &self.sentry_auth)
            .body(body);

        let response = request.send().await?;

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            anyhow::bail!("DSN quota exceeded");
        }
        let _json = response.text().await?;

        // TODO: return the event id
        // TODO: is it possible to somehow feed the upload progress back to the UI?
        Ok(())
    }

    /// Submit a User Feedback report
    ///
    /// See https://docs.sentry.io/api/projects/submit-user-feedback/
    pub async fn send_user_feedback(
        &self,
        event_id: Uuid,
        user_feedback: &UserFeedback,
    ) -> anyhow::Result<()> {
        let request = self
            .client
            .post(&self.user_feedback_url)
            .header(reqwest::header::AUTHORIZATION, &self.dsn_auth)
            .json(&serde_json::json!({
                "event_id": event_id.to_simple().to_string(),
                // NOTE: name/email are marked as "required" in the docs, but technically
                // you can leave them out. We would have to try and see if it works without.
                // See https://github.com/getsentry/relay/blob/master/relay-general/src/protocol/user_report.rs
                "name": user_feedback.name,
                "email": user_feedback.email,
                "comments": user_feedback.comment,
            }));

        let _json = request.send().await?.text().await?;

        // TODO: should we validate that the report has actually been submitted?
        Ok(())
    }
}
