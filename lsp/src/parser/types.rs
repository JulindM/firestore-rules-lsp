use std::sync::LazyLock;

use strum::AsRefStr;

use crate::parser::base::{FirebaseTypeInformation, FunctionParameter};

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
  Any,
  hashing_module,
  latlng_module,
  math_module,
  timestamp_module,
  duration_module,
}

pub trait FirebaseTypeTrait {
  fn properties(&self) -> Vec<(&'static str, FirebaseTypeInformation)>;
  fn methods(
    &self,
  ) -> Vec<(
    &'static str,
    Vec<FunctionParameter>,
    FirebaseTypeInformation,
  )>;
  fn docstring(&self) -> &'static str;
}

impl FirebaseTypeTrait for FirebaseType {
  fn properties(&self) -> Vec<(&'static str, FirebaseTypeInformation)> {
    match self {
      FirebaseType::Resource => vec![
        (
          "__name__",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The resource's full document path as a string.",
          ),
        ),
        (
          "data",
          FirebaseTypeInformation::new_documented(FirebaseType::Map, "Map of the document data"),
        ),
        (
          "id",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The resource's document ID.",
          ),
        ),
      ],
      FirebaseType::Request => vec![
        (
          "auth",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Auth,
            "The request authentication context. For more see [docs/reference/rules/rules.Request.auth](https://firebase.google.com/docs/reference/rules/rules.firestore.Request#auth).",
          ),
        ),
        (
          "method",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The request method\n(for example, 'get', 'list', 'create', 'update', or 'delete').",
          ),
        ),
        (
          "path",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Path,
            "The request's resource path.",
          ),
        ),
        (
          "query",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Map,
            "The request's query parameters. For more see [docs/reference/rules/rules.Request.query](https://firebase.google.com/docs/reference/rules/rules.firestore.Request#query).",
          ),
        ),
        (
          "resource",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Resource,
            "The new resource value, present on write requests only.",
          ),
        ),
        (
          "time",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Timestamp,
            "When the request was received by the service. For Firestore write operations that include server-side timestamps, this time will be equal to the server timestamp.",
          ),
        ),
      ],
      FirebaseType::Auth => vec![
        (
          "uid",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The UID of the requesting user.",
          ),
        ),
        (
          "token",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Token,
            "A map of JWT token claims.",
          ),
        ),
      ],
      FirebaseType::Token => vec![
        (
          "email",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The email address associated with the account, if present.",
          ),
        ),
        (
          "email_verified",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "true if the user has verified they have access to the email address.",
          ),
        ),
        (
          "phone_number",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The phone number associated with the account, if present.",
          ),
        ),
        (
          "name",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The user's display name, if set.",
          ),
        ),
        (
          "sub",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The user's Firebase UID. This is unique within a project.",
          ),
        ),
        (
          "firebase",
          FirebaseTypeInformation::new_documented(
            FirebaseType::FirebaseMap,
            "Firebase-specific token claims including identities, sign-in provider, and tenant information.",
          ),
        ),
      ],
      FirebaseType::FirebaseMap => vec![
        (
          "identities",
          FirebaseTypeInformation::new_documented(
            FirebaseType::Map,
            "A map of all the identities that are associated with this user's account. The keys can be 'email', 'phone', 'google.com', 'facebook.com', 'github.com', 'twitter.com'.",
          ),
        ),
        (
          "sign_in_provider",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The sign-in provider used to obtain this token. Can be one of: 'custom', 'password', 'phone', 'anonymous', 'google.com', 'facebook.com', 'github.com', 'twitter.com'.",
          ),
        ),
        (
          "tenant",
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "The tenantId associated with the account, if present.",
          ),
        ),
      ],
      _ => vec![],
    }
  }

  fn methods(
    &self,
  ) -> Vec<(
    &'static str,
    Vec<FunctionParameter>,
    FirebaseTypeInformation,
  )> {
    match self {
      FirebaseType::math_module => {
        vec![
          (
            "ceil",
            vec![FunctionParameter::new(
              "number",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Number,
                FirebaseType::Number.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Number,
              "Ceiling of the given number.",
            ),
          ),
          (
            "floor",
            vec![FunctionParameter::new(
              "number",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Number,
                FirebaseType::Number.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Number,
              "Floor of the numeric value.",
            ),
          ),
          (
            "isInfinite",
            vec![FunctionParameter::new(
              "number",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Number,
                FirebaseType::Number.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Boolean,
              "Test whether the value is infinite.",
            ),
          ),
          (
            "isNaN",
            vec![FunctionParameter::new(
              "number",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Number,
                FirebaseType::Number.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Boolean,
              "Test whether the value is not a number.",
            ),
          ),
          (
            "pow",
            vec![
              FunctionParameter::new(
                "base",
                FirebaseTypeInformation::new_documented(
                  FirebaseType::Number,
                  FirebaseType::Number.docstring(),
                ),
              ),
              FunctionParameter::new(
                "exponent",
                FirebaseTypeInformation::new_documented(
                  FirebaseType::Number,
                  FirebaseType::Number.docstring(),
                ),
              ),
            ],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Number,
              "Return the value of the first argument raised to the power of the second argument.",
            ),
          ),
          (
            "round",
            vec![FunctionParameter::new(
              "number",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Number,
                FirebaseType::Number.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Number,
              "Round the input value to the nearest int.",
            ),
          ),
          (
            "sqrt",
            vec![FunctionParameter::new(
              "number",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Number,
                FirebaseType::Number.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Number,
              "Square root of the input value.",
            ),
          ),
        ]
      }
      FirebaseType::timestamp_module => {
        vec![
          (
            "date",
            vec![
              FunctionParameter::new(
                "year",
                FirebaseTypeInformation::new_documented(
                  FirebaseType::Integer,
                  FirebaseType::Integer.docstring(),
                ),
              ),
              FunctionParameter::new(
                "month",
                FirebaseTypeInformation::new_documented(
                  FirebaseType::Integer,
                  FirebaseType::Integer.docstring(),
                ),
              ),
              FunctionParameter::new(
                "day",
                FirebaseTypeInformation::new_documented(
                  FirebaseType::Integer,
                  FirebaseType::Integer.docstring(),
                ),
              ),
            ],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Timestamp,
              "Create a timestamp from year, month, and day values.",
            ),
          ),
          (
            "value",
            vec![FunctionParameter::new(
              "epochMillis",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Integer,
                FirebaseType::Integer.docstring(),
              ),
            )],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Timestamp,
              "Create a timestamp from milliseconds since epoch.",
            ),
          ),
        ]
      }
      FirebaseType::Timestamp => {
        vec![
          (
            "date",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Timestamp,
              "Timestamp value containing year, month, and day only.",
            ),
          ),
          (
            "year",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the year value of the timestamp.",
            ),
          ),
          (
            "month",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the month value of the timestamp.",
            ),
          ),
          (
            "day",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the day value of the timestamp.",
            ),
          ),
          (
            "dayOfWeek",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the day of the week as a value from 1 to 7.",
            ),
          ),
          (
            "dayOfYear",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the day of the year as a value from 1 to 366.",
            ),
          ),
          (
            "hours",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the hours value of the timestamp.",
            ),
          ),
          (
            "minutes",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the minutes value of the timestamp.",
            ),
          ),
          (
            "nanos",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the nanos value of the timestamp.",
            ),
          ),
          (
            "seconds",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the seconds value of the timestamp.",
            ),
          ),
          (
            "time",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Duration,
              "Get the duration value from the time portion of the timestamp.",
            ),
          ),
          (
            "toMillis",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the time in milliseconds since the epoch.",
            ),
          ),
        ]
      }
      FirebaseType::Duration => {
        vec![
          (
            "nanos",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the nanoseconds component of the duration.",
            ),
          ),
          (
            "seconds",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Get the seconds component of the duration.",
            ),
          ),
        ]
      }
      FirebaseType::Bytes => {
        vec![
          (
            "size",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::Integer,
              "Returns the number of bytes in a Bytes sequence.",
            ),
          ),
          (
            "toBase64",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::String,
              "Returns the Base64-encoded string corresponding to the provided Bytes sequence.",
            ),
          ),
          (
            "toHexString",
            vec![],
            FirebaseTypeInformation::new_documented(
              FirebaseType::String,
              "Returns the hexadecimal-encoded string corresponding to the provided Bytes sequence.",
            ),
          ),
        ]
      }
      FirebaseType::String => vec![
        (
          "lower",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "Returns a lowercase version of the input string.",
          ),
        ),
        (
          "size",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Integer,
            "Returns the number of characters in the string.",
          ),
        ),
        (
          "toUtf8",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Bytes,
            "Returns the UTF-8 byte encoding of a string.",
          ),
        ),
        (
          "trim",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "Returns a version of the string with leading and trailing spaces removed.",
          ),
        ),
        (
          "upper",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "Returns an uppercase version of the input string.",
          ),
        ),
      ],
      FirebaseType::List => vec![
        (
          "concat",
          vec![FunctionParameter::new(
            "list",
            FirebaseTypeInformation::new_documented(
              FirebaseType::List,
              FirebaseType::List.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::List,
            "Create a new list by adding the elements of another list to the end of this list.",
          ),
        ),
        (
          "hasAll",
          vec![FunctionParameter::new(
            "list",
            FirebaseTypeInformation::new_documented(
              FirebaseType::List,
              FirebaseType::List.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "Determine whether the list contains all elements in another list.",
          ),
        ),
        (
          "hasAny",
          vec![FunctionParameter::new(
            "list",
            FirebaseTypeInformation::new_documented(
              FirebaseType::List,
              FirebaseType::List.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "Determine whether the list contains any element in another list.",
          ),
        ),
        (
          "hasOnly",
          vec![FunctionParameter::new(
            "list",
            FirebaseTypeInformation::new_documented(
              FirebaseType::List,
              FirebaseType::List.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "Determine whether all elements in the list are present in another list.",
          ),
        ),
        (
          "join",
          vec![FunctionParameter::new(
            "separator",
            FirebaseTypeInformation::new_documented(
              FirebaseType::String,
              FirebaseType::String.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::String,
            "Join the elements in the list into a string, with a separator.",
          ),
        ),
        (
          "removeAll",
          vec![FunctionParameter::new(
            "list",
            FirebaseTypeInformation::new_documented(
              FirebaseType::List,
              FirebaseType::List.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::List,
            "Create a new list by removing the elements of another list from this list.",
          ),
        ),
        (
          "size",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Integer,
            "Get the number of values in the list.",
          ),
        ),
        (
          "toSet",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns a set containing all unique elements in the list.",
          ),
        ),
      ],
      FirebaseType::Map => vec![
        (
          "diff",
          vec![FunctionParameter::new(
            "map_to_compare",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Map,
              FirebaseType::Map.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::MapDiff,
            "Return a MapDiff representing the result of comparing the current Map to a comparison Map.",
          ),
        ),
        (
          "get",
          vec![
            FunctionParameter::new(
              "key",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Any,
                FirebaseType::Any.docstring(),
              ),
            ),
            FunctionParameter::new(
              "default_value",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Any,
                FirebaseType::Any.docstring(),
              ),
            ),
          ],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Any,
            "Returns the value associated with a given search key string, or a default value if the key is not found.",
          ),
        ),
        (
          "size",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Integer,
            "Get the number of entries in the map.",
          ),
        ),
        (
          "keys",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::List,
            "Get the list of keys in the map.",
          ),
        ),
        (
          "values",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::List,
            "Get the list of values in the map.",
          ),
        ),
      ],
      FirebaseType::MapDiff => vec![
        (
          "addedKeys",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns the set of keys that have been added.",
          ),
        ),
        (
          "affectedKeys",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns the set of keys that have been affected (added, removed, or changed).",
          ),
        ),
        (
          "changedKeys",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns the set of keys that have been changed.",
          ),
        ),
        (
          "unchangedKeys",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::List,
            "Returns the list of keys that remain unchanged.",
          ),
        ),
        (
          "removedKeys",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns the set of keys that have been removed.",
          ),
        ),
      ],
      FirebaseType::Path => vec![(
        "bind",
        vec![FunctionParameter::new(
          "map",
          FirebaseTypeInformation::new_documented(FirebaseType::Map, FirebaseType::Map.docstring()),
        )],
        FirebaseTypeInformation::new_documented(
          FirebaseType::Path,
          "Bind variable values to a path template, returning a new path.",
        ),
      )],
      FirebaseType::Boolean => vec![],
      FirebaseType::Float => vec![],
      FirebaseType::Integer => vec![],
      FirebaseType::Number => vec![],
      FirebaseType::Request => vec![],
      FirebaseType::Resource => vec![],
      FirebaseType::Set => vec![
        (
          "difference",
          vec![FunctionParameter::new(
            "set",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Set,
              FirebaseType::Set.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns a set that is the difference between this set and the comparison set. Contains elements in the comparison set that are not in this set.",
          ),
        ),
        (
          "hasAll",
          vec![FunctionParameter::new(
            "set",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Set,
              FirebaseType::Set.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "Test whether this set contains all of the items in the comparison set.",
          ),
        ),
        (
          "hasAny",
          vec![FunctionParameter::new(
            "set",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Set,
              FirebaseType::Set.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "Test whether this set contains any of the items in the comparison set.",
          ),
        ),
        (
          "hasOnly",
          vec![FunctionParameter::new(
            "set",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Set,
              FirebaseType::Set.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Boolean,
            "Test whether this set contains only the items in the comparison set.",
          ),
        ),
        (
          "intersection",
          vec![FunctionParameter::new(
            "set",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Set,
              FirebaseType::Set.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns a set that is the intersection between this set and the comparison set. Contains elements found in both sets.",
          ),
        ),
        (
          "union",
          vec![FunctionParameter::new(
            "set",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Set,
              FirebaseType::Set.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Set,
            "Returns a set that is the union of this set and the comparison set. Contains all elements from both sets.",
          ),
        ),
        (
          "size",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Integer,
            "Returns the size of the set.",
          ),
        ),
      ],
      FirebaseType::Null => vec![],
      FirebaseType::hashing_module => vec![
        (
          "crc32",
          vec![FunctionParameter::new(
            "bytes_or_string",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Any,
              FirebaseType::Any.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Bytes,
            "Compute CRC32 hash of the input bytes or string.",
          ),
        ),
        (
          "crc32c",
          vec![FunctionParameter::new(
            "bytes_or_string",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Any,
              FirebaseType::Any.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Bytes,
            "Compute CRC32C hash of the input bytes or string.",
          ),
        ),
        (
          "md5",
          vec![FunctionParameter::new(
            "bytes_or_string",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Any,
              FirebaseType::Any.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Bytes,
            "Compute MD5 hash of the input bytes or string.",
          ),
        ),
        (
          "sha256",
          vec![FunctionParameter::new(
            "bytes_or_string",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Any,
              FirebaseType::Any.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Bytes,
            "Compute SHA256 hash of the input bytes or string.",
          ),
        ),
      ],
      FirebaseType::latlng_module => vec![(
        "value",
        vec![
          FunctionParameter::new(
            "lat",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Float,
              FirebaseType::Float.docstring(),
            ),
          ),
          FunctionParameter::new(
            "lng",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Float,
              FirebaseType::Float.docstring(),
            ),
          ),
        ],
        FirebaseTypeInformation::new_documented(
          FirebaseType::LatLng,
          "Create a LatLng from latitude and longitude values.",
        ),
      )],
      FirebaseType::duration_module => vec![
        (
          "abs",
          vec![FunctionParameter::new(
            "duration",
            FirebaseTypeInformation::new_documented(
              FirebaseType::Duration,
              FirebaseType::Duration.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Duration,
            "Absolute value of a Duration.",
          ),
        ),
        (
          "time",
          vec![
            FunctionParameter::new(
              "hours",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Integer,
                FirebaseType::Integer.docstring(),
              ),
            ),
            FunctionParameter::new(
              "mins",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Integer,
                FirebaseType::Integer.docstring(),
              ),
            ),
            FunctionParameter::new(
              "secs",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Integer,
                FirebaseType::Integer.docstring(),
              ),
            ),
            FunctionParameter::new(
              "nanos",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Integer,
                FirebaseType::Integer.docstring(),
              ),
            ),
          ],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Duration,
            "Create a duration from hours, minutes, seconds, and nanoseconds.",
          ),
        ),
        (
          "static",
          vec![
            FunctionParameter::new(
              "magnitude",
              FirebaseTypeInformation::new_documented(
                FirebaseType::Integer,
                FirebaseType::Integer.docstring(),
              ),
            ),
            FunctionParameter::new(
              "unit",
              FirebaseTypeInformation::new_documented(
                FirebaseType::String,
                FirebaseType::String.docstring(),
              ),
            ),
          ],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Duration,
            "Create a duration from a magnitude and unit string.",
          ),
        ),
      ],
      FirebaseType::LatLng => vec![
        (
          "distance",
          vec![FunctionParameter::new(
            "other",
            FirebaseTypeInformation::new_documented(
              FirebaseType::LatLng,
              FirebaseType::LatLng.docstring(),
            ),
          )],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Integer,
            "Calculate the distance between this LatLng and another LatLng.",
          ),
        ),
        (
          "latitude",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Float,
            "Get the latitude value of the LatLng.",
          ),
        ),
        (
          "longitude",
          vec![],
          FirebaseTypeInformation::new_documented(
            FirebaseType::Float,
            "Get the longitude value of the LatLng.",
          ),
        ),
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
    }
  }
}

