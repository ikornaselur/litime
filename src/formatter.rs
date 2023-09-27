use anyhow::{bail, Result};
use std::fmt;
use textwrap::{fill, Options};

use crate::minute::Minute;

static INITIAL_INDENT: &str = "  \" ";
static SUBSEQUENT_INDENT: &str = "    ";
static FOOTER_INDENT: &str = "        ";

#[derive(Debug, Clone)]
pub enum Colour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

impl TryFrom<&str> for Colour {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "magenta" => Ok(Self::Magenta),
            "cyan" => Ok(Self::Cyan),
            "white" => Ok(Self::White),
            "bright-black" => Ok(Self::BrightBlack),
            "bright-red" => Ok(Self::BrightRed),
            "bright-green" => Ok(Self::BrightGreen),
            "bright-yellow" => Ok(Self::BrightYellow),
            "bright-blue" => Ok(Self::BrightBlue),
            "bright-magenta" => Ok(Self::BrightMagenta),
            "bright-cyan" => Ok(Self::BrightCyan),
            "bright-white" => Ok(Self::BrightWhite),
            _ => bail!("Unknown colour: {}", value),
        }
    }
}

impl Colour {
    fn as_escape_str(&self) -> &str {
        match self {
            Self::Black => "\u{1b}[30m",
            Self::Red => "\u{1b}[31m",
            Self::Green => "\u{1b}[32m",
            Self::Yellow => "\u{1b}[33m",
            Self::Blue => "\u{1b}[34m",
            Self::Magenta => "\u{1b}[35m",
            Self::Cyan => "\u{1b}[36m",
            Self::White => "\u{1b}[37m",
            Self::BrightBlack => "\u{1b}[90m",
            Self::BrightRed => "\u{1b}[91m",
            Self::BrightGreen => "\u{1b}[92m",
            Self::BrightYellow => "\u{1b}[93m",
            Self::BrightBlue => "\u{1b}[94m",
            Self::BrightMagenta => "\u{1b}[95m",
            Self::BrightCyan => "\u{1b}[96m",
            Self::BrightWhite => "\u{1b}[97m",
            Self::Reset => "\u{1b}[0m",
        }
    }

    fn into_name(&self) -> &str {
        match self {
            Colour::Black => "black",
            Colour::Red => "red",
            Colour::Green => "green",
            Colour::Yellow => "yellow",
            Colour::Blue => "blue",
            Colour::Magenta => "magenta",
            Colour::Cyan => "cyan",
            Colour::White => "white",
            Colour::BrightBlack => "bright-black",
            Colour::BrightRed => "bright-red",
            Colour::BrightGreen => "bright-green",
            Colour::BrightYellow => "bright-yellow",
            Colour::BrightBlue => "bright-blue",
            Colour::BrightMagenta => "bright-magenta",
            Colour::BrightCyan => "bright-cyan",
            Colour::BrightWhite => "bright-white",
            Colour::Reset => "reset",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Style {
    Plain,
    Bold,
    Italic,
    Underline,
}

impl TryFrom<&str> for Style {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "plain" => Ok(Self::Plain),
            "bold" => Ok(Self::Bold),
            "italic" => Ok(Self::Italic),
            "underline" => Ok(Self::Underline),
            _ => bail!("Unknown formatting: {}", value),
        }
    }
}

impl Style {
    fn as_escape_str(&self) -> &str {
        match self {
            Self::Plain => "\u{1b}[0m",
            Self::Bold => "\u{1b}[1m",
            Self::Italic => "\u{1b}[3m",
            Self::Underline => "\u{1b}[4m",
        }
    }

