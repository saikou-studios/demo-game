use std::fmt;

pub struct CommitInfo {
    pub hash: &'static str,
    pub short_hash: &'static str,
    pub date: &'static str,
}

pub struct VersionInfo {
    pub version: &'static str,
    pub commit_info: Option<CommitInfo>,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.version)?;

        if let Some(commit_info) = &self.commit_info {
            write!(f, " ({} {})", commit_info.short_hash, commit_info.date)?;
        };

        Ok(())
    }
}

pub const fn version() -> VersionInfo {
    let version = if let Some(x) = option_env!("CFG_RELEASE") {
        x
    } else {
        "0.0.0"
    };
    let commit_info = match (
        option_env!("RA_COMMIT_SHORT_HASH"),
        option_env!("RA_COMMIT_HASH"),
        option_env!("RA_COMMIT_DATE"),
    ) {
        (Some(short_hash), Some(hash), Some(date)) => Some(CommitInfo {
            hash,
            short_hash,
            date,
        }),
        _ => None,
    };

    VersionInfo {
        version,
        commit_info,
    }
}
