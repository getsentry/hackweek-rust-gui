fn main() {
    MainWindow::new().run();
}

sixtyfps::sixtyfps! {
    MainWindow := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}
