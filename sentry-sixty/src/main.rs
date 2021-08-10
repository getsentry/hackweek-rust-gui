sixtyfps::include_modules!();

fn main() {
    let main_window = MainWindow::new();

    main_window.set_header_title("oh hai".into());
    main_window.set_header_subtitle("subtitle!".into());
    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.run();
}
