use fend_core::{Context, FendResult, Interrupt};

#[derive(Default, Debug)]
pub struct Calculator {
  pub context: Context,
  base_variables: Vec<u8>,
}

impl Calculator {
  pub fn new() -> Self {
    let mut new = Self {
      context: Context::new(),
      base_variables: Vec::new(),
    };

    new.save_variables();
    new
  }

  fn save_variables(&mut self) {
    self.base_variables.clear();
    self
      .context
      .serialize_variables(&mut self.base_variables)
      .expect("Vec should be writable without error");
  }

  pub fn reset_context(&mut self) {
    self
      .context
      .deserialize_variables(&mut self.base_variables.as_slice())
      .expect("`base_variables` should have proper variables serialized");
  }

  fn calculate_inner(
    input: &str,
    calculator_fn: impl FnOnce(&str) -> Option<FendResult>,
  ) -> Option<String> {
    if input.len() < 3 {
      return None;
    }

    let res = calculator_fn(input)?;

    if res.is_unit_type() {
      return None;
    }

    Some(res.get_main_result().into())
  }

  pub fn calculate_preview(&self, input: &str) -> Option<String> {
    Self::calculate_inner(input, |input| {
      Some(fend_core::evaluate_preview_with_interrupt(
        input,
        &mut self.context.clone(),
        &NoInterrupt,
      ))
    })
  }

  pub fn calculate(&mut self, input: &str) -> Option<String> {
    Self::calculate_inner(input, |input| {
      fend_core::evaluate(input, &mut self.context).ok()
    })
  }
}

struct NoInterrupt;

impl Interrupt for NoInterrupt {
  fn should_interrupt(&self) -> bool {
    false
  }
}
