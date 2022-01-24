use std::collections::BTreeMap;

use anyhow::{Context, Result};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::mysql::MySqlTypeInfo;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo};
use sqlx::{encode::IsNull, Encode, Type};
use sqlx::{MySql, Postgres};

use synth_gen::prelude::*;
use synth_gen::value::{Token, Tokenizer};

use crate::compile::{Link, NamespaceCompiler};

use crate::schema::{ChronoValueAndFormat, Namespace};

pub mod prelude;
use prelude::*;

pub mod null;
pub use null::NullNode;

pub mod string;
pub use string::{Format, FormatArgs, RandFaker, RandomString, StringNode, Truncated, UuidGen};

pub mod date_time;
pub use date_time::{DateTimeNode, RandomDateTime};

pub mod number;
pub use number::{
    Incrementing, NumberNode, RandomF64, RandomI64, RandomU64, StandardFloatRangeStep,
    StandardIntRangeStep,
};

pub mod boolean;
pub use boolean::{BoolNode, RandomBool};

pub mod iter;
pub use iter::IterNode;

pub mod array;
pub use array::ArrayNode;

pub mod object;
pub use object::{KeyValueOrNothing, ObjectNode};

pub mod unique;
pub use unique::UniqueNode;

pub mod one_of;
pub(crate) mod series;

pub mod json;

use crate::graph::series::SeriesNode;
pub use one_of::OneOfNode;

pub type JustToken<T> = Tokenizer<Just<T>>;

pub type TokenOnce<T> = Tokenizer<Once<T>>;

pub type Valuize<G, T> =
    MapComplete<G, fn(Result<T, Error>) -> Result<Value, Error>, Result<Value, Error>>;

pub type Devaluize<G, T> =
    MapComplete<G, fn(Result<Value, Error>) -> Result<T, Error>, Result<T, Error>>;

pub type OwnedDevaluize<G, T> = Exhaust<Devaluize<G, T>>;

pub type StringGenerator = OwnedDevaluize<Box<Graph>, String>;

pub type SizeGenerator = OwnedDevaluize<Box<Graph>, u64>;

pub type OnceInfallible<G> = TryOnce<Infallible<G, Error>>;

macro_rules! derive_from {
    {
        #[$attr:meta]
        $vis:vis enum $id:ident {
            $( $variant:ident$(($ty:ty))?, )*
        }
    } => {
        #[$attr]
        $vis enum $id {
            $( $variant$(($ty))?, )*
        }

        impl $id {
            pub fn type_(&self) -> &'static str {
                match self {
                    $( Self::$variant(_) => stringify!($variant), )*
                }
            }
        }

        $(
            $(
                impl From<$ty> for $id {
                    fn from(value: $ty) -> Self {
                        Self::$variant(value)
                    }
                }
            )?
        )*
    };
}

pub fn value_from_ok<T>(value: Result<T, Error>) -> Result<Value, Error>
where
    Value: From<T>,
{
    value.map(Value::from)
}

pub fn value_from_ok_number<T>(value: Result<T, Error>) -> Result<Value, Error>
where
    Number: From<T>,
{
    value.map(|t| Number::from(t).into())
}

pub fn number_from_ok<T>(value: Result<Value, Error>) -> Result<T, Error>
where
    T: TryFrom<Number>,
    T::Error: std::error::Error,
{
    value.and_then(|v| v.try_into()).and_then(|n: Number| {
        n.try_into().map_err(|err| {
            failed_crate!(
                target: Release,
                "could not convert from value '{}': {}",
                n,
                err
            )
        })
    })
}

pub fn unsigned_from_ok(value: Result<Value, Error>) -> Result<u64, Error> {
    number_from_ok::<u64>(value.clone())
        .or_else(|_| number_from_ok::<u32>(value).map(|sm| sm as u64))
}

pub fn string_from_ok(value: Result<Value, Error>) -> Result<String, Error> {
    value.and_then(|n| match n {
        Value::String(s) => Ok(s),
        otherwise => Err(failed_crate!(
            target: Release,
            "could not get a string from a value '{}'",
            otherwise
        )),
    })
}

