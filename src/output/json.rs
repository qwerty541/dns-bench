use crate::output::OutputFormatter;
use crate::output::OutputFormatterContext;
use crate::output::OutputFormatterError;
use crate::result::RawResultEntry;
use crate::result::TimeResult;
use std::io;
use std::net::IpAddr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct JsonResultEntry {
    name: String,
    ip: IpAddr,
    last_resolved_ip: IpAddr,
    total_requests: i32,
    successful_requests: i32,
    successful_requests_percentage: f32,
    min_duration: TimeResult,
    max_duration: TimeResult,
    avg_duration: TimeResult,
}

impl From<RawResultEntry> for JsonResultEntry {
    fn from(value: RawResultEntry) -> Self {
        JsonResultEntry {
            name: value.name,
            ip: value.ip,
            last_resolved_ip: value.last_resolved_ip,
            total_requests: value.total_requests,
            successful_requests: value.successful_requests,
            successful_requests_percentage: value.successful_requests_percentage,
            min_duration: value.min_duration,
            max_duration: value.max_duration,
            avg_duration: value.avg_duration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonOutputFormatter;

impl OutputFormatter for JsonOutputFormatter {
    fn write(
        &self,
        results: &[RawResultEntry],
        _ctx: OutputFormatterContext,
        w: &mut dyn io::Write,
    ) -> Result<(), OutputFormatterError> {
        let json_entries: Vec<JsonResultEntry> =
            results.iter().cloned().map(JsonResultEntry::from).collect();

        let json_string = serde_json::to_string_pretty(&json_entries)
            .map_err::<OutputFormatterError, _>(From::from)?;

        writeln!(w, "{}", json_string).map_err::<OutputFormatterError, _>(From::from)?;

        Ok(())
    }
}
