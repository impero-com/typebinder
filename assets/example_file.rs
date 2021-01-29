use std::collections::HashSet;

#[derive(Debug, Serialze, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportSerde<'a> {
    pub field_one: u32,
    pub field_two: String,
    pub field_three: Vec<String>,
    pub field_four: [u8; 4],
    pub field_five: HashSet<i32>,
    pub field_six: [u8],
    pub field_seven: (u32, String),
    pub field_eight: Option<String>,
    pub field_nine: &'a [u8],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPair(i32, i32);

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ControlResultAssigneeKind {
    #[serde(rename_all = "camelCase")]
    User { user_id: u32 },
    #[serde(rename_all = "camelCase")]
    Pool { user_pool_id: u32 },
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ControlResultAssigneeOther {
    #[serde(rename_all = "camelCase")]
    User(u32),
    #[serde(rename_all = "camelCase")]
    Pool(u32),
}
