use crate::args::Style;
use crate::output::OutputFormatter;
use crate::output::OutputFormatterContext;
use crate::output::OutputFormatterError;
use crate::result::RawResultEntry;
use crate::result::TimeResult;
use std::io;
use std::net::IpAddr;
use tabled::settings as tabled_settings;
use tabled::Table;
use tabled::Tabled;

#[derive(Debug, Clone, Tabled)]
struct TabledResultEntry {
    #[tabled(rename = "Server name")]
    name: String,
    #[tabled(rename = "IP address")]
    ip: IpAddr,
    #[tabled(rename = "Last resolved IP")]
    last_resolved_ip: IpAddr,
    /// String with the following format: "successful_requests/total_requests (success_rate)"
    #[tabled(rename = "Success rate")]
    successful_requests: String,
    #[tabled(skip)]
    successful_requests_color: tabled_settings::Color,
    #[tabled(rename = "First duration")]
    first_duration: TimeResult,
    #[tabled(skip)]
    first_duration_color: tabled_settings::Color,
    #[tabled(rename = "Average duration")]
    average_duration: TimeResult,
    #[tabled(skip)]
    average_duration_color: tabled_settings::Color,
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

#[derive(Debug, Clone)]
pub struct TableOutputFormatter;

impl OutputFormatter for TableOutputFormatter {
    fn write(
        &self,
        results: &[RawResultEntry],
        ctx: OutputFormatterContext,
        w: &mut dyn io::Write,
    ) -> Result<(), OutputFormatterError> {
        let system_dns_ips = ctx.system_dns_ips.unwrap_or_default();
        let tabled_result_entries = results
            .iter()
            .cloned()
            .map(|entry| {
                let mut tre = TabledResultEntry::from(entry);
                if system_dns_ips.contains(&tre.ip) {
                    tre.name = format!("> {}", tre.name);
                }
                tre
            })
            .collect::<Vec<TabledResultEntry>>();
        let mut table = Table::new(tabled_result_entries.clone());

        match ctx.config.style {
            Style::Empty => table.with(tabled_settings::Style::empty()),
            Style::Blank => table.with(tabled_settings::Style::blank()),
            Style::Ascii => table.with(tabled_settings::Style::ascii()),
            Style::Psql => table.with(tabled_settings::Style::psql()),
            Style::Markdown => table.with(tabled_settings::Style::markdown()),
            Style::Modern => table.with(tabled_settings::Style::modern()),
            Style::Sharp => table.with(tabled_settings::Style::sharp()),
            Style::Rounded => table.with(tabled_settings::Style::rounded()),
            Style::ModernRounded => table.with(tabled_settings::Style::modern_rounded()),
            Style::Extended => table.with(tabled_settings::Style::extended()),
            Style::Dots => table.with(tabled_settings::Style::dots()),
            Style::ReStructuredText => table.with(tabled_settings::Style::re_structured_text()),
            Style::AsciiRounded => table.with(tabled_settings::Style::ascii_rounded()),
        };

        for (i, entry) in tabled_result_entries.iter().enumerate() {
            table.with(
                tabled_settings::Modify::new(tabled_settings::object::Cell::new(i + 1, 3))
                    .with(entry.successful_requests_color.clone()),
            );
            table.with(
                tabled_settings::Modify::new(tabled_settings::object::Cell::new(i + 1, 4))
                    .with(entry.first_duration_color.clone()),
            );
            table.with(
                tabled_settings::Modify::new(tabled_settings::object::Cell::new(i + 1, 5))
                    .with(entry.average_duration_color.clone()),
            );
        }

        writeln!(w, "{}", table).map_err::<OutputFormatterError, _>(From::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::time::Duration;

    #[test]
    fn test_from_raw_entry() {
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
}