    fn into_name(&self) -> &str {
        match self {
            Style::Plain => "plain",
            Style::Bold => "bold",
            Style::Italic => "italic",
            Style::Underline => "underline",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Formatting {
    pub style: Style,
    pub colour: Colour,
}

impl Formatting {
    fn as_escape_string(&self) -> String {
        format!(
            "{}{}",
            self.style.as_escape_str(),
            self.colour.as_escape_str()
        )
    }
}

impl fmt::Display for Formatting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.style {
            Style::Plain => write!(f, "{}", self.colour.into_name()),
            _ => write!(f, "{} {}", self.style.into_name(), self.colour.into_name()),
        }
    }
}

impl From<Colour> for Formatting {
    fn from(colour: Colour) -> Self {
        Self {
            style: Style::Plain,
            colour,
        }
    }
}

impl Minute<'_> {
    pub fn formatted(
        &self,
        width: usize,
        main: &Formatting,
        time: &Formatting,
        author: &Formatting,
    ) -> String {
        let quote = format!("\x02{}\x03{}\x02{}\x00", self.start, self.time, self.end);
        let footer = format!("\x01{} – {}\x00", self.author, self.title);

        let quote_options = Options::new(width)
            .initial_indent(INITIAL_INDENT)
            .subsequent_indent(SUBSEQUENT_INDENT);
        let footer_options = Options::new(width)
            .initial_indent(FOOTER_INDENT)
            .subsequent_indent(FOOTER_INDENT);

        let quote = fill(quote.as_str(), quote_options);
        let footer = fill(footer.as_str(), footer_options);

        format!("\n{}\n\n{}\n", quote, footer)
            .replace('\x00', Style::Plain.as_escape_str())
            .replace('\x01', &author.as_escape_string())
            .replace('\x02', &main.as_escape_string())
            .replace('\x03', &time.as_escape_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn wrapped_quote() {
        let minute = Minute {
            start: "black black black ",
            time: "red red red red",
            end: " black black black",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(
            20,
            &Colour::BrightBlack.into(),
            &Colour::Red.into(),
            &Colour::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}black black",
                Formatting::from(Colour::BrightBlack).as_escape_string()
            ),
            format!(
                "    black {}red red",
                Formatting::from(Colour::Red).as_escape_string()
            ),
            format!(
                "    red red{} black",
                Formatting::from(Colour::BrightBlack).as_escape_string()
            ),
            format!(
                "    black black{}\n",
                Formatting::from(Colour::Reset).as_escape_string()
            ),
            format!(
                "        {}author –",
                Formatting::from(Colour::White).as_escape_string()
            ),
            format!(
                "        title{}\n",
                Formatting::from(Colour::Reset).as_escape_string()
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn short_quote() {
        let minute = Minute {
            start: "foo ",
            time: "bar",
            end: " baz",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(
            50,
            &Colour::BrightBlack.into(),
            &Colour::Red.into(),
            &Colour::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}foo {}bar{} baz{}\n",
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Red).as_escape_string(),
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
            format!(
                "        {}author – title{}\n",
                Formatting::from(Colour::White).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn no_start() {
        let minute = Minute {
            start: "",
            time: "bar",
            end: " baz",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(
            50,
            &Colour::BrightBlack.into(),
            &Colour::Red.into(),
            &Colour::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}{}bar{} baz{}\n",
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Red).as_escape_string(),
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
            format!(
                "        {}author – title{}\n",
                Formatting::from(Colour::White).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn no_end() {
        let minute = Minute {
            start: "foo ",
            time: "bar",
            end: "",
            author: "author",
            title: "title",
        };

        let formatted = minute.formatted(
            50,
            &Colour::BrightBlack.into(),
            &Colour::Red.into(),
            &Colour::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}foo {}bar{}{}\n",
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Red).as_escape_string(),
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
            format!(
                "        {}author – title{}\n",
                Formatting::from(Colour::White).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn issue_4() {
        let minute = Minute {
            start: "At 10.15 Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked Linda at twenty-five past eleven what time it was. Linda looked at her watch and replied that it was a ",
            time: "quarter to twelve",
            end: ".",
            author: "Agatha Christie",
            title: "Evil under the Sun",
        };

        let formatted = minute.formatted(
            50,
            &Colour::BrightBlack.into(),
            &Colour::Red.into(),
            &Colour::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}At 10.15 Arlena departed from her rondezvous,",
                Formatting::from(Colour::BrightBlack).as_escape_string()
            ),
            String::from("    a minute or two later Patrick Redfern came"),
            String::from("    down and registered surprise, annoyance, etc."),
            String::from("    Christine\'s task was easy enough. Keeping her"),
            String::from("    own watch concealed she asked Linda at twenty-"),
            String::from("    five past eleven what time it was. Linda"),
            String::from("    looked at her watch and replied that it was a"),
            format!(
                "    {}quarter to twelve{}.{}\n",
                Formatting::from(Colour::Red).as_escape_string(),
                Formatting::from(Colour::BrightBlack).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string()
            ),
            format!(
                "        {}Agatha Christie – Evil under the Sun{}\n",
                Formatting::from(Colour::White).as_escape_string(),
                Formatting::from(Colour::Reset).as_escape_string()
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }

    #[test]
    fn issue_6() {
        let minute = Minute {
            start: "And the first stop had been at ",
            time: "1.16pm",
            end: " which was 17 minutes later.",
            author: "Mark Haddon",
            title: "The Curious Incident of the Dog in the Night-Time",
        };

        let formatted = minute.formatted(
            30,
            &Colour::BrightBlack.into(),
            &Colour::Red.into(),
            &Colour::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}And the first stop had",
                Formatting::from(Colour::BrightBlack).as_escape_string(),
            ),
            format!(
                "    been at {}1.16pm{} which was",
                Formatting::from(Colour::Red).as_escape_string(),
                Formatting::from(Colour::BrightBlack).as_escape_string(),
            ),
            format!(
                "    17 minutes later.{}\n",
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
            format!(
                "        {}Mark Haddon – The",
                Formatting::from(Colour::White).as_escape_string(),
            ),
            String::from("        Curious Incident of"),
            String::from("        the Dog in the Night-"),
            format!(
                "        Time{}\n",
                Formatting::from(Colour::Reset).as_escape_string(),
            ),
        ]
        .join("\n");

        assert_eq!(formatted, expected);
    }
}
