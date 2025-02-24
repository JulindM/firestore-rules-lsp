use strum::AsRefStr;

#[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
pub enum FirebaseType {
  Boolean,
  Auth,
  Token,
  FirebaseMap,
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
  UNKNOWN,
}

pub trait FirebaseTypeTrait {
  fn properties(&self) -> Vec<(&'static str, FirebaseType)>;
  fn methods(&self) -> Vec<(&'static str, FirebaseType, Vec<FirebaseType>)>;
}

impl FirebaseTypeTrait for FirebaseType {
  fn properties(&self) -> Vec<(&'static str, FirebaseType)> {
    match self {
      FirebaseType::Resource => vec![("data", FirebaseType::Map), ("id", FirebaseType::String)],
      FirebaseType::Request => vec![
        ("auth", FirebaseType::Auth),
        ("method", FirebaseType::String),
        ("path", FirebaseType::Path),
        ("query", FirebaseType::Map),
        ("resource", FirebaseType::Resource),
        ("time", FirebaseType::Timestamp),
      ],
      FirebaseType::Auth => vec![
        ("uid", FirebaseType::String),
        ("token", FirebaseType::Token),
      ],
      FirebaseType::Token => vec![
        ("email", FirebaseType::String),
        ("email_verified", FirebaseType::Boolean),
        ("phone_number", FirebaseType::String),
        ("name", FirebaseType::String),
        ("sub", FirebaseType::String),
        ("firebase", FirebaseType::FirebaseMap),
      ],
      FirebaseType::FirebaseMap => vec![
        ("identities", FirebaseType::Map),
        ("sign_in_provider", FirebaseType::String),
        ("tenant", FirebaseType::String),
      ],
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
      FirebaseType::Map => vec![
        ("diff", FirebaseType::MapDiff, vec![FirebaseType::Map]),
        (
          "get",
          FirebaseType::UNKNOWN,
          vec![FirebaseType::UNKNOWN, FirebaseType::UNKNOWN],
        ),
        ("size", FirebaseType::Integer, vec![]),
        ("keys", FirebaseType::List, vec![]),
        ("values", FirebaseType::List, vec![]),
      ],
      FirebaseType::MapDiff => vec![
        ("addedKeys", FirebaseType::Set, vec![]),
        ("affectedKeys", FirebaseType::Set, vec![]),
        ("changedKeys", FirebaseType::Set, vec![]),
        ("unchangedKeys", FirebaseType::List, vec![]),
        ("removedKeys", FirebaseType::Set, vec![]),
      ],
      FirebaseType::Path => vec![("bind", FirebaseType::Path, vec![FirebaseType::Map])],
      FirebaseType::Boolean => vec![],
      FirebaseType::Float => vec![],
      FirebaseType::Integer => vec![],
      FirebaseType::Number => vec![],
      FirebaseType::Request => vec![],
      FirebaseType::Resource => vec![],
      FirebaseType::Set => vec![
        ("difference", FirebaseType::Set, vec![]),
        ("hasAll", FirebaseType::Boolean, vec![FirebaseType::Set]),
        ("hasAny", FirebaseType::Boolean, vec![FirebaseType::Set]),
        ("hasOnly", FirebaseType::Boolean, vec![FirebaseType::Set]),
        ("intersection", FirebaseType::Set, vec![FirebaseType::Set]),
        ("union", FirebaseType::Set, vec![FirebaseType::Set]),
        ("size", FirebaseType::Integer, vec![]),
      ],
      FirebaseType::Null => vec![],
      FirebaseType::hashing_module => vec![
        ("crc32", FirebaseType::Bytes, vec![FirebaseType::UNKNOWN]),
        ("crc32c", FirebaseType::Bytes, vec![FirebaseType::UNKNOWN]),
        ("md5", FirebaseType::Bytes, vec![FirebaseType::UNKNOWN]),
        ("sha256", FirebaseType::Bytes, vec![FirebaseType::UNKNOWN]),
      ],
      FirebaseType::latlng_module => vec![(
        "value",
        FirebaseType::LatLng,
        vec![FirebaseType::LatLng, FirebaseType::LatLng],
      )],
      FirebaseType::duration_module => {
        vec![
          ("abs", FirebaseType::Duration, vec![FirebaseType::Duration]),
          (
            "time",
            FirebaseType::Duration,
            vec![
              FirebaseType::Integer,
              FirebaseType::Integer,
              FirebaseType::Integer,
              FirebaseType::Integer,
            ],
          ),
          (
            "static",
            FirebaseType::Duration,
            vec![FirebaseType::Integer, FirebaseType::String],
          ),
        ]
      }
      FirebaseType::UNKNOWN => vec![],
      FirebaseType::Auth => vec![],
      FirebaseType::Token => vec![],
      FirebaseType::FirebaseMap => vec![],
      FirebaseType::LatLng => vec![
        (
          "distance",
          FirebaseType::Integer,
          vec![FirebaseType::LatLng],
        ),
        ("latitude", FirebaseType::Float, vec![]),
        ("longitude", FirebaseType::Float, vec![]),
      ],
    }
  }
}

pub fn namespace_reserved_function<'b>(name: &str) -> Option<FirebaseType> {
  match name {
    "get" => Some(FirebaseType::Resource),
    "getAfter" => Some(FirebaseType::Resource),
    "exists" => Some(FirebaseType::Boolean),
    "existsAfer" => Some(FirebaseType::Boolean),
    "debug" => Some(FirebaseType::Boolean),
    _ => None,
  }
}

pub fn namespace_reserved_variable<'b>(name: &str) -> Option<FirebaseType> {
  match name {
    "duration" => Some(FirebaseType::duration_module),
    "hashing" => Some(FirebaseType::hashing_module),
    "latlng" => Some(FirebaseType::latlng_module),
    "math" => Some(FirebaseType::math_module),
    "timestamp" => Some(FirebaseType::timestamp_module),
    "request" => Some(FirebaseType::Request),
    "resource" => Some(FirebaseType::Resource),
    _ => None,
  }
}

pub fn infer_function_type<'a>(obj_type: FirebaseType, fun_name: &'a str) -> Option<FirebaseType> {
  obj_type
    .methods()
    .iter()
    .find(|f| f.0 == fun_name)
    .and_then(|to| Some(to.1))
}

pub fn infer_variable_type<'a>(obj_type: FirebaseType, var_name: &'a str) -> Option<FirebaseType> {
  obj_type
    .properties()
    .iter()
    .find(|f| f.0 == var_name)
    .and_then(|to| Some(to.1))
}
