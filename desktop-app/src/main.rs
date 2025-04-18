mod canvas;
use iced::window::{self, Settings};

use crate::canvas::Canvas;

fn main() -> iced::Result {
  iced::application(Canvas::title, Canvas::update, Canvas::view)
    .window(Settings {
      ..Default::default()
    })
    .run()
}
