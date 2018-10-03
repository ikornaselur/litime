use textwrap::Wrapper;

use minute::Minute;

static INITIAL_INDENT: &'static str = "  \" ";
static SUBSEQUENT_INDENT: &'static str = "    ";
static FOOTER_INDENT: &'static str = "        ";
const INDENT_LEN: usize = 4;

static BLACK: &'static str = "\u{1b}[90m";
static RED: &'static str = "\u{1b}[31m";
static WHITE: &'static str = "\u{1b}[97m";
static RESET: &'static str = "\u{1b}[0m";

impl Minute {
    pub fn wrapped(&self, width: usize) -> String {
        let start_len = self.start.len();
        let time_len = self.time.len();

        let full = format!("{}{}{}", self.start, self.time, self.end);

        let wrapper = Wrapper::new(width)
            .initial_indent(INITIAL_INDENT)
            .subsequent_indent(SUBSEQUENT_INDENT);

        let wrapped_lines = wrapper.wrap(full.as_str());

        let mut output = String::from(BLACK);

        let mut count = start_len;
        let mut red = false;
        let mut black = false;

        for line in wrapped_lines {
            output.push('\n');
            let line_len = line.len() - INDENT_LEN;

            let mut string = String::from(line);

            if line_len < count {
                // If the count for the current part of the quote is higher than the current line, just
                // continue on
                count -= line_len;
                output.push_str(string.as_str());
            } else if !red {
                // The current line is longer than the current part and we haven't started the red
                // section, so insert the code for red
                red = true;

                string.insert_str(count + INDENT_LEN, RED);
                count += time_len;

                // Check if the red section is not long enough to be split into a different line
                if count < line_len {
                    black = true;
                    string.insert_str(count + RED.len() + INDENT_LEN, BLACK);
                } else {
                    count -= line_len;
                }

                output.push_str(string.as_str());
            } else if !black {
                // End the red section
                black = true;

                string.insert_str(count + INDENT_LEN, BLACK);
                count += time_len;

                output.push_str(string.as_str());
            } else {
                // Just keep adding the final lines
                output.push_str(string.as_str());
            }

            // Account for spaces that would be between the words in different lines, but if a line
            // ends with a dash, there was no space between the words
            if count > 0 && !string.ends_with('-') {
                count -= 1;
            }
        }
        output.push_str(format!("\n{}\n", WHITE).as_str());

        let footer_wrapper = Wrapper::new(width)
            .initial_indent(FOOTER_INDENT)
            .subsequent_indent(FOOTER_INDENT);

        let footer = format!("{} - {}", self.author, self.title);
        let footer_lines = footer_wrapper.wrap(footer.as_str());

        output.push_str(footer_lines.join("\n").as_str());

        output.push_str(format!("\n{}", RESET).as_str());

        output
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
            end: " black black black ",
            author: "author",
            title: "title",
        };

        let wrapped = minute.wrapped(20);
        let expected = [
            String::from(BLACK),
            String::from("  \" black black"),
            format!("    black {}red red", RED),
            format!("    red red{} black", BLACK),
            String::from("    black black "),
            String::from(WHITE),
            String::from("        author -"),
            String::from("        title"),
            String::from(RESET),
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
            String::from(BLACK),
            format!("  \" foo {}bar{} baz", RED, BLACK),
            String::from(WHITE),
            String::from("        author - title"),
            String::from(RESET),
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
            String::from(BLACK),
            format!("  \" {}bar{} baz", RED, BLACK),
            String::from(WHITE),
            String::from("        author - title"),
            String::from(RESET),
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
            String::from(BLACK),
            format!("  \" foo {}bar", RED),
            String::from(WHITE),
            String::from("        author - title"),
            String::from(RESET),
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
            String::from(BLACK),
            String::from("  \" At 10.15 Arlena departed from her rondezvous,"),
            String::from("    a minute or two later Patrick Redfern came"),
            String::from("    down and registered surprise, annoyance, etc."),
            String::from("    Christine\'s task was easy enough. Keeping her"),
            String::from("    own watch concealed she asked Linda at twenty-"),
            String::from("    five past eleven what time it was."),
            String::from("    Linda looked at her watch and replied that it"),
            format!("    was a {}quarter to twelve{}.", RED, BLACK),
            String::from(WHITE),
            String::from("        Agatha Christie - Evil under the Sun"),
            String::from(RESET),
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
            String::from(BLACK),
            String::from("  \" And the first stop had"),
            format!("    been at {}1.16pm{} which was", RED, BLACK),
            String::from("    17 minutes later."),
            String::from(WHITE),
            String::from("        Mark Haddon - The"),
            String::from("        Curious Incident of"),
            String::from("        the Dog in the Night-"),
            String::from("        Time"),
            String::from(RESET),
        ].join("\n");

        assert_eq!(wrapped, expected);
    }
}
