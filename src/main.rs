use iced::{
  theme,
  widget::{button, column, text, Column},
  Theme,
};

fn main() -> Result<(), iced::Error> {
  iced::application("A cool counter", Counter::update, Counter::view)
    .theme(|_| Theme::Dark)
    .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
  Increment,
  Decrement,
}

#[derive(Default)]
struct Counter {
  value: i64,
}

impl Counter {
  fn update(&mut self, message: Message) {
    match message {
      Message::Increment => {
        self.value += 1;
      }
      Message::Decrement => {
        self.value -= 1;
      }
    }
  }

  fn view(&self) -> Column<Message> {
    column![
      button("+").on_press(Message::Increment),
      text(self.value),
      button("-")
        .on_press(Message::Decrement)
        .style(|theme: &Theme, status: button::Status| button::danger(theme, status))
    ]
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_counts_properly() {
    use super::*;

    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
  }
}
