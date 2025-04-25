use std::fmt;
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::string::FromUtf8Error;
use std::time::Duration;
use tabled::settings as tabled_settings;
use tabled::Tabled;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TimeResult
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum TimeResult {
    #[serde(rename = "succeeded")]
    Succeeded(Duration),
    #[serde(rename = "failed")]
    Failed(String),
}

impl TimeResult {
    pub fn is_succeeded(&self) -> bool {
        matches!(self, TimeResult::Succeeded(_))
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, TimeResult::Failed(_))
    }

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
// TabledResultEntry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Tabled)]
pub struct TabledResultEntry {
    #[tabled(rename = "Server name")]
    pub name: String,
    #[tabled(rename = "IP address")]
    pub ip: IpAddr,
    #[tabled(rename = "Last resolved IP")]
    pub last_resolved_ip: IpAddr,
    /// String with the following format: "successful_requests/total_requests (success_rate))"
    #[tabled(rename = "Success rate")]
    pub successful_requests: String,
    #[tabled(skip)]
    pub successful_requests_color: tabled_settings::Color,
    #[tabled(rename = "First duration")]
    pub first_duration: TimeResult,
    #[tabled(skip)]
    pub first_duration_color: tabled_settings::Color,
    #[tabled(rename = "Average duration")]
    pub average_duration: TimeResult,
    #[tabled(skip)]
    pub average_duration_color: tabled_settings::Color,
}

impl From<RawResultEntry> for TabledResultEntry {
    fn from(value: RawResultEntry) -> Self {
        TabledResultEntry {
            name: value.name,
            ip: value.ip,
            last_resolved_ip: value.last_resolved_ip,
            successful_requests: format!(
                "{}/{} ({:.2}%)",
                value.successful_requests,
                value.total_requests,
                value.successful_requests_percentage
            ),
            successful_requests_color: value.successful_requests_color,
            first_duration: value.first_duration,
            first_duration_color: value.first_duration_color,
            average_duration: value.average_duration,
            average_duration_color: value.average_duration_color,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// JsonResultEntry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JsonResultEntry {
    pub name: String,
    pub ip: IpAddr,
    pub last_resolved_ip: IpAddr,
    pub total_requests: i32,
    pub successful_requests: i32,
    pub successful_requests_percentage: f32,
    pub first_duration: TimeResult,
    pub average_duration: TimeResult,
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
            first_duration: value.first_duration,
            average_duration: value.average_duration,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// XmlResultEntry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct XmlResultEntry {
    pub name: String,
    pub ip: IpAddr,
    pub last_resolved_ip: IpAddr,
    pub total_requests: i32,
    pub successful_requests: i32,
    pub successful_requests_percentage: f32,
    pub first_duration: TimeResult,
    pub average_duration: TimeResult,
}

impl XmlResultEntry {
    pub fn write_as_xml(
        self,
        writer: &mut quick_xml::writer::Writer<io::Cursor<Vec<u8>>>,
    ) -> io::Result<()> {
        writer
            .create_element("ResultEntry")
            .write_inner_content(|entry_writer| {
                entry_writer
                    .create_element("Name")
                    .write_text_content(quick_xml::events::BytesText::new(&self.name))?;
                entry_writer.create_element("Ip").write_text_content(
                    quick_xml::events::BytesText::new(self.ip.to_string().as_str()),
                )?;
                entry_writer
                    .create_element("LastResolvedIp")
                    .write_text_content(quick_xml::events::BytesText::new(
                        self.last_resolved_ip.to_string().as_str(),
                    ))?;
                entry_writer
                    .create_element("SuccessfulRequests")
                    .write_inner_content(|srwriter| {
                        srwriter
                            .create_element("TotalRequests")
                            .write_text_content(quick_xml::events::BytesText::new(
                                self.total_requests.to_string().as_str(),
                            ))?;
                        srwriter
                            .create_element("SuccessfulRequests")
                            .write_text_content(quick_xml::events::BytesText::new(
                                self.successful_requests.to_string().as_str(),
                            ))?;
                        srwriter
                            .create_element("SuccessfulRequestsPercentage")
                            .write_text_content(quick_xml::events::BytesText::new(
                                self.successful_requests_percentage.to_string().as_str(),
                            ))?;
                        Ok(())
                    })?;
                entry_writer
                    .create_element("FirstDuration")
                    .with_attribute(("type", self.first_duration.get_xml_type_str()))
                    .write_text_content(quick_xml::events::BytesText::new(
                        self.first_duration.to_string().as_str(),
                    ))?;
                entry_writer
                    .create_element("AverageDuration")
                    .with_attribute(("type", self.average_duration.get_xml_type_str()))
                    .write_text_content(quick_xml::events::BytesText::new(
                        self.average_duration.to_string().as_str(),
                    ))?;

                Ok(())
            })?;

        Ok(())
    }
}

impl From<RawResultEntry> for XmlResultEntry {
    fn from(value: RawResultEntry) -> Self {
        XmlResultEntry {
            name: value.name,
            ip: value.ip,
            last_resolved_ip: value.last_resolved_ip,
            total_requests: value.total_requests,
            successful_requests: value.successful_requests,
            successful_requests_percentage: value.successful_requests_percentage,
            first_duration: value.first_duration,
            average_duration: value.average_duration,
        }
    }
}

#[derive(Debug)]
pub enum XmlConversionError {
    Io(io::Error),
    FromUtf8(FromUtf8Error),
}

impl fmt::Display for XmlConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlConversionError::Io(e) => write!(f, "IO error: {e}"),
            XmlConversionError::FromUtf8(e) => write!(f, "UTF-8 error: {e}"),
        }
    }
}

