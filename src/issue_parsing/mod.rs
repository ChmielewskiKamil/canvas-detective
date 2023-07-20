use std::{fs, path::Path};

#[derive(Debug, PartialEq)]
pub struct Issue {
    pub issue_number: u16,
    pub watson: String,
    pub severity: String,
    pub title: String,
}

pub fn parse_directory(path_to_directory: &str) -> Result<Vec<Issue>, String> {
    let mut issues: Vec<Issue> = Vec::new();

    let mut paths: Vec<_> = fs::read_dir(path_to_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    // Path sorting is not guaranteed by the OS.
    // In order to have 001 before 002 every single time,
    // it is necessary to sort the paths ourselves.
    paths.sort_unstable_by_key(|path| {
        path.file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .and_then(|filename| filename.parse::<u16>().ok())
    });

    for path in paths {
        let path_str = path.to_str().unwrap();
        let issue = parse_markdown_file(path_str)
            .map_err(|err| format!("Error while parsing markdown file `{path_str}`: {err}"))?;
        issues.push(issue);
    }

    Ok(issues)
}

fn parse_markdown_file(path_to_markdown_file: &str) -> Result<Issue, String> {
    let file_content = fs::read_to_string(path_to_markdown_file)
        .map_err(|err| format!("Error while reading file content to string: {err}"))?;
    let file_lines: Vec<&str> = file_content.lines().collect();

    let issue_number = Path::new(path_to_markdown_file)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
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

    Ok(Issue {
        issue_number,
        watson,
        severity,
        title,
    })
}

#[cfg(test)]
mod test_parsing {
    use super::*;
    #[test]
    fn it_should_parse_single_issue() {
        let path_to_markdown_file: &str = "tests/test_data/001.md";

        let parsing_result: Issue = parse_markdown_file(path_to_markdown_file).unwrap();

        let expected_result = Issue {
            issue_number: 001,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_should_parse_directory() {
        let path_to_directory: &str = "tests/test_data/directory_of_issues";
        let parsing_result: Vec<Issue> = parse_directory(path_to_directory).unwrap();

        let expected_result = vec![
            Issue {
                issue_number: 001,
                watson: "John Doe".to_string(),
                severity: "medium".to_string(),
                title: "This is a medium severity bug".to_string(),
            },
            Issue {
                issue_number: 002,
                watson: "Jane Doe".to_string(),
                severity: "high".to_string(),
                title: "This is a high severity bug".to_string(),
            },
            Issue {
                issue_number: 003,
                watson: "John Doe".to_string(),
                severity: "medium".to_string(),
                title: "This is a medium severity bug".to_string(),
            },
        ];

        assert_eq!(parsing_result, expected_result);
    }
}