derive_from! {
    #[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
    pub enum Value {
        Null(()),
        Bool(bool),
        Number(Number),
        String(String),
        DateTime(ChronoValueAndFormat),
        Object(BTreeMap<String, Value>),
        Array(Vec<Value>),
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an object, array, string, number or boolean")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Bool(value))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(s.to_string()))
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Number(value.into()))
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut map = BTreeMap::new();

                while let Some((key, value)) = access.next_entry()? {
                    map.insert(key, value);
                }

                Ok(Value::Object(map))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut arr = Vec::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(value) = seq.next_element()? {
                    arr.push(value);
                }

                Ok(Value::Array(arr))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<String, Self::Error> {
        match value {
            Value::String(str) => Ok(str),
            Value::Number(num) => Ok(num.to_string()),
            Value::DateTime(date) => Ok(date.format_to_string()),
            otherwise => Err(failed_crate!(
                target: Release,
                "invalid type: expected 'String', found '{}'",
                otherwise.type_()
            )),
        }
    }
}

impl TryFrom<Value> for Number {
    type Error = Error;

    fn try_from(value: Value) -> Result<Number, Self::Error> {
        match value {
            Value::Number(num) => Ok(num),
            otherwise => Err(failed_crate!(
                target: Release,
                "invalid type: expected 'Number', found '{}'",
                otherwise.type_()
            )),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

/// Claim we are an unknown type since we don't have a reference to `self` to use.
impl Type<Postgres> for Value {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("unknown")
    }

    fn compatible(_ty: &PgTypeInfo) -> bool {
        unreachable!("This should never happen. Please reach out to https://github.com/getsynth/synth/issues if it does.")
    }
}

impl Type<MySql> for Value {
    fn type_info() -> MySqlTypeInfo {
        <serde_json::value::Value as Type<MySql>>::type_info()
    }

    fn compatible(_ty: &MySqlTypeInfo) -> bool {
        unreachable!("This should never happen. Please reach out to https://github.com/getsynth/synth/issues if it does.")
    }
}

impl Value {
    fn to_postgres_string(&self) -> String {
        match self {
            Self::Array(arr) => {
                let (typ, _) = self.get_postgres_type();
                let inner = arr
                    .iter()
                    .map(|v| v.to_postgres_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                if typ == "jsonb" {
                    format!("[{}]", inner)
                } else {
                    format!("{{{}}}", inner)
                }
            }
            Self::Null(_) => "NULL".to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Number(num) => match num {
                Number::F32(f32) => (*f32).to_string(),
                Number::F64(f64) => (*f64).to_string(),
                _ => num.to_string(),
            },
            Self::String(str) => format!("\"{}\"", str),
            Self::DateTime(date) => date.format_to_string(),
            Self::Object(_) => {
                serde_json::to_string(&json::synth_val_to_json(self.clone())).unwrap()
            }
        }
    }

    pub fn get_postgres_type(&self) -> (&'static str, usize) {
        let mut depth = 0;
        let mut typ = "";

        let mut current = Some(self);

        // Based on https://docs.rs/sqlx-core/0.5.9/sqlx_core/postgres/types/index.html
        while let Some(c) = current {
            let pair = match c {
                Value::Null(_) => (None, "unknown"),
                Value::Bool(_) => (None, "bool"),
                Value::Number(num) => match *num {
                    Number::I8(_) => (None, "char"),
                    Number::I16(_) => (None, "int2"),
                    Number::I32(_) => (None, "int4"),
                    Number::I64(_) => (None, "int8"),
                    Number::I128(_) => (None, "numeric"),
                    Number::U8(_) => (None, "char"),
                    Number::U16(_) => (None, "int2"),
                    Number::U32(_) => (None, "int4"),
                    Number::U64(_) => (None, "int8"),
                    Number::U128(_) => (None, "numeric"),
                    Number::F32(_) => (None, "float4"),
                    Number::F64(_) => (None, "float8"),
                },
                Value::String(_) => (None, "text"),
                Value::DateTime(ChronoValueAndFormat { value, .. }) => match value {
                    ChronoValue::NaiveDate(_) => (None, "date"),
                    ChronoValue::NaiveTime(_) => (None, "time"),
                    ChronoValue::NaiveDateTime(_) => (None, "timestamp"),
                    ChronoValue::DateTime(_) => (None, "timestamptz"),
                },
                Value::Object(_) => (None, "jsonb"),
                Value::Array(arr) => {
                    depth += 1;
                    if arr.is_empty() {
                        (None, "unknown")
                    } else {
                        (Some(&arr[0]), "")
                    }
                }
            };

            current = pair.0;
            typ = pair.1;
        }

        (typ, depth)
    }
}

impl Encode<'_, Postgres> for Value {
    fn produces(&self) -> Option<PgTypeInfo> {
        // Only arrays needs a special type
        match self {
            Value::Array(_) => Some(PgTypeInfo::with_name("text")),
            _ => None,
        }
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        match self {
            Value::Null(_) => IsNull::Yes,
            Value::Bool(b) => <bool as Encode<'_, Postgres>>::encode_by_ref(b, buf),
            Value::Number(num) => match *num {
                Number::I8(i) => <i8 as Encode<'_, Postgres>>::encode_by_ref(&i, buf),
                Number::I16(i) => <i16 as Encode<'_, Postgres>>::encode_by_ref(&i, buf),
                Number::I32(i) => <i32 as Encode<'_, Postgres>>::encode_by_ref(&i, buf),
                Number::I64(i) => <i64 as Encode<'_, Postgres>>::encode_by_ref(&i, buf),
                Number::I128(i) => {
                    <sqlx::types::Decimal as Encode<'_, Postgres>>::encode_by_ref(&i.into(), buf)
                }
                Number::U8(i) => <i8 as Encode<'_, Postgres>>::encode_by_ref(&(i as i8), buf),
                Number::U16(i) => <i16 as Encode<'_, Postgres>>::encode_by_ref(&(i as i16), buf),
                Number::U32(i) => <u32 as Encode<'_, Postgres>>::encode_by_ref(&i, buf),
                Number::U64(i) => <i64 as Encode<'_, Postgres>>::encode_by_ref(&(i as i64), buf),
                Number::U128(i) => {
                    <sqlx::types::Decimal as Encode<'_, Postgres>>::encode_by_ref(&i.into(), buf)
                }
                Number::F32(f) => <f32 as Encode<'_, Postgres>>::encode_by_ref(&f, buf),
                Number::F64(f) => <f64 as Encode<'_, Postgres>>::encode_by_ref(&f, buf),
            },
            Value::String(s) => <String as Encode<'_, Postgres>>::encode_by_ref(s, buf),
            Value::DateTime(ChronoValueAndFormat { value, .. }) => match value {
                ChronoValue::NaiveDate(nd) => {
                    <NaiveDate as Encode<'_, Postgres>>::encode_by_ref(nd, buf)
                }
                ChronoValue::NaiveTime(nt) => {
                    <NaiveTime as Encode<'_, Postgres>>::encode_by_ref(nt, buf)
                }
                ChronoValue::NaiveDateTime(ndt) => {
                    <NaiveDateTime as Encode<'_, Postgres>>::encode_by_ref(ndt, buf)
                }
                ChronoValue::DateTime(dt) => {
                    <DateTime<FixedOffset> as Encode<'_, Postgres>>::encode_by_ref(dt, buf)
                }
            },
            Value::Object(_) => <serde_json::Value as Encode<'_, Postgres>>::encode(
                json::synth_val_to_json(self.clone()),
                buf,
            ),
            Value::Array(_) => {
                let s = self.to_postgres_string();
                <String as Encode<'_, Postgres>>::encode_by_ref(&s, buf)
            } //TODO special-case for BYTEA
        }
    }
}

impl Encode<'_, MySql> for Value {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        match self {
            Value::Null(_) => IsNull::Yes,
            Value::Bool(b) => <bool as Encode<'_, MySql>>::encode_by_ref(b, buf),
            Value::Number(num) => match *num {
                Number::I8(i) => <i8 as Encode<'_, MySql>>::encode_by_ref(&i, buf),
                Number::I16(i) => <i16 as Encode<'_, MySql>>::encode_by_ref(&i, buf),
                Number::I32(i) => <i32 as Encode<'_, MySql>>::encode_by_ref(&i, buf),
                Number::I64(i) => <i64 as Encode<'_, MySql>>::encode_by_ref(&i, buf),
                Number::I128(i) => {
                    <sqlx::types::Decimal as Encode<'_, MySql>>::encode_by_ref(&i.into(), buf)
                }
                Number::U8(i) => <i8 as Encode<'_, MySql>>::encode_by_ref(&(i as i8), buf),
                Number::U16(i) => <i16 as Encode<'_, MySql>>::encode_by_ref(&(i as i16), buf),
                Number::U32(i) => <u32 as Encode<'_, MySql>>::encode_by_ref(&i, buf),
                Number::U64(i) => <i64 as Encode<'_, MySql>>::encode_by_ref(&(i as i64), buf),
                Number::U128(i) => {
                    <sqlx::types::Decimal as Encode<'_, MySql>>::encode_by_ref(&i.into(), buf)
                }
                Number::F32(f) => <f32 as Encode<'_, MySql>>::encode_by_ref(&f, buf),
                Number::F64(f) => <f64 as Encode<'_, MySql>>::encode_by_ref(&f, buf),
            },
            Value::String(s) => <String as Encode<'_, MySql>>::encode_by_ref(s, buf),
            Value::DateTime(ChronoValueAndFormat { value, .. }) => match value {
                ChronoValue::NaiveDate(nd) => {
                    <NaiveDate as Encode<'_, MySql>>::encode_by_ref(nd, buf)
                }
                ChronoValue::NaiveTime(nt) => {
                    <NaiveTime as Encode<'_, MySql>>::encode_by_ref(nt, buf)
                }
                ChronoValue::NaiveDateTime(ndt) => {
                    <NaiveDateTime as Encode<'_, MySql>>::encode_by_ref(ndt, buf)
                }
                ChronoValue::DateTime(dt) => <DateTime<Utc> as Encode<'_, MySql>>::encode_by_ref(
                    &dt.with_timezone(&Utc),
                    buf,
                ),
            },
            Value::Object(_) => <serde_json::Value as Encode<'_, MySql>>::encode(
                json::synth_val_to_json(self.clone()),
                buf,
            ),
            Value::Array(_arr) => todo!(), //<Vec<Value> as Encode<'_, MySql>>::encode_by_ref(arr, buf), //TODO special-case for u8 arrays?
        }
    }

    fn produces(&self) -> Option<MySqlTypeInfo> {
        Some(match self {
            Value::Null(_) => {
                return <serde_json::Value as Encode<'_, MySql>>::produces(&serde_json::Value::Null)
            }
            Value::Bool(_) => <bool as Type<MySql>>::type_info(),
            Value::Number(num) => match num {
                Number::I8(_) => <i8 as Type<MySql>>::type_info(),
                Number::I16(_) => <i16 as Type<MySql>>::type_info(),
                Number::I32(_) => <i32 as Type<MySql>>::type_info(),
                Number::I64(_) => <i64 as Type<MySql>>::type_info(),
                Number::I128(_) => <sqlx::types::Decimal as Type<MySql>>::type_info(),
                Number::U8(_) => <u8 as Type<MySql>>::type_info(),
                Number::U16(_) => <u16 as Type<MySql>>::type_info(),
                Number::U32(_) => <u32 as Type<MySql>>::type_info(),
                Number::U64(_) => <u64 as Type<MySql>>::type_info(),
                Number::U128(_) => <sqlx::types::Decimal as Type<MySql>>::type_info(),
                Number::F32(_) => <f32 as Type<MySql>>::type_info(),
                Number::F64(_) => <f64 as Type<MySql>>::type_info(),
            },
            Value::DateTime(ChronoValueAndFormat { value, .. }) => match value {
                ChronoValue::NaiveDate(_) => <NaiveDate as Type<MySql>>::type_info(),
                ChronoValue::NaiveTime(_) => <NaiveTime as Type<MySql>>::type_info(),
                ChronoValue::NaiveDateTime(_) => <NaiveDateTime as Type<MySql>>::type_info(),
                ChronoValue::DateTime(_) => <DateTime<Utc> as Type<MySql>>::type_info(),
            },
            Value::String(_) => <String as Type<MySql>>::type_info(),
            Value::Object(_) => return None, //TODO: Use JSON here?
            Value::Array(elems) => {
                if elems.is_empty() {
                    return None;
                } else if let Value::Number(Number::U8(_) | Number::I8(_)) = elems[0] {
                    <Vec<u8> as Type<MySql>>::type_info()
                } else {
                    return None; //TODO: other variants that would make sense?
                }
            }
        })
    }
}

