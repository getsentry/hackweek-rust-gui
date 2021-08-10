use druid::widget::Flex;
use druid::Widget;

use crate::AppState;

mod body;
mod footer;
mod header;

pub fn main_window() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(header::header())
        .with_child(body::body())
        .with_child(footer::footer())
}
