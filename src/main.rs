use std::{
    env,
    error::Error,
    fmt,
    io::{stdout, IsTerminal, Write},
    process,
    thread::sleep,
    time::Duration,
};

const ONE_DAY_SECONDS: u64 = 60 * 60 * 24;
const ONE_HOUR_SECONDS: u64 = 60 * 60;
const ONE_MINUTE_SECONDS: u64 = 60;

const SPINNER: &[char] = &['|', '/', '-', '\\'];

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
    let mut stdout = stdout();
    if stdout.is_terminal() {
        let total_seconds = duration.as_secs();
        let duration = HumanDuration::from(duration);
        for current in 0..total_seconds {
            let spinner_char =
                SPINNER[(current % (SPINNER.len() as u64)) as usize];
            let message = format!(
                "\rIdling {app_id} for {duration}: {}, {:.1}% {spinner_char} ",
                HumanDuration::from_secs(current),
                current as f32 / total_seconds as f32 * 100f32,
            );
            stdout.write_all(message.as_bytes()).unwrap();
            stdout.flush().unwrap();
            sleep(Duration::from_secs(1));
        }
        println!();
    } else {
        eprintln!("Idling {app_id} for {duration:?}");
        sleep(duration);
    }
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
                        'd' => ONE_DAY_SECONDS,
                        'h' => ONE_HOUR_SECONDS,
                        'm' => ONE_MINUTE_SECONDS,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HumanDuration {
    days: u64,
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl HumanDuration {
    const ZERO: Self = HumanDuration {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0,
    };

    fn from_secs(secs: u64) -> Self {
        let mut remaining_seconds = secs;
        let days = remaining_seconds / ONE_DAY_SECONDS;
        remaining_seconds %= ONE_DAY_SECONDS;
        let hours = (remaining_seconds / ONE_HOUR_SECONDS) as u8;
        remaining_seconds %= ONE_HOUR_SECONDS;
        let minutes = (remaining_seconds / ONE_MINUTE_SECONDS) as u8;
        remaining_seconds %= ONE_MINUTE_SECONDS;
        let seconds = remaining_seconds as u8;
        HumanDuration {
            days,
            hours,
            minutes,
            seconds,
        }
    }
}

impl From<Duration> for HumanDuration {
    fn from(value: Duration) -> Self {
        HumanDuration::from_secs(value.as_secs())
    }
}

impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == HumanDuration::ZERO {
            return write!(f, "0s");
        }
        if self.days != 0 {
            write!(f, "{}d", self.days)?;
        }
        if self.hours != 0 {
            write!(f, "{}h", self.hours)?;
        }
        if self.minutes != 0 {
            write!(f, "{}m", self.minutes)?;
        }
        if self.seconds != 0 {
            write!(f, "{}s", self.seconds)?;
        }
        Ok(())
    }
}

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
