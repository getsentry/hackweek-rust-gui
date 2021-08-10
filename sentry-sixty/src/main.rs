fn main() {
    let main_window = MainWindow::new();

    main_window.on_close_clicked(sixtyfps::quit_event_loop);

    main_window.run();
}

sixtyfps::sixtyfps! {
    global Theme := {
        property<color> outline_default: #D1CAD8;
        property<color> outline_hover: #80708F;
        property<color> outline_active: #E6E0EB;
        property<color> accent_default: #6C5FC7;
        property<color> accent_hover: #6051C2;
        property<color> accent_active: #7C70CD;
    }

    SentryNormalButton := Rectangle {
        callback clicked;
        property<string> text;
        property<bool> pressed: self.enabled && touch_area.pressed;
        property<bool> enabled <=> touch_area.enabled;

        border_width: 1px;
        border_radius: 4px;
        border_color: self.pressed ? Theme.outline_active : (touch_area.has_hover ? Theme.outline_hover : Theme.outline_default);
        background: white;

        animate background { duration: 100ms; }
        horizontal-stretch: 0;
        vertical-stretch: 0;

        HorizontalLayout {
            padding-top: 12px;
            padding-bottom: 12px;
            padding-left: 16px;
            padding-right: 16px;

            Text {
                text: root.text;
                font-weight: 600;
                font-size: 14px;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }

        touch_area := TouchArea {
            width: root.width;
            height: root.height;
            clicked => {
                root.clicked();
            }
        }
    }


    SentryAccentButton := Rectangle {
        callback clicked;
        property<string> text;
        property<bool> pressed: self.enabled && touch_area.pressed;
        property<bool> enabled <=> touch_area.enabled;

        border_width: 1px;
        border_radius: 4px;
        border_color: self.pressed ? Theme.outline_active : (touch_area.has_hover ? Theme.outline_hover : Theme.outline_default);
        background: self.pressed ? Theme.accent_active : (touch_area.has_hover ? Theme.accent_hover : Theme.accent_default);

        animate background { duration: 100ms; }
        horizontal-stretch: 0;
        vertical-stretch: 0;

        HorizontalLayout {
            padding-top: 12px;
            padding-bottom: 12px;
            padding-left: 16px;
            padding-right: 16px;

            Text {
                text: root.text;
                font-weight: 600;
                font-size: 14px;
                color: white;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }

        touch_area := TouchArea {
            width: root.width;
            height: root.height;
            clicked => {
                root.clicked();
            }
        }
    }

    Header := VerticalLayout {
        vertical_stretch: 0;
        padding: 32px;
        spacing: 4px;

        Text {
            text: "{app-title} has Crashed.";
            font_size: 22px;
            color: #2B1D38;
        }

        Text {
            text: "This report will be sent to {app-title} on submission";
            font_size: 14px;
            color: #776589;
        }
    }

    Body := VerticalLayout {
        vertical_stretch: 1;

        Rectangle {
            vertical_stretch: 1;
        }
    }

    Footer := HorizontalLayout {
        callback close_clicked;
        vertical_stretch: 0;
        padding: 32px;
        spacing: 8px;

        SentryNormalButton {
            text: "Show Files";
        }

        Rectangle {
            horizontal_stretch: 1;
        }

        SentryNormalButton {
            clicked => { root.close_clicked(); }
            text: "Close";
        }

        SentryAccentButton {
            text: "Submit";
        }
    }

    MainWindow := Window {
        callback close_clicked;

        background: white;

        VerticalLayout {
            Header {}
            Body {}
            Footer {
                close_clicked => { root.close_clicked(); }
            }
        }
    }
}
