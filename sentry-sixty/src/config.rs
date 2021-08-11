use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Config {
    pub branding: Branding,
    pub sentry: Sentry,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Branding {
    pub app_name: String,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Sentry {
    // TODO: well the dsn *can* be in the envelope header, maybe use that and make
    // the config into an option? but if there is none, showing the crash reporter is useless.
    pub dsn: String,

    // TODO: these two could also be an option, in which case no user feedback components should be shown
    pub org_slug: String,
    pub project_slug: String,
}
