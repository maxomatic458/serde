// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

#[macro_use]
extern crate serde_derive;

extern crate serde;
use std::collections::HashMap;
use self::serde::{Deserialize, Deserializer, Serialize, Serializer};
use self::serde::de::{self, Unexpected};

extern crate serde_test;
use self::serde_test::{assert_de_tokens, assert_de_tokens_error, assert_ser_tokens,
                       assert_ser_tokens_error, assert_tokens, Token};

trait MyDefault: Sized {
    fn my_default() -> Self;
}

trait ShouldSkip: Sized {
    fn should_skip(&self) -> bool;
}

trait SerializeWith: Sized {
    fn serialize_with<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

trait DeserializeWith: Sized {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl MyDefault for i32 {
    fn my_default() -> Self {
        123
    }
}

impl ShouldSkip for i32 {
    fn should_skip(&self) -> bool {
        *self == 123
    }
}

impl SerializeWith for i32 {
    fn serialize_with<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *self == 123 {
            true.serialize(ser)
        } else {
            false.serialize(ser)
        }
    }
}

impl DeserializeWith for i32 {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if try!(Deserialize::deserialize(de)) {
            Ok(123)
        } else {
            Ok(2)
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DefaultStruct<A, B, C, D, E>
where
    C: MyDefault,
    E: MyDefault,
{
    a1: A,
    #[serde(default)]
    a2: B,
    #[serde(default = "MyDefault::my_default")]
    a3: C,
    #[serde(skip_deserializing)]
    a4: D,
    #[serde(skip_deserializing, default = "MyDefault::my_default")]
    a5: E,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CollectOther {
    a: u32,
    b: u32,
    #[serde(flatten)]
    extra: HashMap<String, u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FlattenStructEnumWrapper {
    #[serde(flatten)]
    data: FlattenStructEnum,
    #[serde(flatten)]
    extra: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FlattenStructEnum {
    InsertInteger {
        index: u32,
        value: u32
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FlattenStructTagContentEnumWrapper {
    outer: u32,
    #[serde(flatten)]
    data: FlattenStructTagContentEnumNewtype,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FlattenStructTagContentEnumNewtype(pub FlattenStructTagContentEnum);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
enum FlattenStructTagContentEnum {
    InsertInteger {
        index: u32,
        value: u32
    },
    NewtypeVariant(FlattenStructTagContentEnumNewtypeVariant),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FlattenStructTagContentEnumNewtypeVariant {
    value: u32,
}

#[test]
fn test_default_struct() {
    assert_de_tokens(
        &DefaultStruct {
            a1: 1,
            a2: 2,
            a3: 3,
            a4: 0,
            a5: 123,
        },
        &[
            Token::Struct {
                name: "DefaultStruct",
                len: 3,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("a2"),
            Token::I32(2),
            Token::Str("a3"),
            Token::I32(3),
            Token::Str("a4"),
            Token::I32(4),
            Token::Str("a5"),
            Token::I32(5),
            Token::StructEnd,
        ],
    );

    assert_de_tokens(
        &DefaultStruct {
            a1: 1,
            a2: 0,
            a3: 123,
            a4: 0,
            a5: 123,
        },
        &[
            Token::Struct {
                name: "DefaultStruct",
                len: 3,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum DefaultEnum<A, B, C, D, E>
where
    C: MyDefault,
    E: MyDefault,
{
    Struct {
        a1: A,
        #[serde(default)]
        a2: B,
        #[serde(default = "MyDefault::my_default")]
        a3: C,
        #[serde(skip_deserializing)]
        a4: D,
        #[serde(skip_deserializing, default = "MyDefault::my_default")]
        a5: E,
    },
}

#[test]
fn test_default_enum() {
    assert_de_tokens(
        &DefaultEnum::Struct {
            a1: 1,
            a2: 2,
            a3: 3,
            a4: 0,
            a5: 123,
        },
        &[
            Token::StructVariant {
                name: "DefaultEnum",
                variant: "Struct",
                len: 3,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("a2"),
            Token::I32(2),
            Token::Str("a3"),
            Token::I32(3),
            Token::Str("a4"),
            Token::I32(4),
            Token::Str("a5"),
            Token::I32(5),
            Token::StructVariantEnd,
        ],
    );

    assert_de_tokens(
        &DefaultEnum::Struct {
            a1: 1,
            a2: 0,
            a3: 123,
            a4: 0,
            a5: 123,
        },
        &[
            Token::StructVariant {
                name: "DefaultEnum",
                variant: "Struct",
                len: 3,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::StructVariantEnd,
        ],
    );
}

// Does not implement std::default::Default.
#[derive(Debug, PartialEq, Deserialize)]
struct NoStdDefault(i8);

impl MyDefault for NoStdDefault {
    fn my_default() -> Self {
        NoStdDefault(123)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct ContainsNoStdDefault<A: MyDefault> {
    #[serde(default = "MyDefault::my_default")]
    a: A,
}

// Tests that a struct field does not need to implement std::default::Default if
// it is annotated with `default=...`.
#[test]
fn test_no_std_default() {
    assert_de_tokens(
        &ContainsNoStdDefault {
            a: NoStdDefault(123),
        },
        &[
            Token::Struct {
                name: "ContainsNoStdDefault",
                len: 1,
            },
            Token::StructEnd,
        ],
    );

    assert_de_tokens(
        &ContainsNoStdDefault { a: NoStdDefault(8) },
        &[
            Token::Struct {
                name: "ContainsNoStdDefault",
                len: 1,
            },
            Token::Str("a"),
            Token::NewtypeStruct {
                name: "NoStdDefault",
            },
            Token::I8(8),
            Token::StructEnd,
        ],
    );
}

// Does not implement Deserialize.
#[derive(Debug, PartialEq)]
struct NotDeserializeStruct(i8);

impl Default for NotDeserializeStruct {
    fn default() -> Self {
        NotDeserializeStruct(123)
    }
}

impl DeserializeWith for NotDeserializeStruct {
    fn deserialize_with<'de, D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        panic!()
    }
}

// Does not implement Deserialize.
#[derive(Debug, PartialEq)]
enum NotDeserializeEnum {
    Trouble,
}

impl MyDefault for NotDeserializeEnum {
    fn my_default() -> Self {
        NotDeserializeEnum::Trouble
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct ContainsNotDeserialize<A, B, C: DeserializeWith, E: MyDefault> {
    #[serde(skip_deserializing)]
    a: A,
    #[serde(skip_deserializing, default)]
    b: B,
    #[serde(deserialize_with = "DeserializeWith::deserialize_with", default)]
    c: C,
    #[serde(skip_deserializing, default = "MyDefault::my_default")]
    e: E,
}

// Tests that a struct field does not need to implement Deserialize if it is
// annotated with skip_deserializing, whether using the std Default or a
// custom default.
#[test]
fn test_elt_not_deserialize() {
    assert_de_tokens(
        &ContainsNotDeserialize {
            a: NotDeserializeStruct(123),
            b: NotDeserializeStruct(123),
            c: NotDeserializeStruct(123),
            e: NotDeserializeEnum::Trouble,
        },
        &[
            Token::Struct {
                name: "ContainsNotDeserialize",
                len: 1,
            },
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DenyUnknown {
    a1: i32,
}

#[test]
fn test_ignore_unknown() {
    // 'Default' allows unknown. Basic smoke test of ignore...
    assert_de_tokens(
        &DefaultStruct {
            a1: 1,
            a2: 2,
            a3: 3,
            a4: 0,
            a5: 123,
        },
        &[
            Token::Struct {
                name: "DefaultStruct",
                len: 3,
            },
            Token::Str("whoops1"),
            Token::I32(2),
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("whoops2"),
            Token::Seq { len: Some(1) },
            Token::I32(2),
            Token::SeqEnd,
            Token::Str("a2"),
            Token::I32(2),
            Token::Str("whoops3"),
            Token::I32(2),
            Token::Str("a3"),
            Token::I32(3),
            Token::StructEnd,
        ],
    );

    assert_de_tokens_error::<DenyUnknown>(
        &[
            Token::Struct {
                name: "DenyUnknown",
                len: 1,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("whoops"),
        ],
        "unknown field `whoops`, expected `a1`",
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Superhero")]
struct RenameStruct {
    a1: i32,
    #[serde(rename = "a3")]
    a2: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename(serialize = "SuperheroSer", deserialize = "SuperheroDe"))]
struct RenameStructSerializeDeserialize {
    a1: i32,
    #[serde(rename(serialize = "a4", deserialize = "a5"))]
    a2: i32,
}

#[test]
fn test_rename_struct() {
    assert_tokens(
        &RenameStruct { a1: 1, a2: 2 },
        &[
            Token::Struct {
                name: "Superhero",
                len: 2,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("a3"),
            Token::I32(2),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &RenameStructSerializeDeserialize { a1: 1, a2: 2 },
        &[
            Token::Struct {
                name: "SuperheroSer",
                len: 2,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("a4"),
            Token::I32(2),
            Token::StructEnd,
        ],
    );

    assert_de_tokens(
        &RenameStructSerializeDeserialize { a1: 1, a2: 2 },
        &[
            Token::Struct {
                name: "SuperheroDe",
                len: 2,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::Str("a5"),
            Token::I32(2),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Superhero")]
enum RenameEnum {
    #[serde(rename = "bruce_wayne")]
    Batman,
    #[serde(rename = "clark_kent")]
    Superman(i8),
    #[serde(rename = "diana_prince")]
    WonderWoman(i8, i8),
    #[serde(rename = "barry_allan")]
    Flash {
        #[serde(rename = "b")]
        a: i32,
    },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename(serialize = "SuperheroSer", deserialize = "SuperheroDe"))]
enum RenameEnumSerializeDeserialize<A> {
    #[serde(rename(serialize = "dick_grayson", deserialize = "jason_todd"))]
    Robin {
        a: i8,
        #[serde(rename(serialize = "c"))]
        #[serde(rename(deserialize = "d"))]
        b: A,
    },
}

#[test]
fn test_rename_enum() {
    assert_tokens(
        &RenameEnum::Batman,
        &[
            Token::UnitVariant {
                name: "Superhero",
                variant: "bruce_wayne",
            },
        ],
    );

    assert_tokens(
        &RenameEnum::Superman(0),
        &[
            Token::NewtypeVariant {
                name: "Superhero",
                variant: "clark_kent",
            },
            Token::I8(0),
        ],
    );

    assert_tokens(
        &RenameEnum::WonderWoman(0, 1),
        &[
            Token::TupleVariant {
                name: "Superhero",
                variant: "diana_prince",
                len: 2,
            },
            Token::I8(0),
            Token::I8(1),
            Token::TupleVariantEnd,
        ],
    );

    assert_tokens(
        &RenameEnum::Flash { a: 1 },
        &[
            Token::StructVariant {
                name: "Superhero",
                variant: "barry_allan",
                len: 1,
            },
            Token::Str("b"),
            Token::I32(1),
            Token::StructVariantEnd,
        ],
    );

    assert_ser_tokens(
        &RenameEnumSerializeDeserialize::Robin {
            a: 0,
            b: String::new(),
        },
        &[
            Token::StructVariant {
                name: "SuperheroSer",
                variant: "dick_grayson",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(0),
            Token::Str("c"),
            Token::Str(""),
            Token::StructVariantEnd,
        ],
    );

    assert_de_tokens(
        &RenameEnumSerializeDeserialize::Robin {
            a: 0,
            b: String::new(),
        },
        &[
            Token::StructVariant {
                name: "SuperheroDe",
                variant: "jason_todd",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(0),
            Token::Str("d"),
            Token::Str(""),
            Token::StructVariantEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize)]
struct SkipSerializingStruct<'a, B, C>
where
    C: ShouldSkip,
{
    a: &'a i8,
    #[serde(skip_serializing)]
    b: B,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    c: C,
}

#[test]
fn test_skip_serializing_struct() {
    let a = 1;
    assert_ser_tokens(
        &SkipSerializingStruct { a: &a, b: 2, c: 3 },
        &[
            Token::Struct {
                name: "SkipSerializingStruct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("c"),
            Token::I32(3),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &SkipSerializingStruct {
            a: &a,
            b: 2,
            c: 123,
        },
        &[
            Token::Struct {
                name: "SkipSerializingStruct",
                len: 1,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SkipStruct<B> {
    a: i8,
    #[serde(skip)]
    b: B,
}

#[test]
fn test_skip_struct() {
    assert_ser_tokens(
        &SkipStruct { a: 1, b: 2 },
        &[
            Token::Struct {
                name: "SkipStruct",
                len: 1,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::StructEnd,
        ],
    );

    assert_de_tokens(
        &SkipStruct { a: 1, b: 0 },
        &[
            Token::Struct {
                name: "SkipStruct",
                len: 1,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize)]
enum SkipSerializingEnum<'a, B, C>
where
    C: ShouldSkip,
{
    Struct {
        a: &'a i8,
        #[serde(skip_serializing)]
        _b: B,
        #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
        c: C,
    },
}

#[test]
fn test_skip_serializing_enum() {
    let a = 1;
    assert_ser_tokens(
        &SkipSerializingEnum::Struct { a: &a, _b: 2, c: 3 },
        &[
            Token::StructVariant {
                name: "SkipSerializingEnum",
                variant: "Struct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("c"),
            Token::I32(3),
            Token::StructVariantEnd,
        ],
    );

    assert_ser_tokens(
        &SkipSerializingEnum::Struct {
            a: &a,
            _b: 2,
            c: 123,
        },
        &[
            Token::StructVariant {
                name: "SkipSerializingEnum",
                variant: "Struct",
                len: 1,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::StructVariantEnd,
        ],
    );
}

#[derive(Debug, PartialEq)]
struct NotSerializeStruct(i8);

#[derive(Debug, PartialEq)]
enum NotSerializeEnum {
    Trouble,
}

impl SerializeWith for NotSerializeEnum {
    fn serialize_with<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        "trouble".serialize(ser)
    }
}

#[derive(Debug, PartialEq, Serialize)]
struct ContainsNotSerialize<'a, B, C, D>
where
    B: 'a,
    D: SerializeWith,
{
    a: &'a Option<i8>,
    #[serde(skip_serializing)]
    b: &'a B,
    #[serde(skip_serializing)]
    c: Option<C>,
    #[serde(serialize_with = "SerializeWith::serialize_with")]
    d: D,
}

#[test]
fn test_elt_not_serialize() {
    let a = 1;
    assert_ser_tokens(
        &ContainsNotSerialize {
            a: &Some(a),
            b: &NotSerializeStruct(2),
            c: Some(NotSerializeEnum::Trouble),
            d: NotSerializeEnum::Trouble,
        },
        &[
            Token::Struct {
                name: "ContainsNotSerialize",
                len: 2,
            },
            Token::Str("a"),
            Token::Some,
            Token::I8(1),
            Token::Str("d"),
            Token::Str("trouble"),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize)]
struct SerializeWithStruct<'a, B>
where
    B: SerializeWith,
{
    a: &'a i8,
    #[serde(serialize_with = "SerializeWith::serialize_with")]
    b: B,
}

#[test]
fn test_serialize_with_struct() {
    let a = 1;
    assert_ser_tokens(
        &SerializeWithStruct { a: &a, b: 2 },
        &[
            Token::Struct {
                name: "SerializeWithStruct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(false),
            Token::StructEnd,
        ],
    );

    assert_ser_tokens(
        &SerializeWithStruct { a: &a, b: 123 },
        &[
            Token::Struct {
                name: "SerializeWithStruct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize)]
enum SerializeWithEnum<'a, B>
where
    B: SerializeWith,
{
    Struct {
        a: &'a i8,
        #[serde(serialize_with = "SerializeWith::serialize_with")]
        b: B,
    },
}

#[test]
fn test_serialize_with_enum() {
    let a = 1;
    assert_ser_tokens(
        &SerializeWithEnum::Struct { a: &a, b: 2 },
        &[
            Token::StructVariant {
                name: "SerializeWithEnum",
                variant: "Struct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(false),
            Token::StructVariantEnd,
        ],
    );

    assert_ser_tokens(
        &SerializeWithEnum::Struct { a: &a, b: 123 },
        &[
            Token::StructVariant {
                name: "SerializeWithEnum",
                variant: "Struct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(true),
            Token::StructVariantEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum WithVariant {
    #[serde(serialize_with = "serialize_unit_variant_as_i8")]
    #[serde(deserialize_with = "deserialize_i8_as_unit_variant")]
    Unit,

    #[serde(serialize_with = "SerializeWith::serialize_with")]
    #[serde(deserialize_with = "DeserializeWith::deserialize_with")]
    Newtype(i32),

    #[serde(serialize_with = "serialize_variant_as_string")]
    #[serde(deserialize_with = "deserialize_string_as_variant")]
    Tuple(String, u8),

    #[serde(serialize_with = "serialize_variant_as_string")]
    #[serde(deserialize_with = "deserialize_string_as_variant")]
    Struct { f1: String, f2: u8 },
}

fn serialize_unit_variant_as_i8<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i8(0)
}

fn deserialize_i8_as_unit_variant<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    let n = i8::deserialize(deserializer)?;
    match n {
        0 => Ok(()),
        _ => Err(de::Error::invalid_value(Unexpected::Signed(n as i64), &"0")),
    }
}

fn serialize_variant_as_string<S>(f1: &str, f2: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{};{:?}", f1, f2).as_str())
}

fn deserialize_string_as_variant<'de, D>(deserializer: D) -> Result<(String, u8), D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut pieces = s.split(';');
    let f1 = match pieces.next() {
        Some(x) => x,
        None => return Err(de::Error::invalid_length(0, &"2")),
    };
    let f2 = match pieces.next() {
        Some(x) => x,
        None => return Err(de::Error::invalid_length(1, &"2")),
    };
    let f2 = match f2.parse() {
        Ok(n) => n,
        Err(_) => {
            return Err(de::Error::invalid_value(
                Unexpected::Str(f2),
                &"an 8-bit signed integer",
            ));
        }
    };
    Ok((f1.into(), f2))
}

#[test]
fn test_serialize_with_variant() {
    assert_ser_tokens(
        &WithVariant::Unit,
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Unit",
            },
            Token::I8(0),
        ],
    );

    assert_ser_tokens(
        &WithVariant::Newtype(123),
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Newtype",
            },
            Token::Bool(true),
        ],
    );

    assert_ser_tokens(
        &WithVariant::Tuple("hello".into(), 0),
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Tuple",
            },
            Token::Str("hello;0"),
        ],
    );

    assert_ser_tokens(
        &WithVariant::Struct {
            f1: "world".into(),
            f2: 1,
        },
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Struct",
            },
            Token::Str("world;1"),
        ],
    );
}

#[test]
fn test_deserialize_with_variant() {
    assert_de_tokens(
        &WithVariant::Unit,
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Unit",
            },
            Token::I8(0),
        ],
    );

    assert_de_tokens(
        &WithVariant::Newtype(123),
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Newtype",
            },
            Token::Bool(true),
        ],
    );

    assert_de_tokens(
        &WithVariant::Tuple("hello".into(), 0),
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Tuple",
            },
            Token::Str("hello;0"),
        ],
    );

    assert_de_tokens(
        &WithVariant::Struct {
            f1: "world".into(),
            f2: 1,
        },
        &[
            Token::NewtypeVariant {
                name: "WithVariant",
                variant: "Struct",
            },
            Token::Str("world;1"),
        ],
    );
}

#[derive(Debug, PartialEq, Deserialize)]
struct DeserializeWithStruct<B>
where
    B: DeserializeWith,
{
    a: i8,
    #[serde(deserialize_with = "DeserializeWith::deserialize_with")]
    b: B,
}

#[test]
fn test_deserialize_with_struct() {
    assert_de_tokens(
        &DeserializeWithStruct { a: 1, b: 2 },
        &[
            Token::Struct {
                name: "DeserializeWithStruct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(false),
            Token::StructEnd,
        ],
    );

    assert_de_tokens(
        &DeserializeWithStruct { a: 1, b: 123 },
        &[
            Token::Struct {
                name: "DeserializeWithStruct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}

#[derive(Debug, PartialEq, Deserialize)]
enum DeserializeWithEnum<B>
where
    B: DeserializeWith,
{
    Struct {
        a: i8,
        #[serde(deserialize_with = "DeserializeWith::deserialize_with")]
        b: B,
    },
}

#[test]
fn test_deserialize_with_enum() {
    assert_de_tokens(
        &DeserializeWithEnum::Struct { a: 1, b: 2 },
        &[
            Token::StructVariant {
                name: "DeserializeWithEnum",
                variant: "Struct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(false),
            Token::StructVariantEnd,
        ],
    );

    assert_de_tokens(
        &DeserializeWithEnum::Struct { a: 1, b: 123 },
        &[
            Token::StructVariant {
                name: "DeserializeWithEnum",
                variant: "Struct",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(1),
            Token::Str("b"),
            Token::Bool(true),
            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_missing_renamed_field_struct() {
    assert_de_tokens_error::<RenameStruct>(
        &[
            Token::Struct {
                name: "Superhero",
                len: 2,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::StructEnd,
        ],
        "missing field `a3`",
    );

    assert_de_tokens_error::<RenameStructSerializeDeserialize>(
        &[
            Token::Struct {
                name: "SuperheroDe",
                len: 2,
            },
            Token::Str("a1"),
            Token::I32(1),
            Token::StructEnd,
        ],
        "missing field `a5`",
    );
}

#[test]
fn test_missing_renamed_field_enum() {
    assert_de_tokens_error::<RenameEnum>(
        &[
            Token::StructVariant {
                name: "Superhero",
                variant: "barry_allan",
                len: 1,
            },
            Token::StructVariantEnd,
        ],
        "missing field `b`",
    );

    assert_de_tokens_error::<RenameEnumSerializeDeserialize<i8>>(
        &[
            Token::StructVariant {
                name: "SuperheroDe",
                variant: "jason_todd",
                len: 2,
            },
            Token::Str("a"),
            Token::I8(0),
            Token::StructVariantEnd,
        ],
        "missing field `d`",
    );
}

#[derive(Debug, PartialEq, Deserialize)]
enum InvalidLengthEnum {
    A(i32, i32, i32),
    B(
        #[serde(skip_deserializing)]
        i32,
        i32,
        i32,
    ),
}

#[test]
fn test_invalid_length_enum() {
    assert_de_tokens_error::<InvalidLengthEnum>(
        &[
            Token::TupleVariant {
                name: "InvalidLengthEnum",
                variant: "A",
                len: 3,
            },
            Token::I32(1),
            Token::TupleVariantEnd,
        ],
        "invalid length 1, expected tuple of 3 elements",
    );
    assert_de_tokens_error::<InvalidLengthEnum>(
        &[
            Token::TupleVariant {
                name: "InvalidLengthEnum",
                variant: "B",
                len: 3,
            },
            Token::I32(1),
            Token::TupleVariantEnd,
        ],
        "invalid length 1, expected tuple of 2 elements",
    );
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(into = "EnumToU32", from = "EnumToU32")]
struct StructFromEnum(Option<u32>);

impl Into<EnumToU32> for StructFromEnum {
    fn into(self) -> EnumToU32 {
        match self {
            StructFromEnum(v) => v.into(),
        }
    }
}

impl From<EnumToU32> for StructFromEnum {
    fn from(v: EnumToU32) -> Self {
        StructFromEnum(v.into())
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(into = "Option<u32>", from = "Option<u32>")]
enum EnumToU32 {
    One,
    Two,
    Three,
    Four,
    Nothing,
}

impl Into<Option<u32>> for EnumToU32 {
    fn into(self) -> Option<u32> {
        match self {
            EnumToU32::One => Some(1),
            EnumToU32::Two => Some(2),
            EnumToU32::Three => Some(3),
            EnumToU32::Four => Some(4),
            EnumToU32::Nothing => None,
        }
    }
}

impl From<Option<u32>> for EnumToU32 {
    fn from(v: Option<u32>) -> Self {
        match v {
            Some(1) => EnumToU32::One,
            Some(2) => EnumToU32::Two,
            Some(3) => EnumToU32::Three,
            Some(4) => EnumToU32::Four,
            _ => EnumToU32::Nothing,
        }
    }
}

#[test]
fn test_from_into_traits() {
    assert_ser_tokens::<EnumToU32>(&EnumToU32::One, &[Token::Some, Token::U32(1)]);
    assert_ser_tokens::<EnumToU32>(&EnumToU32::Nothing, &[Token::None]);
    assert_de_tokens::<EnumToU32>(&EnumToU32::Two, &[Token::Some, Token::U32(2)]);
    assert_ser_tokens::<StructFromEnum>(&StructFromEnum(Some(5)), &[Token::None]);
    assert_ser_tokens::<StructFromEnum>(&StructFromEnum(None), &[Token::None]);
    assert_de_tokens::<StructFromEnum>(&StructFromEnum(Some(2)), &[Token::Some, Token::U32(2)]);
}

#[test]
fn test_collect_other() {
    let mut extra = HashMap::new();
    extra.insert("c".into(), 3);
    assert_de_tokens(
        &CollectOther { a: 1, b: 2, extra: extra.clone() },
        &[
            Token::Map { len: None },
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::U32(2),
            Token::Str("c"),
            Token::U32(3),
            Token::MapEnd,
        ],
    );
    assert_ser_tokens(
        &CollectOther { a: 1, b: 2, extra },
        &[
            Token::Map { len: None },
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::U32(2),
            Token::Str("c"),
            Token::U32(3),
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_flatten_struct_enum() {
    let mut extra = HashMap::new();
    extra.insert("extra_key".into(), "extra value".into());
    let change_request = FlattenStructEnumWrapper {
        data: FlattenStructEnum::InsertInteger {
            index: 0,
            value: 42
        },
        extra,
    };
    assert_de_tokens(
        &change_request,
        &[
            Token::Map { len: None },
            Token::Str("insert_integer"),
            Token::Map { len: None },
            Token::Str("index"),
            Token::U32(0),
            Token::Str("value"),
            Token::U32(42),
            Token::MapEnd,
            Token::Str("extra_key"),
            Token::Str("extra value"),
            Token::MapEnd
        ],
    );
    assert_ser_tokens(
        &change_request,
        &[
            Token::Map { len: None },
            Token::Str("insert_integer"),
            Token::Struct { len: 2, name: "insert_integer" },
            Token::Str("index"),
            Token::U32(0),
            Token::Str("value"),
            Token::U32(42),
            Token::StructEnd,
            Token::Str("extra_key"),
            Token::Str("extra value"),
            Token::MapEnd
        ],
    );
}

#[test]
fn test_flatten_struct_tag_content_enum() {
    let change_request = FlattenStructTagContentEnumWrapper {
        outer: 42,
        data: FlattenStructTagContentEnumNewtype(
            FlattenStructTagContentEnum::InsertInteger {
                index: 0,
                value: 42
            }
        ),
    };
    assert_de_tokens(
        &change_request,
        &[
            Token::Map { len: None },
            Token::Str("outer"),
            Token::U32(42),
            Token::Str("type"),
            Token::Str("insert_integer"),
            Token::Str("value"),
            Token::Map { len: None },
            Token::Str("index"),
            Token::U32(0),
            Token::Str("value"),
            Token::U32(42),
            Token::MapEnd,
            Token::MapEnd,
        ],
    );
    assert_ser_tokens(
        &change_request,
        &[
            Token::Map { len: None },
            Token::Str("outer"),
            Token::U32(42),
            Token::Str("type"),
            Token::Str("insert_integer"),
            Token::Str("value"),
            Token::Struct { len: 2, name: "insert_integer" },
            Token::Str("index"),
            Token::U32(0),
            Token::Str("value"),
            Token::U32(42),
            Token::StructEnd,
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_flatten_struct_tag_content_enum_newtype() {
    let change_request = FlattenStructTagContentEnumWrapper {
        outer: 42,
        data: FlattenStructTagContentEnumNewtype(
            FlattenStructTagContentEnum::NewtypeVariant(
                FlattenStructTagContentEnumNewtypeVariant {
                    value: 23
                }
            )
        ),
    };
    assert_de_tokens(
        &change_request,
        &[
            Token::Map { len: None },
            Token::Str("outer"),
            Token::U32(42),
            Token::Str("type"),
            Token::Str("newtype_variant"),
            Token::Str("value"),
            Token::Map { len: None },
            Token::Str("value"),
            Token::U32(23),
            Token::MapEnd,
            Token::MapEnd,
        ],
    );
    assert_ser_tokens(
        &change_request,
        &[
            Token::Map { len: None },
            Token::Str("outer"),
            Token::U32(42),
            Token::Str("type"),
            Token::Str("newtype_variant"),
            Token::Str("value"),
            Token::Struct { len: 1, name: "FlattenStructTagContentEnumNewtypeVariant" },
            Token::Str("value"),
            Token::U32(23),
            Token::StructEnd,
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_unknown_field_in_flatten() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Outer {
        dummy: String,
        #[serde(flatten)]
        inner: Inner,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Inner {
        foo: HashMap<String, u32>,
    }

    assert_de_tokens_error::<Outer>(
        &[
            Token::Struct {
                name: "Outer",
                len: 1,
            },
            Token::Str("dummy"),
            Token::Str("23"),
            Token::Str("foo"),
            Token::Map { len: None },
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::U32(2),
            Token::MapEnd,
            Token::Str("bar"),
            Token::U32(23),
            Token::StructEnd,
        ],
        "unknown field `bar`",
    );
}

#[test]
fn test_complex_flatten() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Outer {
        y: u32,
        #[serde(flatten)]
        first: First,
        #[serde(flatten)]
        second: Second,
        z: u32
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct First {
        a: u32,
        b: bool,
        c: Vec<String>,
        d: String,
        e: Option<u64>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Second {
        f: u32,
    }

    assert_de_tokens(
        &Outer {
            y: 0,
            first: First {
                a: 1,
                b: true,
                c: vec!["a".into(), "b".into()],
                d: "c".into(),
                e: Some(2),
            },
            second: Second {
                f: 3
            },
            z: 4
        },
        &[
            Token::Map { len: None },
            Token::Str("y"),
            Token::U32(0),
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::Bool(true),
            Token::Str("c"),
            Token::Seq { len: Some(2) },
            Token::Str("a"),
            Token::Str("b"),
            Token::SeqEnd,
            Token::Str("d"),
            Token::Str("c"),
            Token::Str("e"),
            Token::U64(2),
            Token::Str("f"),
            Token::U32(3),
            Token::Str("z"),
            Token::U32(4),
            Token::MapEnd,
        ],
    );

    assert_ser_tokens(
        &Outer {
            y: 0,
            first: First {
                a: 1,
                b: true,
                c: vec!["a".into(), "b".into()],
                d: "c".into(),
                e: Some(2),
            },
            second: Second {
                f: 3
            },
            z: 4
        },
        &[
            Token::Map { len: None },
            Token::Str("y"),
            Token::U32(0),
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::Bool(true),
            Token::Str("c"),
            Token::Seq { len: Some(2) },
            Token::Str("a"),
            Token::Str("b"),
            Token::SeqEnd,
            Token::Str("d"),
            Token::Str("c"),
            Token::Str("e"),
            Token::Some,
            Token::U64(2),
            Token::Str("f"),
            Token::U32(3),
            Token::Str("z"),
            Token::U32(4),
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_flatten_unsupported_type() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Outer {
        outer: String,
        #[serde(flatten)]
        inner: String,
    }

    assert_ser_tokens_error(
        &Outer {
            outer: "foo".into(),
            inner: "bar".into(),
        },
        &[
            Token::Map { len: None },
            Token::Str("outer"),
            Token::Str("foo"),
        ],
        "can only flatten structs and maps (got a string)",
    );
    assert_de_tokens_error::<Outer>(
        &[
            Token::Map { len: None },
            Token::Str("outer"),
            Token::Str("foo"),
            Token::Str("a"),
            Token::Str("b"),
            Token::MapEnd
        ],
        "can only flatten structs and maps",
    );
}

#[test]
fn test_non_string_keys() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        name: String,
        age: u32,
        #[serde(flatten)]
        mapping: HashMap<u32, u32>,
    }

    let mut mapping = HashMap::new();
    mapping.insert(0, 42);
    assert_tokens(
        &TestStruct { name: "peter".into(), age: 3, mapping },
        &[
            Token::Map { len: None },
            Token::Str("name"),
            Token::Str("peter"),
            Token::Str("age"),
            Token::U32(3),
            Token::U32(0),
            Token::U32(42),
            Token::MapEnd,
        ],
    );
}
