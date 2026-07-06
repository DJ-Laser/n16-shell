use std::{collections::HashMap, time::Duration};

use iced::{
  Element, Length, Subscription, Task, gradient,
  keyboard::key,
  widget::{column, container, operation, rule, scrollable},
};
use n16_core::theme::{self, Base16Theme};

use crate::launcher::{
  Providers,
  component::{
    provider_section,
    search::{self, SEARCH_INPUT_ID},
  },
  providers::{ExecutionFinishAction, Match, Matches, ProviderId, ProviderInfo, ProviderType},
};

pub struct Launcher {
  query: String,
  selected_idx: (usize, usize),

  providers: Providers,
  provider_info: Vec<ProviderInfo>,
  matches: HashMap<ProviderId, Vec<Match>>,
}

#[derive(Debug)]
pub enum Action {
  Task(Task<Message>),
  Close,
}

#[derive(Debug, Clone)]
pub enum Message {
  Close,
  FocusInput,
  SelectPrev,
  SelectNext,
  RunSelected,
  RunIdx((usize, usize)),
  SearchQueryChanged(String),
  UpdateStaticMatches(Matches),
  UpdateDynamicMatches(String, Matches),
  ProviderExecutionFinished(ExecutionFinishAction),
}

impl Launcher {
  pub fn new(mut providers: Providers) -> (Self, Task<Message>) {
    let provider_task =
      Task::stream(providers.get_static_matches()).map(Message::UpdateStaticMatches);

    (
      Self {
        query: String::new(),
        selected_idx: (0, 0),

        provider_info: providers.get_sorted_provider_info(),
        providers,
        matches: HashMap::new(),
      },
      Task::batch([
        provider_task,
        Task::future(async {
          tokio::time::sleep(Duration::from_millis(250)).await;
          Message::FocusInput
        }),
      ]),
    )
  }

  fn scroll_to_selected(&self) -> Task<Message> {
    Task::none() //operation::scroll_to(id, offset)
  }

  fn update_query(&mut self, new_query: &str) -> Task<Message> {
    self.query.clear();
    self.query.push_str(new_query);
    self.selected_idx = (0, 0);

    if !self.query.is_empty() {
      let query = self.query.clone();
      Task::stream(self.providers.get_dynamic_matches(query.clone()))
        .map(move |matches| Message::UpdateDynamicMatches(query.clone(), matches))
    } else {
      for info in self.provider_info.iter() {
        if matches!(info.provider_type, ProviderType::Dynamic) {
          self.matches.remove(&info.id);
        }
      }

      Task::none()
    }
  }

  fn get_num_matches(&self, id: &str) -> usize {
    self.matches.get(id).map_or(0, Vec::len)
  }

  fn get_prev_idx(&self, from_idx: (usize, usize)) -> (usize, usize) {
    if from_idx.1 != 0 {
      return (from_idx.0, from_idx.1 - 1);
    }

    let idx_0_len = self.provider_info.len();
    let mut curr_idx_0 = usize::min(from_idx.0, idx_0_len);

    for _ in 0..idx_0_len {
      if curr_idx_0 == 0 {
        curr_idx_0 = idx_0_len;
      }
      curr_idx_0 -= 1;

      match self.get_num_matches(&self.provider_info[curr_idx_0].id) {
        0 => continue,
        idx_1_len => return (curr_idx_0, idx_1_len - 1),
      }
    }

    (0, 0)
  }

  fn get_next_idx(&self, from_idx: (usize, usize)) -> (usize, usize) {
    let Some(info) = self.provider_info.get(from_idx.0) else {
      return (0, 0);
    };

    if from_idx.1 <= self.get_num_matches(&info.id) {
      return (from_idx.0, from_idx.1 + 1);
    }

    let idx_0_len = self.provider_info.len();
    let mut curr_idx_0 = from_idx.0;

    for _ in 0..idx_0_len {
      curr_idx_0 += 1;
      if curr_idx_0 >= idx_0_len {
        curr_idx_0 = 0;
      }

      match self.get_num_matches(&self.provider_info[curr_idx_0].id) {
        0 => continue,
        _ => return (curr_idx_0, 0),
      }
    }

    (0, 0)
  }

  fn get_match_at(&self, idx: (usize, usize)) -> Option<(&String, &Match)> {
    let id = &self.provider_info.get(idx.0)?.id;

    let matches = self.matches.get(id)?;
    if idx.1 < matches.len() {
      Some((id, &matches[idx.1]))
    } else {
      None
    }
  }

