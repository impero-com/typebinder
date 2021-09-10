use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportSerde<'a> {
    pub field_one: u32,
    pub field_two: String,
    pub field_three: Vec<String>,
    pub field_four: [u8; 4],
    pub field_five: HashSet<i32>,
    pub field_six: (u32, String),
    pub field_seven: Option<String>,
    pub field_eight: &'a [u8],
    pub field_nine: (),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPair(i32, i32);

#[derive(PartialEq, Eq, Serialize, Debug)]
#[serde(untagged)]
pub enum Protected<T> {
    Visible(T),
    Confidential,
}

#[derive(PartialEq, Eq, Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum AdjacentEnum {
    #[serde(rename_all = "camelCase")]
    FirstVariant { id: u32, name_of_thing: String },
    #[serde(rename_all = "camelCase")]
    SecondVariant { id: u32, age_of_thing: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExternalEnum {
    #[serde(rename_all = "camelCase")]
    A { id_of_thing: u32 },
    #[serde(rename_all = "camelCase")]
    B { id: u32, name_of_thing: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithGeneric<T> {
    id: i32,
    name: String,
    value: T,
}

type ArrayOfNumbers = Vec<u32>;
type Array<T> = Vec<T>;
type WithGenericNumber = WithGeneric<u32>;

mod test {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StructInMod {
        field_one: String,
        field_two: u32,
    }
}

pub struct MyCowWrapper<'a> {
    my_cow: std::borrow::Cow<'a>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum MyEnum {
    A,
    B(u32),
    C((u32, u32)),
    D { age: u32, name: String },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportSkipSerialize {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    age: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_age: Option<u32>,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct Person {
    age: u32,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum InternallyTagged {
    A,
    D { age: u32, name: String },
}

#[derive(Serialize, Deserialize)]
pub struct MyCustomMap<T> {
    the_map: HashMap<T, u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrNumber {
    String(String),
    Number(f64),
}

type VecOfOptionalNumbers = Vec<Option<u32>>;
type OptionalStringOrNumber = Option<StringOrNumber>;
type VecOfOptionalStringOrNumbers = Vec<Option<StringOrNumber>>;
