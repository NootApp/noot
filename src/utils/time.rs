use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn sqlstr_to_local(str: impl Into<String>) -> DateTime<Local> {
    let time_str = str.into();

    let time_naive = NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S").unwrap();
    let time_utc = DateTime::<Utc>::from_naive_utc_and_offset(time_naive, Utc);
    let time = time_utc.with_timezone(&Local);

    time
}

pub fn local_to_sqlstr(time: DateTime<Local>) -> String {
    let time_utc = time.to_utc();
    let time_str = time_utc.format("%Y-%m-%d %H:%M:%S").to_string();
    time_str
}