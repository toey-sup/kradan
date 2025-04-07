use iced::widget::{button, row, text, Row};

#[derive(Default)]
pub struct Counter {
  value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Increment,
  Decrement,
}

impl Counter {
  pub fn view(&self) -> Row<Message> {
    row![
      button("+").on_press(Message::Increment),
      text(self.value).size(50),
      button("-").on_press(Message::Decrement),
    ]
  }
  pub fn update(&mut self, message: Message) {
    match message {
      Message::Increment => {
        self.value += 1;
      }
      Message::Decrement => {
        self.value -= 1;
      }
    }
  }
}
