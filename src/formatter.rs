use anyhow::{bail, Result};
use std::fmt;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use textwrap::{fill, Options};

use crate::minute::Minute;

static INITIAL_INDENT: &str = "  \" ";
static SUBSEQUENT_INDENT: &str = "    ";
static FOOTER_INDENT: &str = "        ";

pub static FORMATTING_HELP: &str = r#"Formatting in the form of '<style> <colour>' or just '<colour>', such as 'bold red' or 'blue'.

Available colours are: black, white, blue, cyan, green, magenta, red and yellow
Available styles are: italic, bold, strikethrough, underline, intense and dimmed
"#;

#[derive(Debug, Clone)]
pub enum Style {
    Bold,
    Dimmed,
    Intense,
    Italic,
    Plain,
    Strikethrough,
    Underline,
}

impl Style {
    fn name(&self) -> &str {
        match self {
            Self::Bold => "bold",
            Self::Dimmed => "dimmed",
            Self::Intense => "intense",
            Self::Italic => "italic",
            Self::Plain => "plain",
            Self::Strikethrough => "strikethrough",
            Self::Underline => "underline",
        }
    }
}

impl TryFrom<&str> for Style {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "bold" => Ok(Self::Bold),
            "dimmed" => Ok(Self::Dimmed),
            "intense" => Ok(Self::Intense),
            "italic" => Ok(Self::Italic),
            "plain" => Ok(Self::Plain),
            "strikethrough" => Ok(Self::Strikethrough),
            "underline" => Ok(Self::Underline),
            _ => bail!("Unknown style: {}\n\n{}", value, FORMATTING_HELP),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Formatting {
    pub colour: Color,
    pub style: Style,
}

impl From<Color> for Formatting {
    fn from(color: Color) -> Self {
        Self {
            style: Style::Plain,
            colour: color,
        }
    }
}

impl Into<ColorSpec> for Formatting {
    fn into(self) -> ColorSpec {
        let mut spec = ColorSpec::new();

        spec.set_fg(Some(self.colour));

        match self.style {
            Style::Bold => spec.set_bold(true),
            Style::Dimmed => spec.set_dimmed(true),
            Style::Intense => spec.set_intense(true),
            Style::Italic => spec.set_italic(true),
            Style::Plain => &spec,
            Style::Strikethrough => spec.set_strikethrough(true),
            Style::Underline => spec.set_underline(true),
        };

        spec
    }
}

impl fmt::Display for Formatting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colour_name = match self.colour {
            Color::Black => "black",
            Color::Blue => "blue",
            Color::Cyan => "cyan",
            Color::Green => "green",
            Color::Magenta => "magenta",
            Color::Red => "red",
            Color::White => "white",
            Color::Yellow => "yellow",
            _ => panic!("Unsupported colour"),
        };
        match self.style {
            Style::Plain => write!(f, "{}", colour_name),
            _ => write!(f, "{} {}", self.style.name(), colour_name),
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
    ) -> Result<()> {
        let quote = format!("{}\x00{}\x00{}", self.start, self.time, self.end);
        let footer = format!("{} – {}", self.author, self.title);

        let quote_options = Options::new(width)
            .initial_indent(INITIAL_INDENT)
            .subsequent_indent(SUBSEQUENT_INDENT);
        let footer_options = Options::new(width)
            .initial_indent(FOOTER_INDENT)
            .subsequent_indent(FOOTER_INDENT);

        let quote = fill(quote.as_str(), quote_options);
        let footer = fill(footer.as_str(), footer_options);

        // Split the quote into three sections, which will be the start, time and end
        let parts: Vec<_> = quote.split('\x00').collect();

        let main_spec: ColorSpec = main.clone().into();
        let time_spec: ColorSpec = time.clone().into();
        let author_spec: ColorSpec = author.clone().into();

        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        // Initial line
        writeln!(&mut stdout)?;

        // First part of main colour
        stdout.set_color(&main_spec)?;
        write!(&mut stdout, "{}", parts[0])?;

        // The time itself
        stdout.set_color(&time_spec)?;
        write!(&mut stdout, "{}", parts[1])?;

        // Rest of the main colour
        stdout.set_color(&main_spec)?;
        write!(&mut stdout, "{}", parts[2])?;

        // Two lines between quote and author
        writeln!(&mut stdout)?;
        writeln!(&mut stdout)?;

        // Author
        stdout.set_color(&author_spec)?;
        write!(&mut stdout, "{}", footer)?;

        // End with new line
        writeln!(&mut stdout)?;

        Ok(())
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
            &Color::Black.into(),
            &Color::Red.into(),
            &Color::White.into(),
        );
        let expected = [
            format!(
                "\n  \" {}black black",
                Formatting::from(Color::Black)
            ),
            format!(
                "    black {}red red",
                Formatting::from(Color::Red)
            ),
            format!(
                "    red red{} black",
                Formatting::from(Color::Black)
            ),
            format!(
                "    black black{}\n",
                Formatting::from(Color::Reset)
            ),
            format!(
                "        {}author –",
                Formatting::from(Color::White)
            ),
            format!(
                "        title{}\n",
                Formatting::from(Color::Reset)
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
