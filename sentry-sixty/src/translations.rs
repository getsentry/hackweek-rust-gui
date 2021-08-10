use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use unic_langid::{langid, LanguageIdentifier};

const LANGUAGES: &[(LanguageIdentifier, &str)] = &[
    (
        langid!("en-US"),
        include_str!("../../resources/l18n/en-US/crashreporter.ftl"),
    ),
    (
        langid!("de-DE"),
        include_str!("../../resources/l18n/de-DE/crashreporter.ftl"),
    ),
];

pub struct TranslationContext {
    bundle: FluentBundle<FluentResource, intl_memoizer::IntlLangMemoizer>,
}

impl TranslationContext {
    pub fn new(requested_lang: LanguageIdentifier) -> Self {
        let fallback_language = langid!("en-US");
        let available_languages: Vec<_> = LANGUAGES.iter().map(|(id, _)| id.clone()).collect();
        let current_language = negotiate_languages(
            &[requested_lang],
            &available_languages,
            Some(&fallback_language),
            NegotiationStrategy::Matching,
        )
        .into_iter()
        .next()
        .unwrap();

        let resource = LANGUAGES
            .iter()
            .find(|(id, _)| id == current_language)
            .unwrap();
        let resource =
            FluentResource::try_new(resource.1.to_owned()).expect("Could not parse an FTL string.");

        let mut bundle = FluentBundle::new(vec![current_language.to_owned()]);

        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resources to the bundle.");

        Self { bundle }
    }

    pub fn translate(&self, message_id: &str, args: Option<&FluentArgs>) -> String {
        let msg = self
            .bundle
            .get_message(message_id)
            .expect("Message doesn't exist.");

        let mut errors = vec![];

        let pattern = msg.value().expect("Message has no value.");

        self.bundle
            .format_pattern(pattern, args, &mut errors)
            .to_string()
    }
}
