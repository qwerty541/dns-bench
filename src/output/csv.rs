use crate::output::OutputFormatter;
use crate::output::OutputFormatterContext;
use crate::output::OutputFormatterError;
use crate::result::RawResultEntry;
use std::fmt;
use std::io;
use std::net::IpAddr;
use std::string::FromUtf8Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CsvResultEntry {
    name: String,
    ip: IpAddr,
    last_resolved_ip: IpAddr,
    total_requests: i32,
    successful_requests: i32,
    successful_requests_percentage: f32,
    min_duration_value_ms: Option<String>,
    min_duration_error: Option<String>,
    max_duration_value_ms: Option<String>,
    max_duration_error: Option<String>,
    avg_duration_value_ms: Option<String>,
    avg_duration_error: Option<String>,
}

impl From<RawResultEntry> for CsvResultEntry {
    fn from(value: RawResultEntry) -> Self {
        CsvResultEntry {
            name: value.name,
            ip: value.ip,
            last_resolved_ip: value.last_resolved_ip,
            total_requests: value.total_requests,
            successful_requests: value.successful_requests,
            successful_requests_percentage: value.successful_requests_percentage,
            min_duration_value_ms: value.min_duration.get_duration_millis(),
            min_duration_error: value.min_duration.get_error_str().map(|v| v.to_string()),
            max_duration_value_ms: value.max_duration.get_duration_millis(),
            max_duration_error: value.max_duration.get_error_str().map(|v| v.to_string()),
            avg_duration_value_ms: value.avg_duration.get_duration_millis(),
            avg_duration_error: value.avg_duration.get_error_str().map(|v| v.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum CsvConversionError {
    Io(io::Error),
    FromUtf8(FromUtf8Error),
    Csv(csv::Error),
}

impl fmt::Display for CsvConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvConversionError::Io(e) => write!(f, "IO error: {e}"),
            CsvConversionError::FromUtf8(e) => write!(f, "UTF-8 error: {e}"),
            CsvConversionError::Csv(e) => write!(f, "CSV error: {e}"),
        }
    }
}

impl From<io::Error> for CsvConversionError {
    fn from(e: io::Error) -> Self {
        CsvConversionError::Io(e)
    }
}

impl From<FromUtf8Error> for CsvConversionError {
    fn from(e: FromUtf8Error) -> Self {
        CsvConversionError::FromUtf8(e)
    }
}

impl From<csv::Error> for CsvConversionError {
    fn from(e: csv::Error) -> Self {
        CsvConversionError::Csv(e)
    }
}

impl std::error::Error for CsvConversionError {}

fn convert_result_entries_to_csv_string(
    result_entries: Vec<CsvResultEntry>,
) -> Result<String, CsvConversionError> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    for entry in result_entries {
        wtr.serialize(entry).map_err(CsvConversionError::Csv)?;
    }

    let data = String::from_utf8(
        wtr.into_inner()
            .map_err(|e| CsvConversionError::Io(e.into_error()))?,
    )
    .map_err(CsvConversionError::FromUtf8)?;

    Ok(data)
}

#[derive(Debug, Clone)]
pub struct CsvOutputFormatter;

impl OutputFormatter for CsvOutputFormatter {
    fn write(
        &self,
        results: &[RawResultEntry],
        _ctx: OutputFormatterContext,
        w: &mut dyn io::Write,
    ) -> Result<(), OutputFormatterError> {
        let csv_entries: Vec<CsvResultEntry> =
            results.iter().cloned().map(CsvResultEntry::from).collect();

        let csv_string = convert_result_entries_to_csv_string(csv_entries)
            .map_err::<OutputFormatterError, _>(From::from)?;

        writeln!(w, "{}", csv_string).map_err::<OutputFormatterError, _>(From::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::MeasureResult;
    use crate::result::TimeResult;
    use std::net::Ipv4Addr;
    use std::time::Duration;

    #[test]
    fn test_conversion() {
        let result_entries = vec![
            RawResultEntry::from(vec![
                MeasureResult {
                    name: String::from("Google"),
                    ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(144, 144, 144, 144)),
                    time: TimeResult::Succeeded(Duration::new(0, 100)),
                },
                MeasureResult {
                    name: String::from("Google"),
                    ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(144, 144, 144, 144)),
                    time: TimeResult::Succeeded(Duration::new(0, 200)),
                },
                MeasureResult {
                    name: String::from("Google"),
                    ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(144, 144, 144, 144)),
                    time: TimeResult::Failed(String::from("Timeout")),
                },
            ]),
            RawResultEntry::from(vec![
                MeasureResult {
                    name: String::from("Cloudflare"),
                    ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(145, 145, 145, 145)),
                    time: TimeResult::Succeeded(Duration::new(0, 50)),
                },
                MeasureResult {
                    name: String::from("Cloudflare"),
                    ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(145, 145, 145, 145)),
                    time: TimeResult::Succeeded(Duration::new(0, 60)),
                },
                MeasureResult {
                    name: String::from("Cloudflare"),
                    ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(145, 145, 145, 145)),
                    time: TimeResult::Succeeded(Duration::new(0, 70)),
                },
            ]),
        ];
        let csv_string = convert_result_entries_to_csv_string(
            result_entries
                .iter()
                .cloned()
                .map(CsvResultEntry::from)
                .collect(),
        )
        .unwrap();
        let expected_csv = "\
            name,ip,last_resolved_ip,total_requests,successful_requests,successful_requests_percentage,min_duration_value_ms,min_duration_error,max_duration_value_ms,max_duration_error,avg_duration_value_ms,avg_duration_error\n\
            Google,8.8.8.8,144.144.144.144,3,2,66.66667,0.000100,,0.000200,,0.000150,\n\
            Cloudflare,1.1.1.1,145.145.145.145,3,3,100.0,0.000050,,0.000070,,0.000060,\n";
        assert_eq!(csv_string, expected_csv);
    }
}
