use serde::{Deserialize, Deserializer, Serialize};
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::Type;
use sqlx::{error::BoxDynError, Decode, Postgres};

/// Represent the lane of a vehicle in a two-lane road.
/// /// The `Lane` enum has two variants:
/// /// - `Left`: Represents the left lane. (0)
/// /// - `Right`: Represents the right lane. (1)
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[repr(u8)]
pub enum Lane {
    Left = 0,
    Right = 1,
}

/// Implementing the `Deserialize` trait for `Lane` to allow it to be deserialized from JSON or other formats.
impl<'de> Deserialize<'de> for Lane {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: u8 = Deserialize::deserialize(deserializer)?;
        match value {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err(serde::de::Error::custom("Invalid lane value")),
        }
    }
}

/// Implementing the `Serialize` trait for `Lane` to allow it to be serialized to JSON or other formats.
impl Serialize for Lane {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value:u8 = match self {
            Self::Left => 0_u8,
            Self::Right => 1_u8,
        };
        serializer.serialize_u8(value)
    }
}

/// Implementing the `Decode` trait for `Lane` to allow it to be decoded from a `PostgreSQL` value.
impl<'r> Decode<'r, Postgres> for Lane {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let v = <i32 as Decode<Postgres>>::decode(value)?;
        Self::try_from(v).map_err(|_| "invalid value for Lane".into())
    }
}

/// Implementing the `Type` trait for `Lane` to specify its `PostgreSQL` type information.
impl Type<Postgres> for Lane {
    fn type_info() -> PgTypeInfo {
        <i32 as Type<Postgres>>::type_info()
    }
}

/// Implementing `TryFrom<i32>` to convert an integer to a `Lane`.
impl TryFrom<i32> for Lane {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err("Invalid value for Lane"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lane_deserialization() {
        let left: Lane = serde_json::from_str("0").unwrap();
        let right: Lane = serde_json::from_str("1").unwrap();
        assert_eq!(left, Lane::Left);
        assert_eq!(right, Lane::Right);

        let invalid: Result<Lane, _> = serde_json::from_str("2");
        assert!(invalid.is_err());
    }

    #[tokio::test]
    async fn test_lane_try_from() {
        assert_eq!(Lane::try_from(0_i32), Ok(Lane::Left));
        assert_eq!(Lane::try_from(1_i32), Ok(Lane::Right));
        assert_eq!(Lane::try_from(2_i32), Err("Invalid value for Lane"));
    }

    #[tokio::test]
    async fn test_lane_serialization() {
        let left = Lane::Left;
        let right = Lane::Right;

        let left_json = serde_json::to_string(&left).unwrap();
        let right_json = serde_json::to_string(&right).unwrap();

        assert_eq!(left_json, "0");
        assert_eq!(right_json, "1");

        let deserialized_left: Lane = serde_json::from_str(&left_json).unwrap();
        let deserialized_right: Lane = serde_json::from_str(&right_json).unwrap();

        assert_eq!(deserialized_left, Lane::Left);
        assert_eq!(deserialized_right, Lane::Right);
    }
}
