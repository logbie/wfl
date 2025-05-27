pub mod checker;

pub use checker::{
    ConfigChecker, ConfigIssue, ConfigIssueKind, ConfigIssueType, check_config, fix_config,
};