  fn filter_static_match(&self, static_match: &Match) -> bool {
    let trimmed: String = self
      .query
      .split_whitespace()
      .collect::<String>()
      .to_lowercase();

    static_match
      .title
      .split_whitespace()
      .collect::<String>()
      .to_lowercase()
      .contains(&trimmed)
      || static_match
        .keywords
        .iter()
        .any(|kw| trimmed.contains(&kw.to_lowercase()))
  }
}

impl Launcher {
  pub fn update(&mut self, message: Message) -> Action {
    let task = match message {
      Message::Close => return Action::Close,

      Message::RunSelected => {
        if let Some((id, selected_match)) = self.get_match_at(self.selected_idx) {
          let p = self.providers.clone();
          Task::future(p.execute_match((id.clone(), selected_match.clone())))
            .and_then(|a| Task::done(Message::ProviderExecutionFinished(a)))
        } else {
          Task::none()
        }
      }

      Message::RunIdx(idx) => {
        if let Some((id, selected_match)) = self.get_match_at(idx) {
          let p = self.providers.clone();
          Task::future(p.execute_match((id.clone(), selected_match.clone())))
            .and_then(|a| Task::done(Message::ProviderExecutionFinished(a)))
        } else {
          Task::none()
        }
      }

      Message::SearchQueryChanged(new_query) => self.update_query(&new_query),

      Message::SelectPrev => {
        self.selected_idx = self.get_prev_idx(self.selected_idx);
        self.scroll_to_selected()
      }

      Message::SelectNext => {
        self.selected_idx = self.get_next_idx(self.selected_idx);
        self.scroll_to_selected()
      }

      Message::FocusInput => operation::focus(SEARCH_INPUT_ID),

      Message::UpdateStaticMatches(static_matches) => {
        self
          .matches
          .insert(static_matches.id, static_matches.matches);
        Task::none()
      }

      Message::UpdateDynamicMatches(query, dynamic_matches) => {
        if query == self.query {
          self
            .matches
            .insert(dynamic_matches.id, dynamic_matches.matches);
        }

        Task::none()
      }

      Message::ProviderExecutionFinished(action) => match action {
        ExecutionFinishAction::Close => Task::done(Message::Close),
      },
    };

    Action::Task(task)
  }

  pub fn view(&self) -> Element<'_, Message, Base16Theme> {
    let mut provider_sections = column![];

    for (idx, info) in self.provider_info.iter().enumerate() {
      let Some(matches) = self.matches.get(&info.id) else {
        continue;
      };

      let matches = matches.iter().enumerate();
      let matches: Vec<(usize, &Match)> = if matches!(info.provider_type, ProviderType::Static) {
        matches
          .filter(|(_, m)| self.filter_static_match(m))
          .collect()
      } else {
        matches.collect()
      };

      if matches.is_empty() {
        continue;
      }

      let selected = if idx == self.selected_idx.0 {
        Some(self.selected_idx.1)
      } else {
        None
      };

      provider_sections =
        provider_sections.push(provider_section::view(info, matches, selected, |sub_idx| {
          Message::RunIdx((idx, sub_idx))
        }));
    }

    let column = column![
      search::view(&self.query).into(),
      column![rule::horizontal(1).style(|theme: &Base16Theme| theme::rule::colored(theme.base02))]
        .height(20),
      scrollable(provider_sections)
    ];

    let inner = container(column)
      .height(Length::Fill)
      .padding(8)
      .style(|theme| container::Style {
        background: Some(theme.base00.into()),
        ..Default::default()
      });

    container(inner)
      .padding(4)
      .style(|theme| {
        let gradient = gradient::Linear::new(50)
          .add_stop(0.0, theme.base0D)
          .add_stop(1.0, theme.base0E);

        container::Style {
          background: Some(gradient.into()),
          ..Default::default()
        }
      })
      .into()
  }

  pub fn subscription(&self, window_id: iced::window::Id) -> Subscription<Message> {
    iced::event::listen_with(|event, _, id| {
      (match event {
        iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::Close),
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
          iced::keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::SelectPrev),
          iced::keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::SelectNext),
          iced::keyboard::Key::Named(key::Named::Enter) => Some(Message::RunSelected),
          iced::keyboard::Key::Named(key::Named::Escape) => Some(Message::Close),
          _ => None,
        },
        _ => None,
      })
      .map(|m| (m, id))
    })
    .with(window_id)
    .filter_map(|(target_id, (message, event_id))| {
      if target_id == event_id {
        Some(message)
      } else {
        None
      }
    })
  }
}
