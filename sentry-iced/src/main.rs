use iced::{button, Align, Button, Column, Element, Length, Sandbox, Settings, Text};

struct Hello {
    counter: usize,
    button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Inc,
}

impl Sandbox for Hello {
    type Message = Message;

    fn new() -> Self {
        Self {
            counter: 0,
            button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Hello World!")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Inc => self.counter += 1,
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("hello world!"))
            .push(Text::new(self.counter.to_string()))
            .push(Button::new(&mut self.button, Text::new("+")).on_press(Message::Inc))
            .align_items(Align::Center)
            .width(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    Hello::run(Settings::default())
}
