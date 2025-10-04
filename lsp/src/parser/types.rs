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
  Any,
  _UNSET,
}

pub trait FirebaseTypeTrait {
  fn properties(&self) -> Vec<(&'static str, FirebaseType)>;
  fn methods(&self) -> Vec<(&'static str, FirebaseType, Vec<FirebaseType>)>;
  fn docstring(&self) -> &'static str;
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
          FirebaseType::Any,
          vec![FirebaseType::Any, FirebaseType::Any],
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
        ("crc32", FirebaseType::Bytes, vec![FirebaseType::Any]),
        ("crc32c", FirebaseType::Bytes, vec![FirebaseType::Any]),
        ("md5", FirebaseType::Bytes, vec![FirebaseType::Any]),
        ("sha256", FirebaseType::Bytes, vec![FirebaseType::Any]),
      ],
      FirebaseType::latlng_module => vec![(
        "value",
        FirebaseType::LatLng,
        vec![FirebaseType::LatLng, FirebaseType::LatLng],
      )],
      FirebaseType::duration_module => vec![
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
      ],
      FirebaseType::LatLng => vec![
        (
          "distance",
          FirebaseType::Integer,
          vec![FirebaseType::LatLng],
        ),
        ("latitude", FirebaseType::Float, vec![]),
        ("longitude", FirebaseType::Float, vec![]),
      ],
      _ => vec![],
    }
  }

  fn docstring(&self) -> &'static str {
    match self {
      FirebaseType::Boolean => {
        "`Boolean`\n\nA boolean value (true or false).\n\nFor more information, see: [docs/reference/rules/rules.Boolean](https://firebase.google.com/docs/reference/rules/rules.Boolean)."
      }
      FirebaseType::Auth => {
        "`Auth`\n\nAn object representing the authentication context of the request.\n\nFor more information, see: [docs/reference/rules/rules.Auth](https://firebase.google.com/docs/reference/rules/rules.Auth)."
      }
      FirebaseType::Token => {
        "`Token`\n\nAn object representing the user's authentication token.\n\nFor more information, see: [docs/reference/rules/rules.Token](https://firebase.google.com/docs/reference/rules/rules.Token)."
      }
      FirebaseType::FirebaseMap => {
        "`FirebaseMap`\n\nAn object containing additional information about the user's authentication.\n\nFor more information, see: [docs/reference/rules/rules.FirebaseMap](https://firebase.google.com/docs/reference/rules/rules.FirebaseMap)."
      }
      FirebaseType::Bytes => {
        "`Bytes`\n\nA sequence of bytes, typically used for binary data.\n\nFor more information, see: [docs/reference/rules/rules.Bytes](https://firebase.google.com/docs/reference/rules/rules.Bytes)."
      }
      FirebaseType::Float => {
        "`Float`\n\nA floating-point number.\n\nFor more information, see: [docs/reference/rules/rules.Float](https://firebase.google.com/docs/reference/rules/rules.Float)."
      }
      FirebaseType::Integer => {
        "`Integer`\n\nAn integer number.\n\nFor more information, see: [docs/reference/rules/rules.Integer](https://firebase.google.com/docs/reference/rules/rules.Integer)."
      }
      FirebaseType::LatLng => {
        "`LatLng`\n\nA geographical point represented by latitude and longitude.\n\nFor more information, see: [docs/reference/rules/rules.LatLng](https://firebase.google.com/docs/reference/rules/rules.LatLng)."
      }
      FirebaseType::List => {
        "`List`\n\nAn ordered collection of values, which can be of any type.\n\nFor more information, see: [docs/reference/rules/rules.List](https://firebase.google.com/docs/reference/rules/rules.List)."
      }
      FirebaseType::Map => {
        "`Map`\n\nA collection of key-value pairs, where keys are strings and values can be of any type.\n\nFor more information, see: [docs/reference/rules/rules.Map](https://firebase.google.com/docs/reference/rules/rules.Map)."
      }
      FirebaseType::MapDiff => {
        "`MapDiff`\n\nAn object representing the difference between two maps.\n\nFor more information, see: [docs/reference/rules/rules.MapDiff](https://firebase.google.com/docs/reference/rules/rules.MapDiff)."
      }
      FirebaseType::Number => {
        "`Number`\n\nA numeric value, which can be either an integer or a floating-point number.\n\nFor more information, see: [docs/reference/rules/rules.Number](https://firebase.google.com/docs/reference/rules/rules.Number)."
      }
      FirebaseType::Path => {
        "`Path`\n\nA reference to a location in the database.\n\nFor more information, see: [docs/reference/rules/rules.Path](https://firebase.google.com/docs/reference/rules/rules.Path)."
      }
      FirebaseType::Duration => {
        "`Duration`\n\nA span of time, represented in seconds and nanoseconds.\n\nFor more information, see: [docs/reference/rules/rules.Duration](https://firebase.google.com/docs/reference/rules/rules.Duration)."
      }
      FirebaseType::Request => {
        "`Request`\n\nAn object representing the incoming request to the database.\n\nFor more information, see: [docs/reference/rules/rules.Request](https://firebase.google.com/docs/reference/rules/rules.Request)."
      }
      FirebaseType::Resource => {
        "`Resource`\n\nAn object representing the current state of the database resource being accessed.\n\nFor more information, see: [docs/reference/rules/rules.Resource](https://firebase.google.com/docs/reference/rules/rules.Resource)."
      }
      FirebaseType::Set => {
        "`Set`\n\nAn unordered collection of unique values.\n\nFor more information, see: [docs/reference/rules/rules.Set](https://firebase.google.com/docs/reference/rules/rules.Set)."
      }
      FirebaseType::String => {
        "`String`\n\nA sequence of characters, typically used for text data.\n\nFor more information, see: [docs/reference/rules/rules.String](https://firebase.google.com/docs/reference/rules/rules.String)."
      }
      FirebaseType::Timestamp => {
        "`Timestamp`\n\nA point in time, represented as seconds and nanoseconds since the Unix epoch.\n\nFor more information, see: [docs/reference/rules/rules.Timestamp](https://firebase.google.com/docs/reference/rules/rules.Timestamp)."
      }
      FirebaseType::Null => {
        "`Null`\n\nA null value, representing the absence of a value.\n\nFor more information, see: [docs/reference/rules/rules.Null](https://firebase.google.com/docs/reference/rules/rules.Null)."
      }
      FirebaseType::hashing_module => {
        "`hashing`\n\nA module providing hashing functions for various algorithms.\n\nFor more information, see: [docs/reference/rules/rules.hashing](https://firebase.google.com/docs/reference/rules/rules.hashing)."
      }
      FirebaseType::latlng_module => {
        "`latlng`\n\nA module providing functions for geographical calculations using latitude and longitude.\n\nFor more information, see: [docs/reference/rules/rules.latlng](https://firebase.google.com/docs/reference/rules/rules.latlng)."
      }
      FirebaseType::math_module => {
        "`math`\n\nA module providing mathematical functions and constants.\n\nFor more information, see: [docs/reference/rules/rules.math](https://firebase.google.com/docs/reference/rules/rules.math)."
      }
      FirebaseType::timestamp_module => {
        "`timestamp`\n\nA module providing functions for creating and manipulating timestamps.\n\nFor more information, see: [docs/reference/rules/rules.timestamp](https://firebase.google.com/docs/reference/rules/rules.timestamp)."
      }
      FirebaseType::duration_module => {
        "`duration`\n\nA module providing functions for creating and manipulating durations.\n\nFor more information, see: [docs/reference/rules/rules.duration](https://firebase.google.com/docs/reference/rules/rules.duration)."
      }
      FirebaseType::Any => "`Any`",
      FirebaseType::_UNSET => "`_UNSET`\n\nAn unset or unknown type.",
    }
  }
}

pub fn namespace_reserved_function<'b>(name: &str) -> Option<FirebaseType> {
  match name {
    "get" => Some(FirebaseType::Resource),
    "path" => Some(FirebaseType::Path),
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

pub fn from_type_str<'b>(name: &str) -> Option<FirebaseType> {
  match name {
    "bool" => Some(FirebaseType::Boolean),
    "int" => Some(FirebaseType::Integer),
    "float" => Some(FirebaseType::Float),
    "number" => Some(FirebaseType::Number),
    "string" => Some(FirebaseType::String),
    "list" => Some(FirebaseType::List),
    "map" => Some(FirebaseType::Map),
    "timestamp" => Some(FirebaseType::Timestamp),
    "duration" => Some(FirebaseType::Duration),
    "path" => Some(FirebaseType::Path),
    "latlng" => Some(FirebaseType::LatLng),
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
