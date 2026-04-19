use chrono::Duration;
use nu_ansi_term::Color;

pub fn format_duration(duration: &Duration) -> String {
    let mut duration_string = String::new();

    if duration.num_hours() > 0 {
        duration_string.push_str(&format!("{}h ", duration.num_hours()));
    }

    if duration.num_minutes() > 0 {
        duration_string.push_str(&format!("{:0>2}m", duration.num_minutes() % 60));
    } else {
        #[cfg(not(feature = "second-precision"))]
        duration_string.push_str("<1m");
        #[cfg(feature = "second-precision")]
        duration_string.push_str(&format!("{:0>2}s", duration.num_seconds() % 60));
    }

    duration_string
}

pub fn print_activity_update(
    text: &str,
    description: &str,
    project: &str,
    start: &str,
    duration: Option<&str>,
) {
    println!(
        "{}: \"{}\" ({}{}{}) at {}{}{} {}",
        text,
        description,
        Color::Green.prefix(),
        project,
        Color::Green.suffix(),
        Color::Purple.prefix(),
        start,
        Color::Purple.suffix(),
        duration
            .map(|d| format!("({})", d))
            .unwrap_or("".to_owned()),
    );
}
