mod game_data;
mod rando_data;

use std::fmt;

use wotw_seedgen_derive::VVariant;

use crate::{item::{Item, UberStateItem, UberStateOperator}, header::V};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum UberType {
    Bool,
    Teleporter,
    Byte,
    Int,
    Float,
}
impl UberType {
    pub fn code(&self) -> String {
        match self {
            UberType::Bool => "bool",
            UberType::Teleporter => "teleporter",
            UberType::Byte => "byte",
            UberType::Int => "int",
            UberType::Float => "float",
        }.to_string()
    }
}
impl std::str::FromStr for UberType {
    type Err = String;

    fn from_str(uber_type: &str) -> Result<UberType, String> {
        match uber_type {
            "bool" | "teleporter" => Ok(UberType::Bool),
            "byte" => Ok(UberType::Byte),
            "int" => Ok(UberType::Int),
            "float" => Ok(UberType::Float),
            _ => Err(format!("Invalid uberState type {}", uber_type)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct UberIdentifier {
    pub uber_group: u16,
    pub uber_id: u16,
}
impl UberIdentifier {
    pub fn from_parts(group: &str, id: &str) -> Result<UberIdentifier, String> {
        let uber_group: u16 = group.parse().map_err(|_| format!("invalid uber group {}", group))?;
        let uber_id: u16 = id.parse().map_err(|_| format!("invalid uber id {}", id))?;
        Ok(UberIdentifier {
            uber_group,
            uber_id,
        })
    }
}
impl UberIdentifier {
    pub fn code(&self) -> String {
        format!("{}|{}", self.uber_group, self.uber_id)
    }
}
impl fmt::Display for UberIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        rando_data::NAMED_UBER_STATES.iter()
            .find(|(_, identifier)| self == identifier)
            .map_or_else(
                ||
                game_data::UBER_STATES.iter()
                    .find(|(_, identifier)| self == identifier)
                    .map_or_else(|| self.code(), |(name, _)| (*name).to_string()),
                |(name, _)| (*name).to_string())
            .fmt(f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, VVariant)]
pub struct UberState {
    pub identifier: UberIdentifier,
    #[VWrap]
    pub value: String,
}
impl UberState {
    pub fn from_parts(group: &str, id: &str) -> Result<UberState, String> {
        let uber_group = group.parse().map_err(|_| format!("invalid uber group {group}"))?;
        let mut id_parts = id.splitn(2, '=');
        let uber_id = id_parts.next().unwrap().parse().map_err(|_| format!("invalid uber id {id}"))?;
        let value = id_parts.next().unwrap_or("");
        if !value.is_empty() {
            value.parse::<f32>().map_err(|_| format!("invalid uber value {value}"))?;
        }
        Ok(UberState {
            identifier: UberIdentifier {
                uber_group,
                uber_id,
            },
            value: value.to_string(),
        })
    }

    pub fn to_item(self, uber_type: UberType) -> Item {
        let value = if matches!(uber_type, UberType::Bool | UberType::Teleporter) {
            String::from("true")
        } else { self.value };

        Item::UberState(UberStateItem {
            uber_identifier: self.identifier,
            uber_type,
            signed: false,
            sign: false,
            operator: UberStateOperator::Value(value),
            skip: false,
        })
    }

    #[inline]
    pub fn spawn() -> UberState {
        UberState {
            identifier: UberIdentifier {
                uber_group: 3,
                uber_id: 0,
            },
            value: String::new(),
        }
    }
    #[inline]
    pub fn load() -> UberState {
        UberState {
            identifier: UberIdentifier {
                uber_group: 3,
                uber_id: 1,
            },
            value: String::new(),
        }
    }

    pub fn is_shop(&self) -> bool {
        self.identifier.uber_group == 1 ||
        self.identifier.uber_group == 2 ||
        self.identifier.uber_group == 48248 && matches!(self.identifier.uber_id, 19396 | 57987 | 41666)
    }
    pub fn is_purchasable(&self) -> bool {
        self.identifier.uber_group == 1 ||
        self.identifier.uber_group == 2 ||
        self.identifier.uber_group == 48248 && matches!(self.identifier.uber_id, 18767 | 45538 | 3638 | 1590 | 1557 | 29604 | 48423 | 61146 | 4045 | 19396 | 57987 | 41666)
    }
}
impl UberState {
    pub fn code(&self) -> String {
        if self.value.is_empty() {
            self.identifier.code()
        } else {
            format!("{}={}", self.identifier.code(), self.value)
        }
    }
}
impl fmt::Display for UberState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_empty() {
            write!(f, "{}", self.identifier)
        } else {
            write!(f, "{}={}", self.identifier, self.value)
        }
    }
}
impl fmt::Display for VUberState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bare = if let V::Literal(value) = &self.value {
            value.is_empty()
        } else { false };
        if bare {
            write!(f, "{}", self.identifier)
        } else {
            write!(f, "{}={}", self.identifier, self.value)
        }
    }
}
impl std::str::FromStr for UberState {
    type Err = String;

    fn from_str(uber_state: &str) -> Result<UberState, String> {
        let mut parts = uber_state.split(&['|', ','][..]);
        let uber_group = parts.next().ok_or_else(|| String::from("expected uber group"))?;
        let uber_id = parts.next().ok_or_else(|| String::from("expected uber id"))?;
        if parts.next().is_some() { return Err(String::from("expected only two parts")); }

        UberState::from_parts(uber_group, uber_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uber_states() {
        let uber_state = UberState::from_parts("25432", "7854").unwrap();
        assert_eq!(format!("{}", uber_state.code()), "25432|7854");
        let uber_state = UberState::from_parts("25432", "65195=11").unwrap();
        assert_eq!(format!("{}", uber_state.code()), "25432|65195=11");
        assert!(UberState::from_parts("", "3").is_err());
        assert!(UberState::from_parts("a", "3").is_err());
    }
}
