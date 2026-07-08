//! Types for describing the configuration of n16 plugins and capabilities.
//! Plugin configuration keys are defined in the `plugin.kdl`
//! All defined configuration keys are guaranteed to be present for the relavant config object.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Value for a
#[derive(Serialize, Deserialize)]
pub enum ConfigValue {
  Boolean(bool),
  Int(i64),
  Float(f64),
  String(String),
  Strings(Vec<String>),
}

/// Configuration for a plugin or capability.
/// Plugin configuration keys are defined in the `plugin.kdl`
/// All defined configuration keys are guaranteed to be present for the relavant config object.
/// It is recommended to serialize the config values into a custom struct instead of using `.get_*().unwrap()` everywhere.
#[derive(Serialize, Deserialize)]
pub struct Config {
  map: HashMap<String, ConfigValue>,
}

impl Config {
  pub fn new(map: HashMap<String, ConfigValue>) -> Self {
    Self { map }
  }

  pub fn get(&self, key: &'_ str) -> Option<&ConfigValue> {
    self.map.get(key)
  }

  pub fn get_boolean(&self, key: &'_ str) -> Option<bool> {
    self
      .map
      .get(key)
      .and_then(|value| {
        if let ConfigValue::Boolean(boolean) = value {
          Some(boolean)
        } else {
          None
        }
      })
      .copied()
  }

  pub fn get_int(&self, key: &'_ str) -> Option<i64> {
    self
      .map
      .get(key)
      .and_then(|value| {
        if let ConfigValue::Int(int) = value {
          Some(int)
        } else {
          None
        }
      })
      .copied()
  }

  pub fn get_float(&self, key: &'_ str) -> Option<f64> {
    self
      .map
      .get(key)
      .and_then(|value| {
        if let ConfigValue::Float(float) = value {
          Some(float)
        } else {
          None
        }
      })
      .copied()
  }

  pub fn get_string(&self, key: &'_ str) -> Option<&String> {
    self.map.get(key).and_then(|value| {
      if let ConfigValue::String(string) = value {
        Some(string)
      } else {
        None
      }
    })
  }

  pub fn get_strings(&self, key: &'_ str) -> Option<&Vec<String>> {
    self.map.get(key).and_then(|value| {
      if let ConfigValue::Strings(strings) = value {
        Some(strings)
      } else {
        None
      }
    })
  }
}
