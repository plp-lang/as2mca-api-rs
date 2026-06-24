use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Flags(pub u64);

impl Flags {
  #[must_use]
  pub const fn has_flag(&self, bit: u32) -> bool {
    (self.0 & (1 << bit)) != 0
  }
}

impl TryFrom<String> for Flags {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    u64::from_str_radix(&value, 2)
      .map(Flags)
      .map_err(|e| format!("Invalid binary flags '{value}': {e}"))
  }
}

impl From<Flags> for String {
  fn from(flags: Flags) -> Self {
    format!("{:025b}", flags.0)
  }
}
