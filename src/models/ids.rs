use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use destru::{decode, encode};

pub const USER_FLAG: u8 = 0;
pub const STRUCTURE_FLAG: u8 = 1;

macro_rules! define_id {
    ($name:ident, $flag:expr, $expecting:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub i64);

        impl From<i64> for $name {
            fn from(value: i64) -> Self {
                $name(value)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s = encode($flag, self.0).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&s)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct SqidsVisitor;

                impl Visitor<'_> for SqidsVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!("a valid ", $expecting, " string"))
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        decode($flag, s).map($name).map_err(E::custom)
                    }
                }

                deserializer.deserialize_str(SqidsVisitor)
            }
        }
    };
}

define_id!(UserID, USER_FLAG, "UserID");
define_id!(StructureID, STRUCTURE_FLAG, "StructureID");