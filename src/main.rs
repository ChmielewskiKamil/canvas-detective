use std::{fs, path::Path};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

////////////////////////////////////////////////////////////////////
//                            Parsing                             //
////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn parse_markdown_file(path_to_markdown_file: &str) -> Issue {
    let file_content = fs::read_to_string(path_to_markdown_file).expect("Unable to read file");
    let file_lines: Vec<&str> = file_content.lines().collect();

    let issue_number = Path::new(path_to_markdown_file)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap()
        .parse::<u16>()
        .unwrap();
    // Sherlock issues start with the watson's nickname
    let watson = file_lines[0].to_string();
    // followed by newline and severity, either high or medium
    let severity = file_lines[2].to_string();
    // Markdown H1 issue title is on the 5th line.
    // It will be used as a text shown on the canvas note.
    let title = file_lines[4].trim_start_matches('#').trim().to_string();

    Issue {
        issue_number,
        watson,
        severity,
        title,
    }
}

////////////////////////////////////////////////////////////////////
//                      Canvas Node Generation                    //
////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn generate_label(issue: &Issue) -> String {
    // 0 padded 3 digits, separated by a `-` dash, followed by the title
    // e.g. `001 - This is a medium severity bug`
    // or `012 - This is a high severity bug`
    format!("{:03} - {}", issue.issue_number, issue.title)
}

#[allow(dead_code)]
fn generate_id(label: &str) -> String {
    let hash = calculate_hash(&label);
    // Obsidian Canvas nodes use 16 digit unique identifiers
    let id = format!("{:016x}", hash);
    id
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

////////////////////////////////////////////////////////////////////
//                             Tests                              //
////////////////////////////////////////////////////////////////////

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

#[cfg(test)]
mod test_serializing {
    use super::*;
    #[test]
    fn it_should_generate_issue_label() {
        let input_issue = Issue {
            issue_number: 001,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };

        let generated_label = generate_label(&input_issue);

        let expected_label = r#"001 - This is a medium severity bug"#;

        assert_eq!(generated_label, expected_label);
    }

    #[test]
    fn it_should_generate_unique_id() {
        let label = r#"001 - This is a medium severity bug"#;
        let generated_id = generate_id(&label);

        let expected_id = r#"asfdfsaffas"#;

        assert_eq!(generated_id, expected_id);
    }
}
