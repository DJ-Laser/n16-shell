use async_trait::async_trait;
use fend_core::{Context, FendResult, Interrupt};

use crate::providers::{ExecutionFinishAction, Match, Provider, ProviderInfo, ProviderType};

#[derive(Default, Debug)]
pub struct CalculatorProvider {
  context: Context,
}

impl CalculatorProvider {
  pub fn new() -> Self {
    Self {
      context: Context::new(),
    }
  }

  fn calculate_inner(
    input: &str,
    calculator_fn: impl FnOnce(&str) -> Option<FendResult>,
  ) -> Option<String> {
    if input.len() < 3 {
      return None;
    }

    let res = calculator_fn(input)?;

    if res.output_is_empty() {
      return None;
    }

    Some(res.get_main_result().into())
  }

  pub fn calculate(&self, input: &str) -> Option<String> {
    Self::calculate_inner(input, |input| {
      Some(fend_core::evaluate_preview_with_interrupt(
        input,
        &self.context,
        &NoInterrupt,
      ))
    })
  }
}

struct NoInterrupt;

impl Interrupt for NoInterrupt {
  fn should_interrupt(&self) -> bool {
    false
  }
}

#[async_trait]
impl Provider for CalculatorProvider {
  fn init() -> (ProviderInfo, Self)
  where
    Self: Sized,
  {
    (
      ProviderInfo {
        id: "n16/calculator".into(),
        name: "Calculator".into(),
        priorty: 10,
        provider_type: ProviderType::Dynamic,
      },
      Self::new(),
    )
  }

  async fn matches(&self) -> Vec<Match> {
    unimplemented!()
  }

  async fn matches_dynamic(&self, search_text: String) -> Vec<Match> {
    self
      .calculate(&search_text)
      .map(|result| Match {
        title: result,
        description: None,
        icon: None,
        keywords: Vec::new(),
        executable: false,
        id: 0,
      })
      .into_iter()
      .collect()
  }

  async fn execute_match(&self, _selected_match: Match) -> ExecutionFinishAction {
    ExecutionFinishAction::Close
  }
}
