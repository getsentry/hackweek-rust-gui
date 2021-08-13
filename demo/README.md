# Hackweek Crash Reporter

## Us

- Sebastian Zivota
- Jesse Box
- Arpad Borsos

## Goals

- Research the state of Rust-native GUI
  - No platform-native toolkits,
  - Rather frameworks that would allow to _build_ platform-native toolkits
- Build a Crash Reporter GUI
- Also take a look at localizations, etc
- non-goal: Build a polished product that customers ship with their apps

## What we did

- Started with hello-world examples of:
  - [druid](https://github.com/linebender/druid)
  - [sixtyfps](https://github.com/sixtyfpsui/sixtyfps)
  - [iced](https://github.com/hecrj/iced)
- Started implementing our UI designs in `druid`, but hit styling roadblocks
- Continued with `sixtyfps`

## Conclusion

- Rust-native GUI toolkits are very immature, as expected
- Still very much a R&D target
- Choices might be very opinionated
- Builtin Widgets either lack stylability, or functionality
  - eg. multiline textboxes with correct key combos, scrollboxes, tab order, etc

## Demo

You can run the crash reporter like so:

    cargo run -p sentry-sixty -- demo/config.json demo/4852baed-b317-47ca-c867-84b97a0db31f.envelope

Since the envelope has a unique event(\_id) inside, that will only work once though.
