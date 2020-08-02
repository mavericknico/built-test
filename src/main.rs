
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod built_info {
   // The file has been placed there by the build script.
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

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
