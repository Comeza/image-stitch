use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum WriteDirection {
  X,
  Y,
}

impl FromStr for WriteDirection {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "x" => Ok(WriteDirection::X),
      "y" => Ok(WriteDirection::Y),
      _ => Err("not a valid direction"),
    }
  }
}
