use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
  pub value: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Workflow {
  pub name: String,
  pub steps: Vec<Step>,
}
