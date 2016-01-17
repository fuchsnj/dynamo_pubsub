Highly scalable publish/subscribe built on AWS DynamoDB
====

Note that this SDK is not yet feature complete, and is still undergoing major changes.

## Documentation

```
cargo doc
```

## Usage

This will be uploaded to crates.io when it is more stable.
For now, add the following to your `Cargo.toml`

```toml
[dependencies.aws_dynamodb]
git = "https://github.com/fuchsnj/dynamo_pubsub.git"
```

and this to your crate root:

```rust
extern crate dynamo_pubsub;
```