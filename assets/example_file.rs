use std::collections::HashSet;

#[derive(Debug, Serialze, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportSerde {
    pub field_one: u32,
    pub field_two: String,
    pub field_three: Vec<String>,
    pub field_four: [u8; 4],
    pub field_five: HashSet<i32>,
    pub field_six: [u8],
    pub field_seven: (u32, String),
    pub field_eight: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPair(i32, i32);
