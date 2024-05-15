use std::fmt;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;
use tabled::Tabled;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum TimeResult {
    Succeeded(Duration),
    Failed(String),
}

impl fmt::Display for TimeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeResult::Succeeded(duration) => write!(f, "{:?}", duration),
            TimeResult::Failed(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeasureResult {
    pub name: String,
    pub ip: IpAddr,
    pub resolved_ip: IpAddr,
    pub time: TimeResult,
}

#[derive(Debug, Clone, Tabled)]
pub struct ResultEntry {
    #[tabled(rename = "Server name")]
    pub name: String,
    #[tabled(rename = "IP address")]
    pub ip: IpAddr,
    #[tabled(rename = "Last resolved IP")]
    pub last_resolved_ip: IpAddr,
    /// String with the following format: "successfull_requests/total_requests (success_rate))"
    #[tabled(rename = "Success rate")]
    pub successfull_requests: String,
    #[tabled(rename = "First duration")]
    pub first_duration: TimeResult,
    #[tabled(rename = "Average duration")]
    pub average_duration: TimeResult,
}

impl From<Vec<MeasureResult>> for ResultEntry {
    fn from(value: Vec<MeasureResult>) -> Self {
        let mut successfull_requests = 0;
        let mut total_time = Duration::new(0, 0);
        let mut last_resolved_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        for measure_result in &value {
            match measure_result.time {
                TimeResult::Succeeded(duration) => {
                    successfull_requests += 1;
                    total_time += duration;
                    last_resolved_ip = measure_result.resolved_ip;
                }
                TimeResult::Failed(_) => {}
            }
        }

        let average_duration = if successfull_requests > 0 {
            TimeResult::Succeeded(total_time / successfull_requests as u32)
        } else {
            TimeResult::Failed(String::from("No successfull requests"))
        };

        ResultEntry {
            name: value[0].name.clone(),
            ip: value[0].ip,
            last_resolved_ip,
            successfull_requests: format!(
                "{}/{} ({:.2}%)",
                successfull_requests,
                value.len(),
                successfull_requests as f32 / value.len() as f32 * 100.0
            ),
            first_duration: value[0].time.clone(),
            average_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_entry_from() {
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

        let result_entry = ResultEntry::from(measure_results);

        assert_eq!(result_entry.name, "Google");
        assert_eq!(result_entry.ip, IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
        assert_eq!(
            result_entry.last_resolved_ip,
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))
        );
        assert_eq!(result_entry.successfull_requests, "2/3 (66.67%)");
        assert_eq!(
            result_entry.first_duration,
            TimeResult::Succeeded(Duration::new(0, 100))
        );
        assert_eq!(
            result_entry.average_duration,
            TimeResult::Succeeded(Duration::new(0, 150))
        );
    }
}
