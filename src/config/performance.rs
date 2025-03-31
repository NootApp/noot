use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct PerformanceConfiguration {
    pub max_work_threads: usize,
    pub max_history_size: usize,
}

impl Default for PerformanceConfiguration {
    fn default() -> PerformanceConfiguration {
        PerformanceConfiguration {
            max_work_threads: 6,
            max_history_size: 20,
        }
    }
}