#[derive(Debug, Clone)]
pub enum VariableType {
  Variable,
  Module,
}

pub static GLOBAL_VARIABLES: LazyLock<[(&str, FirebaseTypeInformation, VariableType); 8]> =
  LazyLock::new(|| {
    [
      (
        "duration",
        FirebaseTypeInformation::new_documented(
          FirebaseType::duration_module,
          FirebaseType::duration_module.docstring(),
        ),
        VariableType::Module,
      ),
      (
        "hashing",
        FirebaseTypeInformation::new_documented(
          FirebaseType::hashing_module,
          FirebaseType::hashing_module.docstring(),
        ),
        VariableType::Module,
      ),
      (
        "latlng",
        FirebaseTypeInformation::new_documented(
          FirebaseType::latlng_module,
          FirebaseType::latlng_module.docstring(),
        ),
        VariableType::Module,
      ),
      (
        "math",
        FirebaseTypeInformation::new_documented(
          FirebaseType::math_module,
          FirebaseType::math_module.docstring(),
        ),
        VariableType::Module,
      ),
      (
        "timestamp",
        FirebaseTypeInformation::new_documented(
          FirebaseType::timestamp_module,
          FirebaseType::timestamp_module.docstring(),
        ),
        VariableType::Module,
      ),
      (
        "request",
        FirebaseTypeInformation::new_documented(
          FirebaseType::Request,
          FirebaseType::Request.docstring(),
        ),
        VariableType::Variable,
      ),
      (
        "resource",
        FirebaseTypeInformation::new_documented(
          FirebaseType::Resource,
          FirebaseType::Resource.docstring(),
        ),
        VariableType::Variable,
      ),
      (
        "database",
        FirebaseTypeInformation::new_documented(
          FirebaseType::String,
          FirebaseType::String.docstring(),
        ),
        VariableType::Variable,
      ),
    ]
  });

