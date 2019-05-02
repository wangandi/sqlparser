// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Additional modifications to this file may have been made by Timely
// Data, Inc. See the version control log for precise modification
// information. The derived work is copyright 2019 Timely Data and
// is not licensed under the terms of the above license.

use chrono::{offset::FixedOffset, DateTime, NaiveDate, NaiveDateTime, NaiveTime};

/// SQL values such as int, double, string, timestamp
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Literal signed long
    Long(i64),
    /// Literal floating point value
    Double(f64),
    /// 'string value'
    SingleQuotedString(String),
    /// N'string value'
    NationalStringLiteral(String),
    /// Boolean value true or false,
    Boolean(bool),
    /// Date value
    Date(NaiveDate),
    // Time
    Time(NaiveTime),
    /// Date and time
    DateTime(NaiveDateTime),
    /// Timstamp with time zone
    Timestamp(DateTime<FixedOffset>),
    /// NULL value in insert statements,
    Null,
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Long(v) => v.to_string(),
            Value::Double(v) => v.to_string(),
            Value::SingleQuotedString(v) => format!("'{}'", escape_single_quote_string(v)),
            Value::NationalStringLiteral(v) => format!("N'{}'", v),
            Value::Boolean(v) => v.to_string(),
            Value::Date(v) => v.to_string(),
            Value::Time(v) => v.to_string(),
            Value::DateTime(v) => v.to_string(),
            Value::Timestamp(v) => format!("{}", v),
            Value::Null => "NULL".to_string(),
        }
    }
}

fn escape_single_quote_string(s: &str) -> String {
    let mut escaped = String::new();
    for c in s.chars() {
        if c == '\'' {
            escaped.push_str("\'\'");
        } else {
            escaped.push(c);
        }
    }
    escaped
}
