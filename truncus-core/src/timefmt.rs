pub fn date(ts_millis: i64) -> String {
    time::OffsetDateTime::from_unix_timestamp(ts_millis / 1000)
        .map(|t| format!("{:04}-{:02}-{:02}", t.year(), u8::from(t.month()), t.day()))
        .unwrap_or_default()
}
