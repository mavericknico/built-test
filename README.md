# Overview
This is a test Rust program to embed compile time metadata (such as build time, git hash commit, etc) so that it is accessible at runtime. We do this using the `built` crate.


# How It Works
## Update Cargo Manifest

Update `Cargo.toml` to include the following entries;

```
[package]
build = "build.rs"


[build-dependencies]
built = {version="0.4.2", features =["git2", "chrono"] }
```

## Add build.rs File

Create a file called `build.rs` in the root directory of the workspace and add the following:

```rust
fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
```
This will generate a file called `built.rs` containing some metadata.

## Import Metadata 

Embed the data in `built.rs` in a module for runtime use:

```rust
pub mod built_info {
   // The built.rs file has been placed there by the build script.
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
```

## Runtime Access
Now you can access this data at runtime:

```rust
#[derive(Debug, Default, Deserialize, Serialize)]
struct VersionInfo<'a> {
    name: &'a str,
    version: &'a str,
    features: &'a str,
    git_commit: &'a str,
    build_time_utc: &'a str,
    rustc: &'a str,
}

fn main() -> Result<()> {
    
    let vi = VersionInfo {
        name: built_info::PKG_NAME,
        version: built_info::PKG_VERSION,
        features: built_info::FEATURES_STR,
        git_commit: built_info::GIT_COMMIT_HASH.unwrap_or_default(),
        build_time_utc: built_info::BUILT_TIME_UTC,
        rustc: built_info::RUSTC_VERSION,
    };
    let version_info_string = serde_json::to_string(&vi)?;
    println!("{}", version_info_string);
    Ok(())
}
```

Running the code above yields output like the following (prettified):
```json
{
  "name": "test-built",
  "version": "0.1.0",
  "features": "",
  "git_commit": "e0dad77b011b585ecade3660354fd130a660cadd",
  "build_time_utc": "Sun, 02 Aug 2020 03:34:36 +0000",
  "rustc": "rustc 1.45.1 (c367798cf 2020-07-26)"
}
```
