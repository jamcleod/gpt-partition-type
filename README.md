# gpt-partition-type

A Rust library for getting information about a GPT partition from its partition type GUID.

### Usage:

```rust
use gpt_partition_type::{PartitionDescription, parse_guid};

assert_eq!(
    parse_guid("0FC63DAF-8483-4772-8E79-3D69D8477DE4").description().unwrap(),
    PartitionDescription {
        os: "Linux",
        type_description: "Linux filesystem data"
    }
);
```
