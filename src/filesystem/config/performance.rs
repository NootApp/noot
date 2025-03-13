use crate::filesystem::utils::traits::{Configuration, ValidationError};
use serde_derive::{Deserialize, Serialize};


pub const THREAD_COUNT_LIMIT: usize = 12;

/// PerformanceConfiguration is a struct representing all the global performance settings for Noot
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct PerformanceConfiguration {
    /// The maximum number of background threads to handle event processing
    pub max_work_threads: Option<usize>,
    /// The maximum number of history entries for each file (may be overridden by workspaces)
    pub max_history_size: Option<usize>,
}

impl Default for PerformanceConfiguration {
    fn default() -> PerformanceConfiguration {
        PerformanceConfiguration {
            max_work_threads: Some(6),
            max_history_size: Some(20),
        }
    }
}

impl Configuration for PerformanceConfiguration {
    fn validate(&self, prefix: &str) -> Vec<ValidationError> {
        let mut issues = vec![];
        if self.max_work_threads.is_none() {
            issues.push(ValidationError::new(
                &format!("{}.performance.max-work-threads",prefix),
                "Maximum work threads unset. This will default to 6 (or less if your system has fewer threads available)",
                true,
            ));
        } else {
            let max_work_threads = self.max_work_threads.clone().unwrap();
            let system_threads = num_cpus::get();
            let system_physical_cpus = num_cpus::get_physical();
            let ratio = max_work_threads as f64 / system_threads as f64;


            if ratio > 0.5 {
                issues.push(ValidationError::new(
                    &format!("{}.performance.max-work-threads", prefix),
                    &format!("Your work thread count ({}) is more than 50% of your available thread count. This may cause performance issues", max_work_threads),
                    true
                ))
            }
            
            if max_work_threads > THREAD_COUNT_LIMIT {
                issues.push(ValidationError::new(
                    &format!("{}.performance.max-work-threads", prefix),
                    &format!("Cannot use more than {} threads. You chose {}. Automatically sizing back to the upper limit.", THREAD_COUNT_LIMIT, max_work_threads),
                    true
                ))
            }

        }

        issues
    }

    fn repair(&mut self) {
        if self.max_work_threads.is_none() {
            self.max_work_threads = Some(6);
        }
    }
}
