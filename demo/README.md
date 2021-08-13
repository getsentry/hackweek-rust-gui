You can run the crash reporter like so:

    cargo run -p sentry-sixty -- demo/config.json demo/4852baed-b317-47ca-c867-84b97a0db31f.envelope

Since the envelope has a unique event(\_id) inside, that will only work once though.
