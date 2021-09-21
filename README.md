# Typebinder

Translate your Rust structures into TypeScript.

`typebinder` works as a library to integrate in your `build.rs` file, or as a CLI. A default CLI is available for your simple use-cases, `typebinder_cli`.

Using `typebinder` will statically prevent desynchronizations between your front-end (provided that it runs on TypeScript) and your backend.

## Features

* Based on `serde`
* Modular : define your own "hooks" to serialize your own custom types
* Supports Structs, Enums (with serde tag variants) and type aliases, as described below

## Usage

```
# Outputs your bindings to the `<typescript_src>` folder
typebinder_cli <path/to/mod.rs> generate -o <typescript_src>
```

```
# Displays the generated bindings to stdout
typebinder_cli <path/to/mod.rs> generate
```

```
# Checks that your bindings are up to date, synchronized with your Rust codebase
typebinder_cli <path/to/mod.rs> check <typescript_src>
```

## Example

### Structures

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyStruct<'a> {
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

Will translate to the following interface :

```typescript
export interface MyStruct {
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

### Enums

Enums are also supported, and all `serde` tag variants are supported.

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AdjacentlyTagged {
    #[serde(rename_all = "camelCase")]
    FirstVariant { id: u32, name_of_thing: String },
    #[serde(rename_all = "camelCase")]
    SecondVariant { id: u32, age_of_thing: u32 },
}

#[derive(Serialize, Deserialize)]
pub enum ExternallyTagged {
    #[serde(rename_all = "camelCase")]
    A { id_of_thing: u32 },
    #[serde(rename_all = "camelCase")]
    B { id: u32, name_of_thing: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum InternallyTagged {
    A,
    B(u32),
    D { age: u32, name: String },
}


#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Option<T> {
    Some(T),
    None,
}
```

Will output :

```typescript
export type AdjacentlyTagged = {
    type: "FirstVariant",
    data: {
        id: number,
        nameOfThing: string
    }
} | {
    type: "SecondVariant",
    data: {
        id: number,
        ageOfThing: number
    }
};

export type ExternallyTagged = {
    "A": {
        idOfThing: number
    }
} | {
    "B": {
        id: number,
        nameOfThing: string
    }
};

export type InternallyTagged = ({
    type: "A"
}) | ({
    type: "B"
} & number ) | ({
    type: "D"
} & {
    age: number,
    name: string
});

export type Option<T> = T | null;
```

## Type alias

Type alias are also supported.

```rust
type ArrayOfNumbers = Vec<u32>;
```

Will translate to

```typescript
type ArrayOfNumbers = number[];
```

## Fair warning

While the tool works and is being used in production at [Impero](https://impero.com), `typebinder` is still in development and might not be exactly feature-complete. **Codegen is hard**.

## Run tests

### Unit tests

```
cargo test
```

### Integration tests

This launches the unit tests and the test suite (see `typebinder_test_suite`) :

```
make test
```

This assumes that you have `tsc` installed. If not, consider:

```
npm install -g typescript
```
