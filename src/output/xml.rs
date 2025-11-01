use crate::output::OutputFormatter;
use crate::output::OutputFormatterContext;
use crate::output::OutputFormatterError;
use crate::result::RawResultEntry;
use crate::result::TimeResult;
use std::fmt;
use std::io;
use std::net::IpAddr;
use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
struct XmlResultEntry {
    name: String,
    ip: IpAddr,
    last_resolved_ip: IpAddr,
    total_requests: i32,
    successful_requests: i32,
    successful_requests_percentage: f32,
    first_duration: TimeResult,
    average_duration: TimeResult,
}

impl XmlResultEntry {
    fn write_as_xml(
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

fn convert_result_entries_to_xml_string(
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

#[derive(Debug, Clone)]
pub struct XmlOutputFormatter;

impl OutputFormatter for XmlOutputFormatter {
    fn write(
        &self,
        results: &[RawResultEntry],
        _ctx: OutputFormatterContext,
        w: &mut dyn io::Write,
    ) -> Result<(), OutputFormatterError> {
        let xml_result_entries: Vec<XmlResultEntry> =
            results.iter().cloned().map(XmlResultEntry::from).collect();

        let xml_string = convert_result_entries_to_xml_string(xml_result_entries)
            .map_err::<OutputFormatterError, _>(From::from)?;

        writeln!(w, "{}", xml_string).map_err::<OutputFormatterError, _>(From::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::time::Duration;

    #[test]
    fn test_conversion() {
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
}
