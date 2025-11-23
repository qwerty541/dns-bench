use std::fmt;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;
use tabled::settings as tabled_settings;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TimeResult
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, derive_more::IsVariant)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum TimeResult {
    #[serde(rename = "succeeded")]
    Succeeded(Duration),
    #[serde(rename = "failed")]
    Failed(String),
}

impl TimeResult {
    pub fn get_xml_type_str(&self) -> &str {
        match self {
            TimeResult::Succeeded(_) => "succeeded",
            TimeResult::Failed(_) => "failed",
        }
    }

    pub fn get_duration_millis(&self) -> Option<String> {
        match self {
            TimeResult::Succeeded(duration) => {
                let millis = duration.as_secs() * 1000 + u64::from(duration.subsec_millis());
                let fractional = duration.subsec_nanos() % 1_000_000; // Remaining fractional part in nanoseconds
                Some(format!(
                    "{:.6}",
                    millis as f64 + fractional as f64 / 1_000_000.0
                ))
            }
            TimeResult::Failed(_) => None,
        }
    }

    pub fn get_error_str(&self) -> Option<&str> {
        match self {
            TimeResult::Succeeded(_) => None,
            TimeResult::Failed(error) => Some(error),
        }
    }

    pub fn is_timeout(&self) -> bool {
        match self {
            TimeResult::Succeeded(_) => false,
            TimeResult::Failed(error) => {
                let lower = error.to_ascii_lowercase();
                lower.contains("timeout") || lower.contains("timed out")
            }
        }
    }
}

impl fmt::Display for TimeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeResult::Succeeded(duration) => write!(f, "{duration:?}"),
            TimeResult::Failed(error) => write!(f, "{error}"),
        }
    }
}

impl From<TimeResult> for tabled_settings::Color {
    fn from(value: TimeResult) -> Self {
        match value {
            TimeResult::Succeeded(duration) => {
                if duration.as_millis() <= 30 {
                    tabled_settings::Color::FG_BRIGHT_GREEN
                } else if duration.as_millis() <= 80 {
                    tabled_settings::Color::FG_BRIGHT_YELLOW
                } else {
                    tabled_settings::Color::FG_BRIGHT_RED
                }
            }
            TimeResult::Failed(_) => tabled_settings::Color::FG_RED,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// MeasureResult
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MeasureResult {
    pub name: String,
    pub ip: IpAddr,
    pub resolved_ip: IpAddr,
    pub time: TimeResult,
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// RawResultEntry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct RawResultEntry {
    pub name: String,
    pub ip: IpAddr,
    pub last_resolved_ip: IpAddr,
    pub total_requests: i32,
    pub successful_requests: i32,
    pub successful_requests_percentage: f32,
    pub successful_requests_color: tabled_settings::Color,
    pub min_duration: TimeResult,
    pub min_duration_color: tabled_settings::Color,
    pub max_duration: TimeResult,
    pub max_duration_color: tabled_settings::Color,
    pub avg_duration: TimeResult,
    pub avg_duration_color: tabled_settings::Color,
}

impl From<Vec<MeasureResult>> for RawResultEntry {
    fn from(value: Vec<MeasureResult>) -> Self {
        let mut successful_requests = 0;
        let mut total_time = Duration::new(0, 0);
        let mut last_resolved_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        // Compute min/max on successful requests
        let mut min_dur: Option<Duration> = None;
        let mut max_dur: Option<Duration> = None;

        for measure_result in &value {
            match measure_result.time {
                TimeResult::Succeeded(duration) => {
                    successful_requests += 1;
                    total_time += duration;
                    last_resolved_ip = measure_result.resolved_ip;

                    min_dur = Some(match min_dur {
                        Some(current_min) => current_min.min(duration),
                        None => duration,
                    });
                    max_dur = Some(match max_dur {
                        Some(current_max) => current_max.max(duration),
                        None => duration,
                    });
                }
                TimeResult::Failed(_) => {}
            }
        }

        let avg_duration = if successful_requests > 0 {
            TimeResult::Succeeded(total_time / successful_requests as u32)
        } else {
            TimeResult::Failed(String::from("No responses"))
        };

        let min_duration = if let Some(d) = min_dur {
            TimeResult::Succeeded(d)
        } else {
            TimeResult::Failed(String::from("No responses"))
        };

        let max_duration = if let Some(d) = max_dur {
            TimeResult::Succeeded(d)
        } else {
            TimeResult::Failed(String::from("No responses"))
        };

        let successful_requests_percentage =
            successful_requests as f32 / value.len() as f32 * 100.0;
        let successful_requests_color = if successful_requests_percentage == 100.0 {
            tabled_settings::Color::FG_BRIGHT_GREEN
        } else if successful_requests_percentage >= 50.0 {
            tabled_settings::Color::FG_BRIGHT_YELLOW
        } else if successful_requests_percentage >= 20.0 {
            tabled_settings::Color::FG_BRIGHT_RED
        } else {
            tabled_settings::Color::FG_RED
        };

        RawResultEntry {
            name: value[0].name.clone(),
            ip: value[0].ip,
            last_resolved_ip,
            total_requests: value.len() as i32,
            successful_requests,
            successful_requests_percentage,
            successful_requests_color,
            min_duration: min_duration.clone(),
            min_duration_color: min_duration.clone().into(),
            max_duration: max_duration.clone(),
            max_duration_color: max_duration.into(),
            avg_duration: avg_duration.clone(),
            avg_duration_color: avg_duration.clone().into(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Tests
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_result_entry_from() {
        let measure_results = vec![
            MeasureResult {
                name: String::from("Google"),
                ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                resolved_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                time: TimeResult::Succeeded(Duration::new(0, 100)),
            },
            MeasureResult {
                name: String::from("Google"),
                ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                resolved_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                time: TimeResult::Succeeded(Duration::new(0, 200)),
            },
            MeasureResult {
                name: String::from("Google"),
                ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                resolved_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                time: TimeResult::Failed(String::from("Timeout")),
            },
        ];

        let result_entry = RawResultEntry::from(measure_results);

        assert_eq!(result_entry.name, "Google");
        assert_eq!(result_entry.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        assert_eq!(
            result_entry.last_resolved_ip,
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))
        );
        assert_eq!(result_entry.total_requests, 3);
        assert_eq!(result_entry.successful_requests, 2);
        assert_eq!(result_entry.successful_requests_percentage, 66.66667);
        assert_eq!(
            result_entry.successful_requests_color,
            tabled_settings::Color::FG_BRIGHT_YELLOW
        );
        // Min
        assert_eq!(
            result_entry.min_duration,
            TimeResult::Succeeded(Duration::new(0, 100))
        );
        assert_eq!(
            result_entry.min_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
        // Max
        assert_eq!(
            result_entry.max_duration,
            TimeResult::Succeeded(Duration::new(0, 200))
        );
        assert_eq!(
            result_entry.max_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
        // Avg
        assert_eq!(
            result_entry.avg_duration,
            TimeResult::Succeeded(Duration::new(0, 150))
        );
        assert_eq!(
            result_entry.avg_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
    }
}
