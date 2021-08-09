use iced::{executor, Application, Clipboard, Command, Element, Settings, Text};

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hello World!")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("hello world!").into()
    }
}

fn main() -> iced::Result {
    Hello::run(Settings::default())
}
