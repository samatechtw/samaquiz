use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref REGEX_UUID: Regex =
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
            .unwrap();
    pub static ref REGEX_DATE: Regex =
        Regex::new(r#"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?Z$"#).unwrap();
    pub static ref REGEX_CODE: Regex = Regex::new(r#"^[0-9a-zA-Z_-]{1,12}$"#).unwrap();
    pub static ref REGEX_BOOL: Regex = Regex::new(r#"^(__true__)|(__false__)$"#).unwrap();
    pub static ref REGEX_NUMBER: Regex = Regex::new(r#"^-?\d+$"#).unwrap();
    pub static ref REGEX_POSITIVE_NUMBER: Regex = Regex::new(r#"^\d+$"#).unwrap();
    pub static ref REGEX_FLOAT: Regex = Regex::new(r#"^-?\d+?\.^-?\d*$"#).unwrap();
    pub static ref REGEX_ARRAY: Regex = Regex::new(r"^\[.*\]$").unwrap();
    pub static ref REGEX_PORT: Regex = Regex::new(r":\d+$").unwrap();
    pub static ref REGEX_SITE_NAME: Regex = Regex::new(r"^[a-zA-Z0-9 ]{2,50}$").unwrap();
    pub static ref REGEX_EMAIL: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    pub static ref REGEX_TABLE_NAME: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]{1,99}$").unwrap();
}

pub fn is_uuid(str: &str) -> bool {
    REGEX_UUID.is_match(str)
}

pub fn is_date(str: &str) -> bool {
    REGEX_DATE.is_match(str)
}

pub fn is_bool(str: &str) -> bool {
    REGEX_BOOL.is_match(str)
}

pub fn is_number(str: &str) -> bool {
    REGEX_NUMBER.is_match(str)
}

pub fn is_float(str: &str) -> bool {
    REGEX_FLOAT.is_match(str)
}

pub fn is_array(str: &str) -> bool {
    REGEX_ARRAY.is_match(str)
}

pub fn is_port(str: &str) -> bool {
    REGEX_PORT.is_match(str)
}

pub fn is_email(str: &str) -> bool {
    REGEX_EMAIL.is_match(str)
}
