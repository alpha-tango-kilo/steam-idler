use std::{env, thread::sleep, time::Duration};

fn main() {
    let mut args = env::args().skip(1);
    // 480 for Spacewar
    // 606150 for Moonlighter
    let app_id = args
        .next()
        .expect("didn't give app ID")
        .parse::<u32>()
        .expect("invalid ID");
    let duration = args
        .next()
        .map(parse_time)
        .expect("didn't give duration")
        .expect("bad duration");
    let _client = steamworks::Client::init_app(app_id)
        .expect("failed to initialise steamworks");
    eprintln!("Idling {app_id} for {duration:?}");
    sleep(duration);
}

fn parse_time(input: impl AsRef<str>) -> Result<Duration, &'static str> {
    let input = input.as_ref();
    let mut duration = Duration::default();
    let mut slice_start = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            'd' | 'h' | 'm' => {
                if slice_start != i {
                    // Unwrap is okay here as we've verified all previous
                    // characters are ASCII digits
                    let value = input[slice_start..i].parse::<u64>().unwrap();
                    let scale = match c {
                        'd' => 60 * 60 * 24,
                        'h' => 60 * 60,
                        'm' => 60,
                        _ => unreachable!(),
                    };
                    duration += Duration::from_secs(value * scale);
                    slice_start = i + 1;
                } else {
                    return Err("d/h/m without value");
                }
            },
            _ if c.is_ascii_digit() => {},
            _ => return Err("unexpected character"),
        }
    }
    Ok(duration)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::parse_time;

    const DAY: Duration = Duration::from_secs(60 * 60 * 24);
    const HOUR: Duration = Duration::from_secs(60 * 60);
    const MINUTE: Duration = Duration::from_secs(60);

    #[test]
    fn time_parsing() {
        assert_eq!(parse_time(""), Ok(Default::default()));
        assert_eq!(parse_time("1h"), Ok(HOUR));
        assert_eq!(parse_time("1h20m"), Ok(HOUR + MINUTE * 20));
        assert_eq!(parse_time("1h20m4d"), Ok(DAY * 4 + HOUR + MINUTE * 20));

        assert!(parse_time("asdf").is_err());
        assert!(parse_time("365dm").is_err());
    }
}
