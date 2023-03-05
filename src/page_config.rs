use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct PageConfig {
  pub(crate) chain: Chain,
  pub(crate) domain: Option<String>,
}
