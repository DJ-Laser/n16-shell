use std::{collections::HashMap, time::Duration};

use iced::{
  Element, Length, Subscription, Task, gradient,
  keyboard::key,
  widget::{column, container, operation, rule},
};
use n16_core::theme::{self, Base16Theme};

use crate::{
  Providers,
  component::{
    provider_section,
    search::{self, SEARCH_INPUT_ID},
  },
  providers::{ExecutionFinishAction, Match, Matches, ProviderId, ProviderInfo},
};

pub struct Launcher {
  query: String,
  selected_idx: (usize, usize),

  providers: Providers,
  provider_info: Vec<ProviderInfo>,
  static_matches: HashMap<ProviderId, Vec<Match>>,
  dynamic_matches: HashMap<ProviderId, Vec<Match>>,
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
        static_matches: HashMap::new(),
        dynamic_matches: HashMap::new(),
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
    Task::none()
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
      for matches in self.dynamic_matches.values_mut() {
        matches.clear();
      }

      Task::none()
    }
  }

  fn get_num_matches(&self, id: &str) -> usize {
    self.static_matches.get(id).map_or(0, Vec::len)
      + self.dynamic_matches.get(id).map_or(0, Vec::len)
  }

  fn get_prev_idx(&self) -> (usize, usize) {
    let mut idx = self.selected_idx;

    if idx.1 == 0 {
      if idx.0 == 0 {
        idx.0 = self.provider_info.len();
      }

      idx.0 -= 1;
      idx.1 = self.get_num_matches(&self.provider_info[idx.0].id);
    }

    idx.1 -= 1;

    idx
  }

  fn get_next_idx(&self) -> (usize, usize) {
    let Some(info) = self.provider_info.get(self.selected_idx.0) else {
      return (0, 0);
    };

    let mut idx = self.selected_idx;
    idx.1 += 1;
    if idx.1 >= self.get_num_matches(&info.id) {
      idx.0 += 1;
      idx.1 = 0;

      if idx.0 >= self.provider_info.len() {
        idx.0 = 0
      }
    }

    idx
  }

  fn get_match_at(&self, idx: (usize, usize)) -> Option<(&String, &Match)> {
    let id = &self.provider_info.get(idx.0)?.id;

    let dynamic_len = if let Some(dynamic_matches) = self.dynamic_matches.get(id) {
      if idx.1 < dynamic_matches.len() {
        return Some((id, &dynamic_matches[idx.1]));
      }

      dynamic_matches.len()
    } else {
      0
    };

    if let Some(static_matches) = self.static_matches.get(id) {
      let static_idx = idx.1 - dynamic_len;
      if static_idx < static_matches.len() {
        return Some((id, &static_matches[static_idx]));
      }
    }

    None
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
        self.selected_idx = self.get_prev_idx();
        self.scroll_to_selected()
      }

      Message::SelectNext => {
        self.selected_idx = self.get_next_idx();
        self.scroll_to_selected()
      }

      Message::FocusInput => operation::focus(SEARCH_INPUT_ID),

      Message::UpdateStaticMatches(static_matches) => {
        self
          .static_matches
          .insert(static_matches.id, static_matches.matches);
        Task::none()
      }

      Message::UpdateDynamicMatches(query, dynamic_matches) => {
        if query == self.query {
          self
            .dynamic_matches
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
      let matches: Vec<&Match> = match (
        self.dynamic_matches.get(&info.id).map(|v| v.iter()),
        self
          .static_matches
          .get(&info.id)
          .map(|v| v.iter().filter(|m| self.filter_static_match(m))),
      ) {
        (Some(dynamic_matches), Some(static_matches)) => {
          dynamic_matches.chain(static_matches).collect()
        }
        (Some(dynamic_matches), None) => dynamic_matches.collect(),
        (None, Some(static_matches)) => static_matches.collect(),
        (None, None) => continue,
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
      provider_sections
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
