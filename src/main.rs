use std::{fs, path::Path};

#[derive(Debug, PartialEq)]
struct Issue {
    issue_number: u16,
    watson: String,
    severity: String,
    title: String,
}

fn main() {
    todo!();
}

fn parse_markdown_file(path_to_markdown_file: &str) -> Issue {
    let file_content = fs::read_to_string(path_to_markdown_file).expect("Unable to read file");
    let file_lines: Vec<&str> = file_content.lines().collect();

    let issue_number = Path::new(path_to_markdown_file)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let watson = file_lines[0].to_string();
    let severity = file_lines[2].to_string();
    let title = file_lines[4].trim_start_matches('#').trim().to_string();

    Issue {
        issue_number,
        watson,
        severity,
        title,
    }
}

#[cfg(test)]
mod test_parsing {
    use super::*;
    #[test]
    fn it_should_parse_single_issue() {
        let path_to_markdown_file: &str = "tests/test_data/001.md";

        let parsing_result: Issue = parse_markdown_file(&path_to_markdown_file);

        let expected_result = Issue {
            issue_number: 001,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };

        assert_eq!(parsing_result, expected_result);
    }
}
