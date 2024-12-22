use iced::widget::{button, column, text, Column};

fn main() -> Result<(), iced::Error> {
  iced::run("A cool counter", Counter::update, Counter::view)
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
      button("-").on_press(Message::Decrement)
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
