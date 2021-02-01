# TS Export 

Outputs TS definitions from your Rust code. Works as a library to integrate in your `build.rs` file, or as a CLI.

* Based on `serde`
* Modular : define your own "hooks" to serialize your own custom types 

## Examples

Given this Rust definition of a struct 

```rust
#[derive(Debug, Serialze, Deserialize)]
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
}
```

The tool outputs : 

```typescript
export interface SupportSerde {
        fieldOne: number,
        fieldTwo: string,
        fieldThree: string[],
        fieldFour: number[],
        fieldFive: number[],
        fieldSix: [ number, string ],
        fieldSeven: string | null,
        fieldEight: number[]
}
```