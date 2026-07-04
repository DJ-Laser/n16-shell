use std::process;

use async_trait::async_trait;

use crate::providers::{ExecutionFinishAction, Match, Provider, ProviderInfo, ProviderType};

struct PowerManagementInfo {
  title: &'static str,
  keywords: &'static [&'static str],
  command: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
enum PowerManagementAction {
  Shutdown,
  Suspend,
  Hibernate,
  Reboot,
}

impl PowerManagementAction {
  const fn info(self) -> PowerManagementInfo {
    match self {
      PowerManagementAction::Shutdown => PowerManagementInfo {
        title: "Shut Down",
        keywords: &["power", "off"],
        command: &["systemctl", "poweroff"],
      },
      PowerManagementAction::Suspend => PowerManagementInfo {
        title: "Sleep",
        keywords: &["power", "suspend"],
        command: &["systemctl", "suspend"],
      },
      PowerManagementAction::Hibernate => PowerManagementInfo {
        title: "Hibernate",
        keywords: &["power"],
        command: &["systemctl", "hibernate"],
      },
      PowerManagementAction::Reboot => PowerManagementInfo {
        title: "Reboot",
        keywords: &["power", "restart"],
        command: &["systemctl", "reboot"],
      },
    }
  }
}

impl TryFrom<u64> for PowerManagementAction {
  type Error = u64;

  fn try_from(value: u64) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => PowerManagementAction::Shutdown,
      1 => PowerManagementAction::Suspend,
      2 => PowerManagementAction::Hibernate,
      3 => PowerManagementAction::Reboot,
      _ => return Err(value),
    })
  }
}

pub struct PowerManagementProvider {
  matches: Vec<Match>,
}

impl PowerManagementProvider {
  #[expect(clippy::new_without_default)]
  pub fn new() -> Self {
    Self {
      matches: Vec::new(),
    }
  }

  fn init_matches(&mut self) {
    self.matches = [
      PowerManagementAction::Shutdown,
      PowerManagementAction::Suspend,
      PowerManagementAction::Hibernate,
      PowerManagementAction::Reboot,
    ]
    .map(|action| {
      let PowerManagementInfo {
        title, keywords, ..
      } = action.info();
      Match {
        title: title.into(),
        description: None,
        icon: None,
        keywords: keywords.iter().map(|s| (*s).into()).collect(),
        executable: true,
        id: action as u64,
      }
    })
    .into()
  }
}

#[async_trait]
impl Provider for PowerManagementProvider {
  fn init() -> (ProviderInfo, Self)
  where
    Self: Sized,
  {
    let mut this = Self::new();
    this.init_matches();

    (
      ProviderInfo {
        id: "n16/power_management".into(),
        name: "Power Management".into(),
        priorty: -1,
        provider_type: ProviderType::Static,
      },
      this,
    )
  }

  async fn matches(&self) -> Vec<Match> {
    return self.matches.clone();
  }

  async fn matches_dynamic(&self, _search_text: String) -> Vec<Match> {
    unimplemented!()
  }

  async fn execute_match(&self, selected_match: Match) -> ExecutionFinishAction {
    let Ok(command) = selected_match
      .id
      .try_into()
      .map(|action: PowerManagementAction| action.info().command)
    else {
      return ExecutionFinishAction::Close;
    };

    if let Err(error) = process::Command::new(command[0])
      .args(&command[1..])
      .spawn()
    {
      eprintln!("{error}")
    };

    ExecutionFinishAction::Close
  }
}
