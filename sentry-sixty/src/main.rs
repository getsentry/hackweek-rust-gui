sixtyfps::include_modules!();

fn main() {
    let main_window = MainWindow::new();

    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.run();
}
