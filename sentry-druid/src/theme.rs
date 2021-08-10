use druid::{Color, Env, Insets, Key};

use crate::state::AppState;

pub const TEXT_PRIMARY: Key<Color> = Key::new("sentry.text-primary");
pub const TEXT_SECONDARY: Key<Color> = Key::new("sentry.text-secondary");
pub const SECTION_PADDING: Key<Insets> = Key::new("sentry.section-padding");

pub const BORDER_COLOR: Key<Color> = Key::new("sentry.background-color");

pub fn configure(env: &mut Env, _data: &AppState) {
    env.set(TEXT_PRIMARY, Color::from_rgba32_u32(0x2B1D38FF));
    env.set(TEXT_SECONDARY, Color::from_rgba32_u32(0x776589FF));

    env.set(SECTION_PADDING, Insets::uniform(32.));

    env.set(BORDER_COLOR, Color::from_rgba32_u32(0xD1CAD8FF));

    // druid default theme
    use druid::theme::*;
    env.set(WINDOW_BACKGROUND_COLOR, Color::WHITE);
    env.set(TEXT_COLOR, Color::from_rgba32_u32(0x2B1D38FF));
}