impl From<io::Error> for XmlConversionError {
    fn from(e: io::Error) -> Self {
        XmlConversionError::Io(e)
    }
}

impl From<FromUtf8Error> for XmlConversionError {
    fn from(e: FromUtf8Error) -> Self {
        XmlConversionError::FromUtf8(e)
    }
}

impl std::error::Error for XmlConversionError {}

pub fn convert_result_entries_to_xml_string(
    result_entries: Vec<XmlResultEntry>,
) -> Result<String, XmlConversionError> {
    let mut writer = quick_xml::writer::Writer::new(io::Cursor::new(Vec::new()));

    writer
        .create_element("DnsBenchResultEntries")
        .write_inner_content(|writer| {
            for entry in result_entries {
                entry.write_as_xml(writer)?;
            }
            Ok(())
        })
        .map_err(XmlConversionError::Io)?;

    let result = String::from_utf8(writer.into_inner().into_inner())
        .map_err(XmlConversionError::FromUtf8)?;

    Ok(result)
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// CsvResultEntry
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvResultEntry {
    pub name: String,
    pub ip: IpAddr,
    pub last_resolved_ip: IpAddr,
    pub total_requests: i32,
    pub successful_requests: i32,
    pub successful_requests_percentage: f32,
    pub first_duration_value_ms: Option<String>,
    pub first_duration_error: Option<String>,
    pub average_duration_value_ms: Option<String>,
    pub average_duration_error: Option<String>,
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
            first_duration_value_ms: value.first_duration.get_duration_millis(),
            first_duration_error: value.first_duration.get_error_str().map(|v| v.to_string()),
            average_duration_value_ms: value.average_duration.get_duration_millis(),
            average_duration_error: value
                .average_duration
                .get_error_str()
                .map(|v| v.to_string()),
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

pub fn convert_result_entries_to_csv_string(
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

    #[test]
    fn test_tabled_result_entry_from() {
        let raw_result_entry = RawResultEntry {
            name: String::from("Google"),
            ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            last_resolved_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            total_requests: 3,
            successful_requests: 2,
            successful_requests_percentage: 66.66667,
            successful_requests_color: tabled_settings::Color::FG_BRIGHT_YELLOW,
            first_duration: TimeResult::Succeeded(Duration::new(0, 100)),
            first_duration_color: tabled_settings::Color::FG_BRIGHT_GREEN,
            average_duration: TimeResult::Succeeded(Duration::new(0, 150)),
            average_duration_color: tabled_settings::Color::FG_BRIGHT_GREEN,
        };

        let tabled_result_entry = TabledResultEntry::from(raw_result_entry);

        assert_eq!(tabled_result_entry.name, "Google");
        assert_eq!(
            tabled_result_entry.ip,
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))
        );
        assert_eq!(
            tabled_result_entry.last_resolved_ip,
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))
        );
        assert_eq!(tabled_result_entry.successful_requests, "2/3 (66.67%)");
        assert_eq!(
            tabled_result_entry.successful_requests_color,
            tabled_settings::Color::FG_BRIGHT_YELLOW
        );
        assert_eq!(
            tabled_result_entry.first_duration,
            TimeResult::Succeeded(Duration::new(0, 100))
        );
        assert_eq!(
            tabled_result_entry.first_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
        assert_eq!(
            tabled_result_entry.average_duration,
            TimeResult::Succeeded(Duration::new(0, 150))
        );
        assert_eq!(
            tabled_result_entry.average_duration_color,
            tabled_settings::Color::FG_BRIGHT_GREEN
        );
    }

    #[test]
    fn test_convert_result_entries_to_xml_string() {
        let result_entries = vec![
            XmlResultEntry {
                name: String::from("Google"),
                ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                last_resolved_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                total_requests: 3,
                successful_requests: 2,
                successful_requests_percentage: 66.66667,
                first_duration: TimeResult::Succeeded(Duration::new(0, 100)),
                average_duration: TimeResult::Succeeded(Duration::new(0, 150)),
            },
            XmlResultEntry {
                name: String::from("Cloudflare"),
                ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                last_resolved_ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                total_requests: 3,
                successful_requests: 3,
                successful_requests_percentage: 100.0,
                first_duration: TimeResult::Succeeded(Duration::new(0, 50)),
                average_duration: TimeResult::Succeeded(Duration::new(0, 60)),
            },
        ];
        let xml_string = convert_result_entries_to_xml_string(result_entries).unwrap();
        let expected_string = "<DnsBenchResultEntries><ResultEntry><Name>Google</Name><Ip>8.8.8.8</Ip><LastResolvedIp>8.8.8.8</LastResolvedIp><SuccessfulRequests><TotalRequests>3</TotalRequests><SuccessfulRequests>2</SuccessfulRequests><SuccessfulRequestsPercentage>66.66667</SuccessfulRequestsPercentage></SuccessfulRequests><FirstDuration type=\"succeeded\">100ns</FirstDuration><AverageDuration type=\"succeeded\">150ns</AverageDuration></ResultEntry><ResultEntry><Name>Cloudflare</Name><Ip>1.1.1.1</Ip><LastResolvedIp>1.1.1.1</LastResolvedIp><SuccessfulRequests><TotalRequests>3</TotalRequests><SuccessfulRequests>3</SuccessfulRequests><SuccessfulRequestsPercentage>100</SuccessfulRequestsPercentage></SuccessfulRequests><FirstDuration type=\"succeeded\">50ns</FirstDuration><AverageDuration type=\"succeeded\">60ns</AverageDuration></ResultEntry></DnsBenchResultEntries>";
        assert_eq!(xml_string, expected_string);
    }

    #[test]
    fn test_convert_result_entries_to_csv_string() {
        let result_entries = vec![
            RawResultEntry::from(vec![
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
            ]),
            RawResultEntry::from(vec![
                MeasureResult {
                    name: String::from("Cloudflare"),
                    ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    time: TimeResult::Succeeded(Duration::new(0, 50)),
                },
                MeasureResult {
                    name: String::from("Cloudflare"),
                    ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    time: TimeResult::Succeeded(Duration::new(0, 60)),
                },
                MeasureResult {
                    name: String::from("Cloudflare"),
                    ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    resolved_ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
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
        let expected_csv = "name,ip,last_resolved_ip,total_requests,successful_requests,successful_requests_percentage,first_duration_value_ms,first_duration_error,average_duration_value_ms,average_duration_error\nGoogle,8.8.8.8,8.8.8.8,3,2,66.66667,0.000100,,0.000150,\nCloudflare,1.1.1.1,1.1.1.1,3,3,100.0,0.000050,,0.000060,\n";
        assert_eq!(csv_string, expected_csv);
    }
}
