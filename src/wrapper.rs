use textwrap::Wrapper;

use minute::Minute;

static INITIAL_INDENT: &'static str = "  \" ";
static SUBSEQUENT_INDENT: &'static str = "    ";
static FOOTER_INDENT: &'static str = "        ";

static RESET: &'static str = "\u{1b}[0m"; // 0x00
static WHITE: &'static str = "\u{1b}[97m"; // 0x01
static BLACK: &'static str = "\u{1b}[90m"; // 0x02
static RED: &'static str = "\u{1b}[31m"; // 0x03

impl Minute {
    pub fn wrapped(&self, width: usize) -> String {
        let quote = format!("\x02{}\x03{}\x02{}\x00", self.start, self.time, self.end);
        let footer = format!("\x01{} – {}\x00", self.author, self.title);

        let quote_wrapper = Wrapper::new(width)
            .initial_indent(INITIAL_INDENT)
            .subsequent_indent(SUBSEQUENT_INDENT);
        let footer_wrapper = Wrapper::new(width)
            .initial_indent(FOOTER_INDENT)
            .subsequent_indent(FOOTER_INDENT);

        let quote = quote_wrapper.wrap(quote.as_str()).join("\n");
        let footer = footer_wrapper.wrap(footer.as_str()).join("\n");

        format!("\n{}\n\n{}\n", quote, footer)
            .replace('\x00', RESET)
            .replace('\x01', WHITE)
            .replace('\x02', BLACK)
            .replace('\x03', RED)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrapped_quote() {
        let minute = Minute {
            start: "black black black ",
            time: "red red red red",
            end: " black black black",
            author: "author",
            title: "title",
        };

        let wrapped = minute.wrapped(20);
        let expected = [
            format!("\n  \" {}black black", BLACK),
            format!("    black {}red red", RED),
            format!("    red red{} black", BLACK),
            format!("    black black{}\n", RESET),
            format!("        {}author –", WHITE),
            format!("        title{}\n", RESET),
        ].join("\n");

        assert_eq!(wrapped, expected);
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

        let wrapped = minute.wrapped(50);
        let expected = [
            format!("\n  \" {}foo {}bar{} baz{}\n", BLACK, RED, BLACK, RESET),
            format!("        {}author – title{}\n", WHITE, RESET),
        ].join("\n");

        assert_eq!(wrapped, expected);
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

        let wrapped = minute.wrapped(50);
        let expected = [
            format!("\n  \" {}{}bar{} baz{}\n", BLACK, RED, BLACK, RESET),
            format!("        {}author – title{}\n", WHITE, RESET),
        ].join("\n");

        assert_eq!(wrapped, expected);
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

        let wrapped = minute.wrapped(50);
        let expected = [
            format!("\n  \" {}foo {}bar{}{}\n", BLACK, RED, BLACK, RESET),
            format!("        {}author – title{}\n", WHITE, RESET),
        ].join("\n");

        assert_eq!(wrapped, expected);
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

        let wrapped = minute.wrapped(50);
        let expected = [
            format!(
                "\n  \" {}At 10.15 Arlena departed from her rondezvous,",
                BLACK
            ),
            String::from("    a minute or two later Patrick Redfern came"),
            String::from("    down and registered surprise, annoyance, etc."),
            String::from("    Christine\'s task was easy enough. Keeping her"),
            String::from("    own watch concealed she asked Linda at twenty-"),
            String::from("    five past eleven what time it was."),
            String::from("    Linda looked at her watch and replied that it"),
            format!("    was a {}quarter to twelve{}.{}\n", RED, BLACK, RESET),
            format!(
                "        {}Agatha Christie – Evil under the Sun{}\n",
                WHITE, RESET
            ),
        ].join("\n");

        assert_eq!(wrapped, expected);
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

        let wrapped = minute.wrapped(30);
        let expected = [
            format!("\n  \" {}And the first stop had", BLACK),
            format!("    been at {}1.16pm{} which was", RED, BLACK),
            format!("    17 minutes later.{}\n", RESET),
            format!("        {}Mark Haddon – The", WHITE),
            String::from("        Curious Incident of"),
            String::from("        the Dog in the Night-"),
            format!("        Time{}\n", RESET),
        ].join("\n");

        assert_eq!(wrapped, expected);
    }
}