#[allow(unused)]
impl Value {
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn is_number(&self) -> bool {
        self.as_number().is_some()
    }

    pub fn is_string(&self) -> bool {
        self.as_string().is_some()
    }

    pub fn is_datetime(&self) -> bool {
        self.as_datetime().is_some()
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null(()) => Some(()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match *self {
            Value::Bool(ref bool) => Some(bool),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&Number> {
        match *self {
            Value::Number(ref number) => Some(number),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match *self {
            Value::String(ref string) => Some(string),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<&ChronoValue> {
        match *self {
            Value::DateTime(ref chrono_value) => Some(&chrono_value.value),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match *self {
            Value::Bool(ref mut bool) => Some(bool),
            _ => None,
        }
    }

    pub fn as_number_mut(&mut self) -> Option<&mut Number> {
        match *self {
            Value::Number(ref mut number) => Some(number),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match *self {
            Value::String(ref mut string) => Some(string),
            _ => None,
        }
    }

    pub fn as_datetime_mut(&mut self) -> Option<&mut ChronoValue> {
        match *self {
            Value::DateTime(ChronoValueAndFormat {
                value: ref mut chrono_value,
                ..
            }) => Some(chrono_value),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match *self {
            Value::Object(ref mut map) => Some(map),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut vec) => Some(vec),
            _ => None,
        }
    }
}

pub struct IntoCompleted<G> {
    inner: G,
    complete: bool,
}

impl<G> IntoCompleted<G> {
    pub fn wrap(inner: G) -> Self {
        Self {
            inner,
            complete: false,
        }
    }
}

impl<G> Generator for IntoCompleted<G>
where
    G: Generator<Yield = Token>,
    G::Return: GeneratorResult,
    <G::Return as GeneratorResult>::Err: IntoToken,
{
    type Yield = Token;
    type Return = ();

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        if std::mem::replace(&mut self.complete, false) {
            GeneratorState::Complete(())
        } else {
            match self.inner.next(rng) {
                GeneratorState::Yielded(y) => GeneratorState::Yielded(y),
                GeneratorState::Complete(r) => match r.into_result() {
                    Ok(_) => {
                        self.complete = true;
                        self.next(rng)
                    }
                    Err(r_err) => {
                        self.complete = true;
                        GeneratorState::Yielded(r_err.into_token())
                    }
                },
            }
        }
    }
}

derive_generator!(
    yield Token,
    return Result<Value, Error>,
    pub enum Graph {
        Null(NullNode),
        Bool(BoolNode),
        Number(NumberNode),
        String(StringNode),
        DateTime(DateTimeNode),
        Object(ObjectNode),
        Array(ArrayNode),
        OneOf(OneOfNode),
        Series(SeriesNode),
        Unique(UniqueNode),
        Link(Box<LinkNode>),
        Hidden(Box<Graph>),
        Iter(IterNode),
    }
);

impl Graph {
    pub fn into_string(self) -> StringGenerator {
        Box::new(self)
            .map_complete(string_from_ok as fn(Result<Value, Error>) -> Result<String, Error>)
            .exhaust()
    }

    pub fn into_size(self) -> SizeGenerator {
        Box::new(self)
            .map_complete(unsigned_from_ok as fn(Result<Value, Error>) -> Result<u64, Error>)
            .exhaust()
    }
}

enum LinkNodeState {
    YieldFrom,
    Yield(Token),
    Return(Value),
}

pub struct LinkNode(Link<Graph, Token, Result<Value, Error>>, LinkNodeState);

impl Generator for Box<LinkNode> {
    type Yield = Token;
    type Return = Result<Value, Error>;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        match std::mem::replace(&mut (*self).1, LinkNodeState::YieldFrom) {
            LinkNodeState::YieldFrom => match self.0.next(rng) {
                GeneratorState::Yielded(y) => GeneratorState::Yielded(y),
                GeneratorState::Complete(Some(r)) => GeneratorState::Complete(r),
                GeneratorState::Complete(None) => {
                    (*self).1 = LinkNodeState::Yield(Token::Primitive(Primitive::Null(())));
                    self.next(rng)
                }
            },
            LinkNodeState::Yield(token) => {
                (*self).1 = LinkNodeState::Return(Value::Null(()));
                GeneratorState::Yielded(token)
            }
            LinkNodeState::Return(value) => GeneratorState::Complete(Ok(value)),
        }
    }
}

impl crate::compile::FromLink for Graph {
    type Yield = Token;

    type Return = Result<Value, Error>;

    fn from_link(link: Link<Self, Self::Yield, Self::Return>) -> Self {
        Self::Link(Box::new(LinkNode(link, LinkNodeState::YieldFrom)))
    }
}

pub type BoxedGraph = Box<Graph>;

impl Generator for Box<Graph> {
    type Yield = <Graph as Generator>::Yield;

    type Return = <Graph as Generator>::Return;

    fn next<R: Rng>(&mut self, rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        <Graph as Generator>::next(self, rng)
    }
}

impl Graph {
    pub fn null() -> Self {
        Graph::Null(
            ().yield_token()
                .infallible()
                .map_complete(value_from_ok::<()>),
        )
    }

    pub fn from_namespace(ns: &Namespace) -> Result<Self> {
        NamespaceCompiler::new(ns)
            .compile()
            .context("cannot compile the namespace")
    }

    pub fn from_content(content: &Content) -> Result<Self> {
        NamespaceCompiler::new_flat(content)
            .compile()
            .context("cannot compile the schema")
    }

    pub fn iter_ordered(&self) -> Option<impl Iterator<Item = &str>> {
        match self {
            Self::Link(box LinkNode(link, _)) => Some(link.iter_order()?),
            _ => None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::BTreeSet;

    use rand::{thread_rng, SeedableRng};

    use super::*;
    use crate::schema::ChronoValueFormatter;

    use crate::schema::tests::USER_NAMESPACE;

    #[test]
    fn schema_to_generator() {
        let schema: Namespace = schema!({
            "type": "object",
            "users": {
                "type": "array",
                "length": 10,
                "content": {
                    "type": "object",
                    "id" : {
                        "type" : "number",
                        "subtype" : "u64",
                        "id" : {
                            "start_at" : 100
                        }
                    },
                    "is_active": {
                        "type": "bool",
                        "frequency": 0.2
                    },
                    "username": {
                        "type": "string",
                        "unique": true,
                        "truncated": {
                            "content": {
                                "type": "string",
                                "pattern": "[a-zA-Z0-9]{0, 255}"
                            },
                            "length": {
                                "type": "number",
                                "subtype": "u64",
                                "range": {
                                    "low": 2,
                                    "high": 6,
                                    "step": 1
                                }
                            }
                        }
                    },
                    "bank_country": {
                        "type": "string",
                        "pattern": "(GB|ES)"
                    },
                    "num_logins": {
                        "type": "number",
                        "subtype": "u64",
                        "range": {
                            "high": 100,
                            "low": 0,
                            "step": 1
                        }
                    },
                    "currency": {
                        "type": "string",
                        "pattern": "(USD|GBP)"
                    },
                    "credit_card": {
                        "type": "string",
                        "faker": {
                            "generator": "credit_card"
                        }
                    },
                    "formatted_username": {
                        "type": "string",
                        "format": {
                            "format": "my username is {name} and I trade in {currency}",
                            "arguments": {
                                "name": "@users.content.username",
                                "currency": "@users.content.currency"
                            }
                        }
                    },
                    "created_at_date": {
                        "type": "date_time",
                        "format": "%Y/%m/%d"
                    },
                    "created_at_time": {
                        "type": "date_time",
                        "format": "%H:%M:%S"
                    },
                    "last_login_at": {
                        "type": "date_time",
                        "format": "%Y-%m-%dT%H:%M:%S%z",
                        "begin": "2020-01-01T00:00:00+0000"
                    },
                    "maybe_an_email": {
                        "optional": true,
                        "unique": true,
                        "type": "string",
                        "faker": {
                            "generator": "safe_email"
                        }
                    },
                    "num_logins_again": "@users.content.num_logins",
                    "a_large_number": {
                        "type": "number",
                        "range": {}
                    },
                    "constant_id": "42",
                    "created_at_year": {
                        "type": "string",
                        "sliced": {
                            "content": {
                                "type": "string",
                                "format": {
                                    "format": "{date} ",
                                    "arguments": {
                                        "date": "@users.content.created_at_date"
                                    }
                                }
                            },
                            "slice": "0:4"
                        }
                    }
                }
            },
            "transactions": {
                "type": "array",
                "length": 100,
                "content": {
                    "type": "object",
                    "username": {
                        "type": "same_as",
                        "ref": "users.content.username"
                    },
                    "currency": {
                        "type": "same_as",
                        "ref": "users.content.currency"
                    },
                    "timestamp": {
                        "type": "date_time",
                        "format": "%Y-%m-%dT%H:%M:%S%z",
                        "begin": "2020-01-01T00:00:00+0000"
                    },
                    "amount": {
                        "type": "number",
                        "subtype": "f64",
                        "range": {
                            "high": 10000,
                            "low": 0,
                            "step": 0.1
                        }
                    },
                    "serialized_nonce": {
                        "type" : "string",
                        "serialized" : {
                            "serializer" : "json",
                            "content" : {
                                "type" : "object",
                                "nonce" : {
                                    "type" : "string",
                                    "pattern" : "[A-Z a-z 0-9]+",
                                }
                            }
                        }
                    },
                }
            }
        })
        .into_namespace()
        .unwrap();

        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        let mut model = Graph::from_namespace(&schema)
            .unwrap()
            .inspect(|yielded| {
                println!("{:?}", yielded);
            })
            .aggregate();

        #[derive(Deserialize, Debug)]
        struct SampleData {
            users: Vec<SampleUserData>,
            transactions: Vec<SampleTransactionData>,
        }

        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct SampleTransactionData {
            username: String,
            currency: String,
            timestamp: String,
            amount: f64,
            serialized_nonce: String,
        }

        #[allow(dead_code)]
        #[derive(Deserialize, Debug)]
        struct SampleUserData {
            id: u64,
            num_logins: u64,
            username: String,
            bank_country: String,
            currency: String,
            credit_card: String,
            maybe_an_email: Option<String>,
            formatted_username: String,
            is_active: bool,
            created_at_date: String,
            created_at_time: String,
            last_login_at: String,
            num_logins_again: u64,
            constant_id: String,
            created_at_year: String,
        }

        for _ in 0..100 {
            let ser = OwnedSerializable::new(model.try_next_yielded(&mut rng).unwrap());
            let generated_str = serde_json::to_string_pretty(&ser).unwrap();

            let sample_data = serde_json::from_str::<SampleData>(&generated_str).unwrap();

            let mut all_users = BTreeSet::new();
            let mut all_emails = BTreeSet::new();
            let mut currencies = BTreeMap::new();

            for user in &sample_data.users {
                assert_eq!(user.num_logins, user.num_logins_again);
                println!("bank_country={}", user.bank_country);
                assert!(user.bank_country == "GB" || user.bank_country == "ES");
                assert!(user.id >= 100);
                assert!(user.username.len() <= 5);
                assert_eq!(user.constant_id, "42");
                assert_eq!(user.created_at_year, user.created_at_date[0..4]);

                all_users.insert(user.username.as_str());

                if let Some(email) = &user.maybe_an_email {
                    assert!(all_emails.insert(email.as_str()))
                }

                currencies.insert(user.username.as_str(), user.currency.as_str());
                ChronoValueFormatter::new("%Y/%m/%d")
                    .parse(&user.created_at_date)
                    .unwrap();

                ChronoValueFormatter::new("%H:%M:%S")
                    .parse(&user.created_at_time)
                    .unwrap();

                ChronoValueFormatter::new("%Y-%m-%dT%H:%M:%S%z")
                    .parse(&user.last_login_at)
                    .unwrap();
            }

            assert_eq!(all_users.len(), 10);

            println!("currencies={:?}", currencies);

            let mut counts = BTreeMap::new();
            for transaction in &sample_data.transactions {
                println!("transaction={:?}", transaction);
                assert!(all_users.contains(transaction.username.as_str()));
                println!(
                    "username={}, amount={}",
                    transaction.username, transaction.amount
                );
                assert_eq!(
                    transaction.currency,
                    *currencies.get(transaction.username.as_str()).unwrap()
                );
                *counts.entry(transaction.username.as_str()).or_insert(0) += 1;

                assert!(serde_json::to_value(&transaction.serialized_nonce).is_ok());
            }

            for value in counts.values() {
                assert_eq!(*value, 10);
            }
        }
    }

    #[test]
    fn test_schema_compiles_and_generates() {
        let mut model = Graph::from_namespace(&USER_NAMESPACE).unwrap().aggregate();
        let mut rng = rand::thread_rng();
        let ser = OwnedSerializable::new(model.try_next_yielded(&mut rng).unwrap());
        serde_json::to_string_pretty(&ser).unwrap();
    }

    #[test]
    fn range_distribution_u64() {
        let range = RangeStep::<u64>::new(15, 40, 5);
        let dist = StandardIntRangeStep::<u64, u64>::try_from_range(range).unwrap();
        let mut rng = thread_rng();
        for _ in 1..100 {
            match dist.sample(&mut rng) {
                15 | 20 | 25 | 30 | 35 => {}
                n => panic!("Generated '{}' which should not happen", n),
            }
        }
    }

    #[test]
    fn range_distribution_i64() {
        let range = RangeStep::<i64>::new(-10, 10, 5);
        let dist = StandardIntRangeStep::<u64, i128>::try_from_range(range).unwrap();
        let mut rng = thread_rng();
        for _ in 1..100 {
            match dist.sample(&mut rng) {
                -10 | -5 | 0 | 5 => {}
                n => panic!("Generated '{}' which should not happen", n),
            }
        }
    }

    #[test]
    fn range_distribution_f64() {
        let range = RangeStep::new(-2.5, 1.0, 1.5);
        let dist = StandardFloatRangeStep::<f64>::try_from_range(range).unwrap();
        let mut rng = thread_rng();
        let error_margin = f64::EPSILON;
        for _ in 1..1000 {
            let sample: f64 = dist.sample(&mut rng);
            // This is equal to:
            //     if sample != -2.5 && sample != -1.0 && sample != 0.5 { /* ... */ }
            // but it protects against imprecisions in floating point comparisons.
            if (sample - -2.5).abs() > error_margin
                && (sample - -1.0).abs() > error_margin
                && (sample - 0.5).abs() > error_margin
            {
                panic!("Generated '{}' which should not happen", sample)
            }
        }
    }

    #[test]
    fn range_distribution_constant() {
        let range = RangeStep::<u64>::new(10, 10, 5);
        assert!(StandardIntRangeStep::<u64, u64>::try_from_range(range).is_err())
    }

    #[test]
    fn range_distribution_step_larger_than_delta() {
        let range = RangeStep::<u64>::new(10, 15, 10);
        let dist = StandardIntRangeStep::<u64, u64>::try_from_range(range).unwrap();
        let mut rng = thread_rng();
        for _ in 1..100 {
            match dist.sample(&mut rng) {
                10 => {}
                n => panic!("Generated '{}' which should not happen", n),
            }
        }
    }

    #[test]
    fn range_distribution_step_is_delta() {
        let range = RangeStep::<u64>::new(10, 15, 5);
        let dist = StandardIntRangeStep::<u64, u64>::try_from_range(range).unwrap();
        let mut rng = thread_rng();
        for _ in 1..100 {
            match dist.sample(&mut rng) {
                10 => {}
                n => panic!("Generated '{}' which should not happen", n),
            }
        }
    }
}
