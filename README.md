# DBF Reader
A streaming parser for
[.dbf](https://en.wikipedia.org/wiki/.dbf) files,
written in Rust.
Extracts deserialized records one at a time from anything implementing
[`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html),
without having to allocate space to store the entire document.

## Usage

```rust
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
struct Record {
    my_field_1: f32,
    my_field_2: String,
    my_field_3: NaiveDate,
}

fn iterate<R: std::io::Read>(file: R) -> dbf_reader::Result<()> {
    let mut document = dbf_reader::from_file(file)?;

    for record in document.records() {
        let record: Record = record?;
        // process a record
    }

    Ok(())
}
```
