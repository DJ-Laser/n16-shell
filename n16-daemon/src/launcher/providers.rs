mod applications;
mod calculator;
mod power_management;

use std::{collections::HashMap, sync::Arc};

pub use applications::ApplicationProvider;
use async_trait::async_trait;
pub use calculator::CalculatorProvider;
use futures_lite::Stream;
use iced::widget::{image, svg};
pub use power_management::PowerManagementProvider;

#[derive(Debug, Clone)]
pub enum ProviderType {
  /// Provides a static list of matches to be filtered by the search text
  Static,
  /// Provides a dynamic list of matches based on the search text
  Dynamic,
}

pub type ProviderId = String;

#[derive(Debug, Clone)]
pub struct ProviderInfo {
  /// Unique Provider Id
  pub id: ProviderId,
  /// Provider name shown to the user
  pub name: String,
  /// Priority, higher appears first
  pub priorty: i64,
  /// Provider type
  pub provider_type: ProviderType,
}

#[derive(Debug, Clone)]
pub enum ExecutionFinishAction {
  Close,
}

#[async_trait]
pub trait Provider: Send {
  /// Initialize the provider and return it's information.
  /// Called on startup and when config is refreshed
  fn init() -> (ProviderInfo, Self)
  where
    Self: Sized;

  /// Get the static matches
  /// Only called on `ProviderType::Static` Providers
  async fn matches(&self) -> Vec<Match>;

  /// Update the dynamic matches based on the search text
  /// Only called on `ProviderType::Dynamic` Providers
  async fn matches_dynamic(&self, search_text: String) -> Vec<Match>;

  async fn execute_match(&self, selected_match: Match) -> ExecutionFinishAction;
}

#[derive(Debug, Clone)]
pub enum MatchIcon {
  Bitmap(image::Handle),
  Vector(svg::Handle),
}

#[derive(Debug, Clone)]
pub struct Match {
  /// Title shown to the user
  pub title: String,
  /// Optional description shown to the user
  pub description: Option<String>,
  /// Optional icon shown to the user
  pub icon: Option<MatchIcon>,
  /// Keywords to use when filtering
  pub keywords: Vec<String>,

  /// Whether the match can be executed
  pub executable: bool,
  /// Id used for idenitfying matches when handling their execution
  pub id: u64,
}

#[derive(Debug, Clone)]
pub struct Matches {
  pub id: ProviderId,
  pub matches: Vec<Match>,
}

type ProvidersInner = HashMap<ProviderId, (ProviderInfo, Box<dyn Provider + Sync>)>;

#[derive(Clone)]
pub struct Providers {
  providers: Arc<ProvidersInner>,
}

impl Providers {
  pub fn get_sorted_provider_info(&self) -> Vec<ProviderInfo> {
    let mut info: Vec<ProviderInfo> = self
      .providers
      .values()
      .map(|(info, _)| info)
      .cloned()
      .collect();
    info.sort_by(|a, b| a.name.cmp(&b.name));
    info.sort_by_key(|b| std::cmp::Reverse(b.priorty));
    info
  }

  pub fn get_static_matches(&mut self) -> impl Stream<Item = Matches> + use<> {
    let (matches_tx, matches_rx) = async_channel::unbounded();
    let providers = self.providers.clone();
    tokio::spawn(async move {
      for (info, provider) in providers.values() {
        if matches!(info.provider_type, ProviderType::Static) {
          let _ = matches_tx.try_send(Matches {
            id: info.id.clone(),
            matches: provider.matches().await,
          });
        }
      }
    });

    matches_rx
  }

  pub fn get_dynamic_matches(&mut self, query: String) -> impl Stream<Item = Matches> + use<> {
    let (matches_tx, matches_rx) = async_channel::unbounded();
    let providers = self.providers.clone();
    tokio::spawn(async move {
      for (info, provider) in providers.values() {
        if matches!(info.provider_type, ProviderType::Dynamic) {
          let _ = matches_tx.try_send(Matches {
            id: info.id.clone(),
            matches: provider.matches_dynamic(query.clone()).await,
          });
        }
      }
    });

    matches_rx
  }

  pub fn execute_match(
    &self,
    (id, selected_match): (String, Match),
  ) -> impl Future<Output = Option<ExecutionFinishAction>> + use<> {
    let (action_tx, action_rx) = async_channel::unbounded();
    let providers = self.providers.clone();
    tokio::spawn(async move {
      let Some((_, provider)) = providers.get(&id) else {
        let _ = action_tx.send(None).await;
        return;
      };

      println!("Processing provider: {id}");

      let _ = action_tx
        .send(Some(provider.execute_match(selected_match).await))
        .await;
    });

    async move { action_rx.recv().await.ok().flatten() }
  }
}

pub struct ProvidersBuilder {
  providers: ProvidersInner,
}

impl ProvidersBuilder {
  pub fn new() -> Self {
    Self {
      providers: HashMap::new(),
    }
  }

  pub fn add_provider<P: Provider + Sync + 'static>(&mut self) {
    let (info, provider) = P::init();
    self
      .providers
      .insert(info.id.clone(), (info, Box::new(provider)));
  }

  pub fn build(self) -> Providers {
    Providers {
      providers: Arc::new(self.providers),
    }
  }
}
