#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirebaseType {
  Boolean,
  Bytes,
  Duration,
  Float,
  Integer,
  LatLng,
  List,
  Map,
  MapDiff,
  Number,
  Path,
  Request,
  Resource,
  Set,
  String,
  Timestamp,
  Null,
  UNKNOWN,
}

pub trait FirebaseTypeTrait {
  fn properties(self) -> Vec<(&'static str, FirebaseType)>;
  fn methods(self) -> Vec<(&'static str, FirebaseType, Vec<FirebaseType>)>;
}

impl FirebaseTypeTrait for FirebaseType {
  fn properties(self) -> Vec<(&'static str, FirebaseType)> {
    match self {
      FirebaseType::Boolean => vec![],
      FirebaseType::Bytes => vec![],
      FirebaseType::Duration => vec![],
      FirebaseType::Float => vec![],
      FirebaseType::Integer => vec![],
      FirebaseType::LatLng => vec![],
      FirebaseType::List => vec![],
      FirebaseType::Map => vec![],
      FirebaseType::MapDiff => vec![],
      FirebaseType::Number => vec![],
      FirebaseType::Path => vec![],
      FirebaseType::Request => vec![],
      FirebaseType::Resource => vec![],
      FirebaseType::Set => vec![],
      FirebaseType::String => vec![],
      FirebaseType::Timestamp => vec![],
      FirebaseType::UNKNOWN => vec![],
      FirebaseType::Null => vec![],
    }
  }

  fn methods(self) -> Vec<(&'static str, FirebaseType, Vec<FirebaseType>)> {
    match self {
      FirebaseType::Boolean => vec![],
      FirebaseType::Bytes => {
        vec![
          ("size", FirebaseType::Integer, vec![]),
          ("toBase64", FirebaseType::String, vec![]),
          ("toHexString", FirebaseType::String, vec![]),
        ]
      }
      FirebaseType::Duration => vec![],
      FirebaseType::Float => vec![],
      FirebaseType::Integer => vec![],
      FirebaseType::LatLng => vec![],
      FirebaseType::List => vec![],
      FirebaseType::Map => vec![],
      FirebaseType::MapDiff => vec![],
      FirebaseType::Number => vec![],
      FirebaseType::Path => vec![],
      FirebaseType::Request => vec![],
      FirebaseType::Resource => vec![],
      FirebaseType::Set => vec![],
      FirebaseType::String => vec![
        ("lower", FirebaseType::String, vec![]),
        ("size", FirebaseType::Integer, vec![]),
        ("toUtf8", FirebaseType::Bytes, vec![]),
        ("trim", FirebaseType::String, vec![]),
        ("upper", FirebaseType::String, vec![]),
      ],
      FirebaseType::Timestamp => vec![],
      FirebaseType::UNKNOWN => vec![],
      FirebaseType::Null => vec![],
    }
  }
}
