use std::hash::{Hash, Hasher};

use super::prelude::*;
use super::Categorical;
use crate::graph::string::{Constant, Serialized, Sliced};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum StringContent {
    Pattern(RegexContent),
    Faker(FakerContent),
    Categorical(Categorical<String>),
    Serialized(SerializedContent),
    Uuid(Uuid),
    Truncated(TruncatedContent),
    Sliced(SlicedContent),
    Format(FormatContent),
    Constant(ConstantContent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct Uuid;

impl StringContent {
    pub fn kind(&self) -> String {
        match self {
            Self::Pattern(_) => "pattern".to_string(),
            Self::Faker(faker) => faker.kind(),
            Self::Categorical(_) => "categorical".to_string(),
            Self::Serialized(_) => "serialized".to_string(),
            Self::Uuid(_) => "uuid".to_string(),
            Self::Truncated(_) => "truncated".to_string(),
            Self::Sliced(_) => "sliced".to_string(),
            Self::Constant(_) => "constant".to_string(),
            Self::Format(_) => "format".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegexContent(String, RandRegex);

impl Display for RegexContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for RegexContent {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl RegexContent {
    pub fn to_regex(&self) -> RandRegex {
        self.1.clone()
    }

    pub fn pattern(pattern: String) -> Result<Self, RegexError> {
        Self::compile(pattern, 1)
    }

    pub fn compile(pattern: String, max_repeat: u32) -> Result<Self, RegexError> {
        let rand_regex = RandRegex::compile(pattern.as_str(), max_repeat)?;
        Ok(Self(pattern, rand_regex))
    }
}

impl<'de> Deserialize<'de> for RegexContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RegexVisitor;
        impl<'de> Visitor<'de> for RegexVisitor {
            type Value = RegexContent;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a string")
            }
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(s.to_string())
            }
            fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let rand_regex = RandRegex::compile(s.as_str(), 32).map_err(|e| {
                    let msg = format!("bad regex: {}", e);
                    E::custom(msg)
                })?;
                Ok(RegexContent(s, rand_regex))
            }
        }
        deserializer.deserialize_string(RegexVisitor)
    }
}

impl Serialize for RegexContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl Default for RegexContent {
    fn default() -> Self {
        let pattern = "[a-zA-Z0-9]*".to_string();
        RegexContent::compile(pattern, 32).unwrap()
    }
}

impl Hash for RegexContent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Default for StringContent {
    fn default() -> Self {
        Self::Pattern(RegexContent::default())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FakerContentArgument(Value);

impl FakerContentArgument {
    pub fn as_inner(&self) -> &Value {
        &self.0
    }
}

impl<'de> Deserialize<'de> for FakerContentArgument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match &value {
            Value::Number(_) |
            Value::String(_) |
            Value::Bool(_) => Ok(Self(value)),
            _ => {
                Err(D::Error::custom("invalid argument for a faker generator: can only be of a primitive type (i.e. one of string, number or boolean)"))
            }
        }
    }
}

impl Serialize for FakerContentArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Hash)]
pub struct FakerContent {
    pub generator: String,
    /// deprecated: Use FakerArgs::locale instead
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locales: Vec<String>,
    #[serde(flatten)]
    pub args: crate::graph::string::FakerArgs,
}

impl FakerContent {
    fn kind(&self) -> String {
        self.generator.to_string()
    }
}

impl<'de> Deserialize<'de> for FakerContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Generator,
            Locales,
            #[serde(other)]
            Unknown,
        }
        struct FakerVisitor;

        impl<'de> Visitor<'de> for FakerVisitor {
            type Value = FakerContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("'generator'")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut generator = None;
                let mut args = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Generator => {
                            if generator.is_some() {
                                return Err(A::Error::duplicate_field("generator"));
                            }
                            generator = Some(map.next_value()?);
                        }
                        Field::Locales => {
                            if args.is_some() {
                                return Err(A::Error::duplicate_field("locales"));
                            }
                            args = Some(map.next_value()?);
                        }
                        Field::Unknown => {}
                    }
                }
                let generator = generator.ok_or_else(|| A::Error::missing_field("generator"))?;
                let args = args.unwrap_or_default();
                Ok(FakerContent {
                    generator,
                    args,
                    locales: Vec::new(),
                })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(E::custom(format!("`faker` is expected to have a `generator` field. Try '\"faker\": {{\"generator\": \"{}\"}}'", v)))
            }
        }

        deserializer.deserialize_any(FakerVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "serializer")]
pub enum SerializedContent {
    Json(JsonContent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub struct TruncatedContent {
    content: Box<Content>,
    length: Box<Content>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub struct SlicedContent {
    content: Box<Content>,
    slice: Box<Content>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct ConstantContent(String);

impl From<String> for ConstantContent {
    fn from(s: String) -> Self {
        ConstantContent(s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct FormatContent {
    format: String,
    pub arguments: HashMap<String, Content>,
}

impl Hash for FormatContent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.format.hash(state);

        for (key, value) in &self.arguments {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl PartialEq for FormatContent {
    fn eq(&self, other: &FormatContent) -> bool {
        self.format == other.format && self.arguments == other.arguments
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash)]
pub struct JsonContent {
    content: Box<Content>,
}

impl Compile for StringContent {
    fn compile<'a, C: Compiler<'a>>(&'a self, mut compiler: C) -> Result<Graph> {
        let string_node = match self {
            StringContent::Format(FormatContent { format, arguments }) => {
                let args = FormatArgs {
                    named: arguments
                        .iter()
                        .map(|(name, value)| Ok((name.to_string(), compiler.build(name, value)?)))
                        .collect::<Result<_>>()?,
                    ..Default::default()
                };
                RandomString::from(Format::new(format, args)).into()
            }
            StringContent::Pattern(pattern) => RandomString::from(pattern.to_regex()).into(),
            StringContent::Faker(FakerContent {
                generator,
                args,
                locales: _, // to combine locales from the 'locales' field and the args::locales,
                            // we should impl Hash on locale and then put them in a Set
            }) => RandomString::from(RandFaker::new(generator.clone(), args.clone())?).into(),
            StringContent::Categorical(cat) => RandomString::from(cat.clone()).into(),
            StringContent::Serialized(sc) => match sc {
                SerializedContent::Json(serialized_json_content) => {
                    let inner = serialized_json_content.content.compile(compiler)?;
                    RandomString::from(Serialized::new_json(inner)).into()
                }
            },
            StringContent::Truncated(TruncatedContent {
                box length,
                box content,
            }) => {
                let content = compiler.build("content", content)?.into_string();
                let length = compiler.build("length", length)?.into_size();
                RandomString::from(Truncated::new(content, length)).into()
            }
            StringContent::Sliced(SlicedContent {
                box slice,
                box content,
            }) => {
                let content = compiler.build("content", content)?.into_string();
                let slice = compiler.build("slice", slice)?.into_string();
                RandomString::from(Sliced::new(content, slice)).into()
            }
            StringContent::Constant(ConstantContent(s)) => {
                RandomString::from(Constant(s.into())).into()
            }
            StringContent::Uuid(_uuid) => RandomString::from(UuidGen {}).into(),
        };
        Ok(Graph::String(string_node))
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::content::Content;

    #[test]
    #[should_panic(
        expected = "`faker` is expected to have a `generator` field. Try '\\\"faker\\\": {\\\"generator\\\": \\\"lastname\\\"}'"
    )]
    fn faker_missing_generator() {
        let _schema: Content = schema!({
            "type": "string",
            "faker": "lastname"
        });
    }
}
