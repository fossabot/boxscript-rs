use std::fs;

pub fn read(args: Vec<String>) -> Result<String, String> {
    if args.len() != 1 {
        return Err(String::from(
            "\u{001b}[31mExactly 1 argument is required: `filename`\u{001b}[0m",
        ));
    }

    let filename: &String = &args[0];

    let content = fs::read_to_string(filename);

    if content.is_err() {
        return Err(format!(
            "\u{001b}[31mNo file exists at `{}`\u{001b}[0m",
            filename
        ));
    }

    Ok(content.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_errors_on_bad_args() {
        assert_eq!(
            read(["a", "b"].iter().map(|s| String::from(*s)).collect()),
            Err(String::from(
                "\u{001b}[31mExactly 1 argument is required: `filename`\u{001b}[0m",
            ))
        );

        assert_eq!(
            read([""].iter().map(|s| String::from(*s)).collect()),
            Err(String::from("\u{001b}[31mNo file exists at ``\u{001b}[0m",))
        );
    }

    #[test]
    fn it_reads_a_file() {
        assert_eq!(
            read(
                ["src/cli/mod.rs"]
                    .iter()
                    .map(|s| String::from(*s))
                    .collect()
            ),
            Ok(String::from("pub mod env;\n",))
        );
    }
}