pub static GLOBAL_FUNCTIONS: LazyLock<[(&str, FirebaseTypeInformation); 6]> = LazyLock::new(|| {
  [
    (
      "get",
      FirebaseTypeInformation::new_documented(
        FirebaseType::Resource,
        FirebaseType::Resource.docstring(),
      ),
    ),
    (
      "path",
      FirebaseTypeInformation::new_documented(FirebaseType::Path, FirebaseType::Path.docstring()),
    ),
    (
      "getAfter",
      FirebaseTypeInformation::new_documented(
        FirebaseType::Resource,
        FirebaseType::Resource.docstring(),
      ),
    ),
    (
      "exists",
      FirebaseTypeInformation::new_documented(
        FirebaseType::Boolean,
        FirebaseType::Boolean.docstring(),
      ),
    ),
    (
      "existsAfer",
      FirebaseTypeInformation::new_documented(
        FirebaseType::Boolean,
        FirebaseType::Boolean.docstring(),
      ),
    ),
    (
      "debug",
      FirebaseTypeInformation::new_documented(
        FirebaseType::Boolean,
        FirebaseType::Boolean.docstring(),
      ),
    ),
  ]
});

pub static SPECIAL_KEYWORDS: [&str; 20] = [
  "allow", "if", "true", "false", "null", "in", "return", "function", "let", "var", "is", "read",
  "write", "get", "list", "create", "update", "delete", "service", "match",
];

pub fn namespace_reserved_function<'b>(name: &str) -> Option<FirebaseTypeInformation> {
  GLOBAL_FUNCTIONS
    .iter()
    .find(|(n, _)| *n == name)
    .map(|(_, t)| t.clone())
}

pub fn namespace_reserved_variable<'b>(name: &str) -> Option<FirebaseTypeInformation> {
  GLOBAL_VARIABLES
    .iter()
    .find(|(n, _, _)| *n == name)
    .map(|(_, t, _)| t.clone())
}

pub fn infer_function_type<'a>(
  obj_type: FirebaseType,
  fun_name: &'a str,
) -> Option<FirebaseTypeInformation> {
  obj_type
    .methods()
    .iter()
    .find(|f| f.0 == fun_name)
    .map(|to| to.2.clone())
}

pub fn infer_variable_type<'a>(
  obj_type: FirebaseType,
  var_name: &'a str,
) -> Option<FirebaseTypeInformation> {
  obj_type
    .properties()
    .iter()
    .find(|f| f.0 == var_name)
    .and_then(|to| Some(to.1.clone()))
}
