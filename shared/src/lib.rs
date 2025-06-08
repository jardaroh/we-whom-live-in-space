pub enum Node {
  Sun,
  Earth,
  Mars,
  Venus,
  Jupiter,
  Saturn,
  Uranus,
  Neptune,
  Pluto,
  Custom(String),
}
impl Node {
  pub fn as_str(&self) -> &str {
    match self {
      Node::Sun => "Sun",
      Node::Earth => "Earth",
      Node::Mars => "Mars",
      Node::Venus => "Venus",
      Node::Jupiter => "Jupiter",
      Node::Saturn => "Saturn",
      Node::Uranus => "Uranus",
      Node::Neptune => "Neptune",
      Node::Pluto => "Pluto",
      Node::Custom(name) => name,
    }
  }
}

impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

impl std::str::FromStr for Node {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Sun" => Ok(Node::Sun),
      "Earth" => Ok(Node::Earth),
      "Mars" => Ok(Node::Mars),
      "Venus" => Ok(Node::Venus),
      "Jupiter" => Ok(Node::Jupiter),
      "Saturn" => Ok(Node::Saturn),
      "Uranus" => Ok(Node::Uranus),
      "Neptune" => Ok(Node::Neptune),
      "Pluto" => Ok(Node::Pluto),
      _ => Ok(Node::Custom(s.to_string())),
    }
  }
}

pub enum EntityType {
  Ship,
  Planet,
  Asteroid,
  Container,
}

impl EntityType {
  pub fn as_str(&self) -> &str {
    match self {
      EntityType::Ship => "Ship",
      EntityType::Planet => "Planet",
      EntityType::Asteroid => "Asteroid",
      EntityType::Container => "Container",
    }
  }
}

impl std::fmt::Display for EntityType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

impl std::str::FromStr for EntityType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Ship" => Ok(EntityType::Ship),
      "Planet" => Ok(EntityType::Planet),
      "Asteroid" => Ok(EntityType::Asteroid),
      "Container" => Ok(EntityType::Container),
      _ => Err(format!("Unknown entity type: {}", s)),
    }
  }
}