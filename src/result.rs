use std::fmt;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;
use tabled::Tabled;

#[derive(Debug, Clone)]
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
    pub name: String,
    pub ip: IpAddr,
    pub last_resolved_ip: IpAddr,
    /// String with the following format: "successfull_requests/total_requests (success_rate))"
    pub successfull_requests: String,
    pub first_duration: TimeResult,
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
