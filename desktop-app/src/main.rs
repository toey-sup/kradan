mod counter;
use crate::counter::Counter;

fn main() -> iced::Result {
  iced::run("A cool counter", Counter::update, Counter::view)
}
