mod csv;
mod json;
mod table;
mod xml;

pub use csv::CsvConversionError;
pub use csv::CsvOutputFormatter;
pub use json::JsonOutputFormatter;
pub use table::TableOutputFormatter;
pub use xml::XmlConversionError;
pub use xml::XmlOutputFormatter;

use crate::args::Format;
use crate::config::DnsBenchConfig;
use crate::result::RawResultEntry;
use std::fmt;
use std::io;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct OutputFormatterContext {
    pub config: DnsBenchConfig,
    pub system_dns_ips: Option<Vec<IpAddr>>,
}

#[derive(Debug)]
pub enum OutputFormatterError {
    Io(io::Error),
    Json(serde_json::Error),
    Xml(XmlConversionError),
    Csv(CsvConversionError),
}

impl fmt::Display for OutputFormatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormatterError::Io(e) => write!(f, "IO error: {e}"),
            OutputFormatterError::Json(e) => write!(f, "JSON error: {e}"),
            OutputFormatterError::Xml(e) => write!(f, "XML error: {e}"),
            OutputFormatterError::Csv(e) => write!(f, "CSV error: {e}"),
        }
    }
}

impl From<io::Error> for OutputFormatterError {
    fn from(value: io::Error) -> Self {
        OutputFormatterError::Io(value)
    }
}

impl From<serde_json::Error> for OutputFormatterError {
    fn from(value: serde_json::Error) -> Self {
        OutputFormatterError::Json(value)
    }
}

impl From<XmlConversionError> for OutputFormatterError {
    fn from(value: XmlConversionError) -> Self {
        OutputFormatterError::Xml(value)
    }
}

impl From<CsvConversionError> for OutputFormatterError {
    fn from(value: CsvConversionError) -> Self {
        OutputFormatterError::Csv(value)
    }
}

impl std::error::Error for OutputFormatterError {}

pub trait OutputFormatter {
    fn write(
        &self,
        results: &[RawResultEntry],
        ctx: OutputFormatterContext,
        w: &mut dyn io::Write,
    ) -> Result<(), OutputFormatterError>;
}

pub fn get_output_formatter(format: &Format) -> Box<dyn OutputFormatter> {
    match format {
        Format::HumanReadable => Box::new(TableOutputFormatter {}),
        Format::Json => Box::new(JsonOutputFormatter {}),
        Format::Xml => Box::new(XmlOutputFormatter {}),
        Format::Csv => Box::new(CsvOutputFormatter {}),
    }
}
