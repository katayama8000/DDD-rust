use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(i32);

impl CircleId {
    pub fn gen() -> Self {
        Self(rand::random::<i32>())
    }
}

impl Hash for CircleId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for CircleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<i32> for CircleId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl std::convert::From<CircleId> for i32 {
    fn from(circle_id: CircleId) -> Self {
        circle_id.0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let circle_id = CircleId::from(1);
        assert_eq!(circle_id.to_string(), "1");
        assert_eq!(i32::from(circle_id), 1);
    }
}
