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
    pub first_duration: TimeResult,
    pub first_duration_color: tabled_settings::Color,
    pub average_duration: TimeResult,
    pub average_duration_color: tabled_settings::Color,
}

impl From<Vec<MeasureResult>> for RawResultEntry {
    fn from(value: Vec<MeasureResult>) -> Self {
        let mut successful_requests = 0;
        let mut total_time = Duration::new(0, 0);
        let mut last_resolved_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        for measure_result in &value {
            match measure_result.time {
                TimeResult::Succeeded(duration) => {
                    successful_requests += 1;
                    total_time += duration;
                    last_resolved_ip = measure_result.resolved_ip;
                }
                TimeResult::Failed(_) => {}
            }
        }

        let average_duration = if successful_requests > 0 {
            TimeResult::Succeeded(total_time / successful_requests as u32)
        } else {
            TimeResult::Failed(String::from("No successful requests"))
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
            first_duration: value[0].time.clone(),
            first_duration_color: value[0].time.clone().into(),
            average_duration: average_duration.clone(),
            average_duration_color: average_duration.into(),
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
        assert_eq!(
            result_entry.first_duration,
            TimeResult::Succeeded(Duration::new(0, 100))
        );
        assert_eq!(
            result_entry.first_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
        assert_eq!(
            result_entry.average_duration,
            TimeResult::Succeeded(Duration::new(0, 150))
        );
        assert_eq!(
            result_entry.average_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
    }
}
