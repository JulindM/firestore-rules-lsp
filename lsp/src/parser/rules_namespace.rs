use strum::IntoStaticStr;

#[derive(Debug, Clone, Copy, PartialEq, IntoStaticStr)]
pub enum FirebaseType {
  UNKNOWN,
  Boolean,
  Bytes,
  Float,
  Integer,
  LatLng,
  List,
  Map,
  MapDiff,
  Number,
  Path,
  Duration,
  Request,
  Resource,
  Set,
  String,
  Timestamp,
  Null,
  hashing_module,
  latlng_module,
  math_module,
  timestamp_module,
  duration_module,
}

pub trait FirebaseTypeTrait {
  fn properties(&self) -> Vec<(&'static str, FirebaseType)>;
  fn methods(&self) -> Vec<(&'static str, FirebaseType, Vec<FirebaseType>)>;
  fn display_name(&self) -> &'static str;
}

impl FirebaseTypeTrait for FirebaseType {
  fn properties(&self) -> Vec<(&'static str, FirebaseType)> {
    match self {
      FirebaseType::Resource => vec![("data", FirebaseType::Map), ("id", FirebaseType::String)],
      _ => vec![],
    }
  }

  fn methods(&self) -> Vec<(&'static str, FirebaseType, Vec<FirebaseType>)> {
    match self {
      FirebaseType::math_module => {
        vec![
          ("size", FirebaseType::Number, vec![FirebaseType::Number]),
          ("ceil", FirebaseType::Number, vec![FirebaseType::Number]),
          ("floor", FirebaseType::Number, vec![FirebaseType::Number]),
          (
            "isInfinite",
            FirebaseType::Boolean,
            vec![FirebaseType::Number],
          ),
          (
            "isNaN",
            FirebaseType::Boolean,
            vec![FirebaseType::Number, FirebaseType::Number],
          ),
          (
            "pow",
            FirebaseType::Number,
            vec![FirebaseType::Number, FirebaseType::Number],
          ),
          ("round", FirebaseType::Number, vec![FirebaseType::Number]),
          ("sqrt", FirebaseType::Number, vec![FirebaseType::Number]),
        ]
      }
      FirebaseType::timestamp_module => {
        vec![
          (
            "date",
            FirebaseType::Timestamp,
            vec![
              FirebaseType::Integer,
              FirebaseType::Integer,
              FirebaseType::Integer,
            ],
          ),
          (
            "value",
            FirebaseType::Timestamp,
            vec![FirebaseType::Integer],
          ),
        ]
      }
      FirebaseType::Timestamp => {
        vec![
          ("date", FirebaseType::Timestamp, vec![]),
          ("year", FirebaseType::Integer, vec![]),
          ("month", FirebaseType::Integer, vec![]),
          ("day", FirebaseType::Integer, vec![]),
          ("dayOfWeek", FirebaseType::Integer, vec![]),
          ("dayOfYear", FirebaseType::Integer, vec![]),
          ("hours", FirebaseType::Integer, vec![]),
          ("minutes", FirebaseType::Integer, vec![]),
          ("nanos", FirebaseType::Integer, vec![]),
          ("seconds", FirebaseType::Integer, vec![]),
          ("time", FirebaseType::Duration, vec![]),
          ("toMillis", FirebaseType::Integer, vec![]),
        ]
      }
      FirebaseType::Duration => {
        vec![
          ("nanos", FirebaseType::Integer, vec![]),
          ("seconds", FirebaseType::Integer, vec![]),
        ]
      }
      FirebaseType::LatLng => {
        vec![("distance", FirebaseType::Float, vec![FirebaseType::LatLng])]
      }
      FirebaseType::Bytes => {
        vec![
          ("size", FirebaseType::Integer, vec![]),
          ("toBase64", FirebaseType::String, vec![]),
          ("toHexString", FirebaseType::String, vec![]),
        ]
      }
      FirebaseType::String => vec![
        ("lower", FirebaseType::String, vec![]),
        ("size", FirebaseType::Integer, vec![]),
        ("toUtf8", FirebaseType::Bytes, vec![]),
        ("trim", FirebaseType::String, vec![]),
        ("upper", FirebaseType::String, vec![]),
      ],
      FirebaseType::List => vec![
        ("concat", FirebaseType::List, vec![FirebaseType::List]),
        ("hasAll", FirebaseType::Boolean, vec![FirebaseType::List]),
        ("hasAny", FirebaseType::Boolean, vec![FirebaseType::List]),
        ("hasOnly", FirebaseType::Boolean, vec![FirebaseType::List]),
        ("join", FirebaseType::String, vec![FirebaseType::String]),
        ("removeAll", FirebaseType::List, vec![FirebaseType::List]),
        ("size", FirebaseType::Integer, vec![]),
        ("toSet", FirebaseType::Set, vec![]),
      ],
      _ => vec![],
    }
  }

  fn display_name(&self) -> &'static str {
    if FirebaseType::UNKNOWN == *self {
      return "";
    }

    self.into()
  }
}
