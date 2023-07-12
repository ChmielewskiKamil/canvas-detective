use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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
fn generate_canvas_node(issue: &Issue) -> String {
    let node_label = generate_label(&issue);
    let node_id = generate_id(&node_label);
    let node_x = calculate_node_x(issue.issue_number);
    let node_y = calculate_node_y(issue.issue_number);
    let canvas_node = format!(
        r#"{{"type":"text","text":"{}","id":"{}","x":{},"y":{},"width":300,"height":150}}"#,
        node_label, node_id, node_x, node_y
    );
    canvas_node
}

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

fn calculate_node_x(issue_number: u16) -> u16 {
    let column_number = (issue_number - 1) % 20;
    let x_coordinate = column_number * 360;
    x_coordinate
}

fn calculate_node_y(issue_number: u16) -> u16 {
    let row_number = (issue_number - 1) / 20;
    let y_coordinate = row_number * 300;
    y_coordinate
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

        let expected_id = r#"1426e27891c91000"#;

        assert_eq!(generated_id, expected_id);
    }

    #[test]
    fn it_should_generate_single_canvas_node() {
        let issue = Issue {
            issue_number: 001,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };

        let generated_canvas_node = generate_canvas_node(&issue);

        let expected_result = r#"{"type":"text","text":"001 - This is a medium severity bug","id":"1426e27891c91000","x":0,"y":0,"width":300,"height":150}"#;

        assert_eq!(generated_canvas_node, expected_result);
    }

    #[test]
    fn it_should_space_out_multiple_nodes() {
        let issue1 = Issue {
            issue_number: 001,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };
        let issue2 = Issue {
            issue_number: 002,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };
        let issue3 = Issue {
            issue_number: 003,
            watson: "John Doe".to_string(),
            severity: "medium".to_string(),
            title: "This is a medium severity bug".to_string(),
        };

        let generated_canvas_node1 = generate_canvas_node(&issue1);
        let generated_canvas_node2 = generate_canvas_node(&issue2);
        let generated_canvas_node3 = generate_canvas_node(&issue3);

        let expected_result1 = r#"{"type":"text","text":"001 - This is a medium severity bug","id":"1426e27891c91000","x":0,"y":0,"width":300,"height":150}"#;
        let expected_result2 = r#"{"type":"text","text":"002 - This is a medium severity bug","id":"0b9b5099b5b2bc17","x":360,"y":0,"width":300,"height":150}"#;
        let expected_result3 = r#"{"type":"text","text":"003 - This is a medium severity bug","id":"83eb17f90d7e7a32","x":720,"y":0,"width":300,"height":150}"#;

        assert_eq!(generated_canvas_node1, expected_result1);
        assert_eq!(generated_canvas_node2, expected_result2);
        assert_eq!(generated_canvas_node3, expected_result3);
    }
}
