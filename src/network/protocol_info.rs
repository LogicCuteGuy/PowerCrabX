use std::ops::Deref;
use once_cell::sync::Lazy;
use crate::utils::sem_version::SemVersion;

pub static CODE_NAME: &str = "PowerCrabX";
pub static API_VERSION: &str = "0.0.1";
pub static CURRENT_PROTOCOL: i32 = 786;
pub static GAME_VERSION: &str = "1.21.70";
pub static GAME_VERSION_FULL: &str = "v1.21.70";

pub static SEM_VERSION: Lazy<SemVersion> = Lazy::new(|| SemVersion::new(1, 21, 7, 0 ,0));
pub static BLOCK_STATE_VERSION: Lazy<i32> = Lazy::new(|| {
    let v = SEM_VERSION.deref();
    (v.major << 24) | (v.minor << 16) | (v.patch << 8)
});