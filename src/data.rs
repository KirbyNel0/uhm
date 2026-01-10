use chrono::{DateTime, TimeDelta, Utc};

use serde::{Deserialize, Serialize};

use crate::stats::UhmStats;

/// The core datastructure of the crate. It stores all data related to a uhm data series.
///
/// It does _not_ store the actual times at which the uhms occured. Instead, it stores
/// the start time and ordered offsets for each uhm ([Self::data]).
///
/// There are also some optional attributes which do not relate to the data directly
/// but can improve documentation/relationships.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Uhms {
    /// The timestamp at which the presentation started.
    #[serde(with = "uhm_serde")]
    pub start: DateTime<Utc>,
    /// The timestamp at which the presentation ended.
    #[serde(with = "uhm_serde")]
    pub end: DateTime<Utc>,
    /// The actual time data. This vector contains the offsets between individual "uhms"
    /// during the presentation.
    pub data: Vec<i64>,
    /// The name of this dataset. Is not required to, but should be, unique through all
    /// data sets. A dataset is not required to have a name.
    pub name: Option<String>,
    /// Optional notes for the dataset.
    pub notes: Option<String>,
}

impl Uhms {
    /// Calculate the time span of this series.
    pub fn duration(&self) -> TimeDelta {
        self.end - self.start
    }

    /// Calculate all stats for this series. See [uhm::stats] for more information.
    pub fn stats(&self) -> UhmStats {
        UhmStats::new(self)
    }
}

mod uhm_serde {
    use chrono::{DateTime, Utc};
    use serde::{Deserializer, Serializer};

    const DATE: &str = "%F %T %z";

    /// Serialize a `DateTime<Utc>` into the format of [DATE] using [serde].
    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.format(DATE).to_string())
    }

    /// This struct contains all required implementation for [serde] to be able to deserialize
    /// a `DateTime<Utc>`.
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "string formatted like '{}'", DATE)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let result = DateTime::parse_from_str(v, DATE);
            match result {
                Ok(date) => Ok(date.to_utc()),
                Err(error) => Err(serde::de::Error::custom(error)),
            }
        }
    }

    /// Deserialize a `DateTime<Utc>` into the format of [DATE] using [serde].
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(Visitor)
    }
}
