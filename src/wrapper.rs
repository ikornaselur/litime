use textwrap::Wrapper;

use minute::Minute;

static INITIAL_INDENT: &'static str = "  \" ";
static SUBSEQUENT_INDENT: &'static str = "    ";
const INDENT_LEN: usize = 4;

static BLACK: &'static str = "\u{1b}[90m";
static RED: &'static str = "\u{1b}[31m";
static WHITE: &'static str = "\u{1b}[97m";

pub fn wrap_minute(minute: Minute, width: usize) -> String {
    let start_len = minute.start.len();
    let time_len = minute.time.len();

    let full = format!("{}{}{}", minute.start, minute.time, minute.end);

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

        if line_len < count {
            // If the count for the current part of the quote is higher than the current line, just
            // continue on
            count -= line_len;
            output.push_str(&line);
        } else if !red {
            // The current line is longer than the current part and we haven't started the red
            // section, so insert the code for red
            red = true;

            let mut s = String::from(line);

            s.insert_str(count + INDENT_LEN, RED);
            count += time_len;

            // Check if the red section is not long enough to be split into a different line
            if count < line_len {
                black = true;
                s.insert_str(count + RED.len() + INDENT_LEN, BLACK);
            } else {
                count -= line_len;
            }

            output.push_str(s.as_str());
        } else if !black {
            // End the red section
            black = true;

            let mut s = String::from(line);

            s.insert_str(count + INDENT_LEN, BLACK);
            count += time_len;

            output.push_str(s.as_str());
        } else {
            // Just keep adding the final lines
            output.push_str(&line);
        }

        if count > 0 {
            count -= 1; // Account for spaces that would be between the words in different lines
        }
    }

    output.push_str(
        format!(
            "\n\n{}{}{}{} - {}\n",
            WHITE, SUBSEQUENT_INDENT, SUBSEQUENT_INDENT, minute.author, minute.title
        ).as_str(),
    );

    output
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

        let wrapped = wrap_minute(minute, 20);
        let expected = [
            String::from(BLACK),
            String::from("  \" black black"),
            format!("    black {}red red", RED),
            format!("    red red{} black", BLACK),
            String::from("    black black "),
            String::new(),
            format!("{}        author - title", WHITE),
            String::new(),
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

        let wrapped = wrap_minute(minute, 50);
        let expected = [
            String::from(BLACK),
            format!("  \" foo {}bar{} baz", RED, BLACK),
            String::new(),
            format!("{}        author - title", WHITE),
            String::new(),
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

        let wrapped = wrap_minute(minute, 50);
        let expected = [
            String::from(BLACK),
            format!("  \" {}bar{} baz", RED, BLACK),
            String::new(),
            format!("{}        author - title", WHITE),
            String::new(),
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

        let wrapped = wrap_minute(minute, 50);
        let expected = [
            String::from(BLACK),
            format!("  \" foo {}bar", RED),
            String::new(),
            format!("{}        author - title", WHITE),
            String::new(),
        ].join("\n");

        assert_eq!(wrapped, expected);
    }
}
