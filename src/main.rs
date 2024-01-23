use std::{env, error::Error, fmt, process, thread::sleep, time::Duration};

fn main() {
    if let Err(why) = idler(env::args().skip(1)) {
        eprintln!("error: {why}");
        process::exit(1);
    }
}

fn idler(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    // 480 for Spacewar
    // 606150 for Moonlighter
    let app_id = args.next().ok_or("didn't give app ID")?.parse::<u32>()?;
    let duration = args.next().ok_or("didn't give duration")?;
    let duration = parse_duration(duration)?;
    let _client = match steamworks::Client::init_app(app_id) {
        Ok(client) => client,
        Err(_) => return Err("failed to initialise Steamworks".into()),
    };
    eprintln!("Idling {app_id} for {duration:?}");
    sleep(duration);
    Ok(())
}

fn parse_duration(
    input: impl AsRef<str>,
) -> Result<Duration, ParseDurationError> {
    let input = input.as_ref();
    let mut duration = Duration::default();
    let mut slice_start = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            'd' | 'h' | 'm' | 's' => {
                if slice_start != i {
                    // Unwrap is okay here as we've verified all previous
                    // characters are ASCII digits
                    let value = input[slice_start..i].parse::<u64>().unwrap();
                    let scale = match c {
                        'd' => 60 * 60 * 24,
                        'h' => 60 * 60,
                        'm' => 60,
                        's' => 1,
                        _ => unreachable!(),
                    };
                    duration = duration.saturating_add(Duration::from_secs(
                        value.saturating_mul(scale),
                    ));
                    slice_start = i + 1;
                } else {
                    return Err(ParseDurationError::Valueless(c));
                }
            },
            '0'..='9' => {},
            _ => return Err(ParseDurationError::Unexpected(c)),
        }
    }
    Ok(duration)
}

#[derive(Debug, Copy, Clone)]
enum ParseDurationError {
    Unexpected(char),
    Valueless(char),
}

impl fmt::Display for ParseDurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseDurationError::Unexpected(c) => {
                write!(f, "unexpected character {c:?}")
            },
            ParseDurationError::Valueless(c) => {
                write!(f, "missing value before {c:?}")
            },
        }
    }
}

impl Error for ParseDurationError {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::parse_duration;

    const DAY: Duration = Duration::from_secs(60 * 60 * 24);
    const HOUR: Duration = Duration::from_secs(60 * 60);
    const MINUTE: Duration = Duration::from_secs(60);

    #[test]
    fn parses_durations() {
        assert_eq!(parse_duration(""), Ok(Default::default()));
        assert_eq!(parse_duration("1h"), Ok(HOUR));
        assert_eq!(parse_duration("1h20m"), Ok(HOUR + MINUTE * 20));
        assert_eq!(parse_duration("1h20m4d"), Ok(DAY * 4 + HOUR + MINUTE * 20));

        assert!(parse_duration("asdf").is_err());
        assert!(parse_duration("365dm").is_err());
    }
}
