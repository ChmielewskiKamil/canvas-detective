use crate::issue_parsing::parse_directory;
use crate::issue_parsing::Issue;
use serde::{Deserialize, Serialize};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CanvasFile {
    nodes: Vec<CanvasNode>,
    edges: Vec<()>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CanvasNode {
    #[serde(rename = "type")]
    node_type: String,
    text: String,
    id: String,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

pub fn generate_canvas_file_content(directory_path: &str) -> Result<String, String> {
    let issues = parse_directory(directory_path)
        .map_err(|err| format!("Error while parsing directory `{directory_path}`: {err}"))?;
    let canvas_nodes = generate_multiple_canvas_nodes(&issues);

    let canvas = CanvasFile {
        nodes: canvas_nodes,
        edges: vec![],
    };

    Ok(serde_json::to_string_pretty(&canvas).unwrap())
}

fn generate_multiple_canvas_nodes(issues: &[Issue]) -> Vec<CanvasNode> {
    issues.iter().map(generate_single_canvas_node).collect()
}

fn generate_single_canvas_node(issue: &Issue) -> CanvasNode {
    let node_label = generate_label(issue);
    let node_id = generate_id(&node_label);
    let node_x = calculate_node_x(issue.issue_number);
    let node_y = calculate_node_y(issue.issue_number);

    CanvasNode {
        node_type: "text".to_string(),
        text: node_label,
        id: node_id,
        x: node_x,
        y: node_y,
        width: 300,
        height: 150,
    }
}

fn generate_label(issue: &Issue) -> String {
    // 0 padded 3 digits, separated by a `-` dash, followed by the title
    // e.g. `001 - This is a medium severity bug`
    // or `012 - This is a high severity bug`
    format!("{:03} - {}", issue.issue_number, issue.title)
}

fn generate_id(label: &str) -> String {
    let hash = calculate_hash(&label);
    // Obsidian Canvas nodes use 16 digit unique identifiers
    let id = format!("{hash:016x}");
    id
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

const fn calculate_node_x(issue_number: u16) -> u16 {
    let column_number = (issue_number - 1) % 20;

    column_number * 360
}

const fn calculate_node_y(issue_number: u16) -> u16 {
    let row_number = (issue_number - 1) / 20;

    row_number * 300
}

////////////////////////////////////////////////////////////////////
//                             Tests                              //
////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test_serializing {
    use std::fs;
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
        let generated_id = generate_id(label);

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

        let generated_canvas_node = generate_single_canvas_node(&issue);

        let expected_result = CanvasNode {
            node_type: "text".to_string(),
            text: "001 - This is a medium severity bug".to_string(),
            id: "1426e27891c91000".to_string(),
            x: 0,
            y: 0,
            width: 300,
            height: 150,
        };

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

        let generated_canvas_node1 = generate_single_canvas_node(&issue1);
        let generated_canvas_node2 = generate_single_canvas_node(&issue2);
        let generated_canvas_node3 = generate_single_canvas_node(&issue3);

        let expected_result1 = CanvasNode {
            node_type: "text".to_string(),
            text: "001 - This is a medium severity bug".to_string(),
            id: "1426e27891c91000".to_string(),
            x: 0,
            y: 0,
            width: 300,
            height: 150,
        };

        let expected_result2 = CanvasNode {
            node_type: "text".to_string(),
            text: "002 - This is a medium severity bug".to_string(),
            id: "0b9b5099b5b2bc17".to_string(),
            x: 360,
            y: 0,
            width: 300,
            height: 150,
        };

        let expected_result3 = CanvasNode {
            node_type: "text".to_string(),
            text: "003 - This is a medium severity bug".to_string(),
            id: "83eb17f90d7e7a32".to_string(),
            x: 720,
            y: 0,
            width: 300,
            height: 150,
        };

        assert_eq!(generated_canvas_node1, expected_result1);
        assert_eq!(generated_canvas_node2, expected_result2);
        assert_eq!(generated_canvas_node3, expected_result3);
    }

    #[test]
    fn it_should_generate_multiple_canvas_nodes() {
        let issues = vec![
            Issue {
                issue_number: 001,
                watson: "John Doe".to_string(),
                severity: "medium".to_string(),
                title: "This is a medium severity bug".to_string(),
            },
            Issue {
                issue_number: 002,
                watson: "John Doe".to_string(),
                severity: "medium".to_string(),
                title: "This is a medium severity bug".to_string(),
            },
            Issue {
                issue_number: 003,
                watson: "John Doe".to_string(),
                severity: "medium".to_string(),
                title: "This is a medium severity bug".to_string(),
            },
        ];

        let generated_canvas_nodes = generate_multiple_canvas_nodes(&issues);
        let expected_result = vec![
            CanvasNode {
                node_type: "text".to_string(),
                text: "001 - This is a medium severity bug".to_string(),
                id: "1426e27891c91000".to_string(),
                x: 0,
                y: 0,
                width: 300,
                height: 150,
            },
            CanvasNode {
                node_type: "text".to_string(),
                text: "002 - This is a medium severity bug".to_string(),
                id: "0b9b5099b5b2bc17".to_string(),
                x: 360,
                y: 0,
                width: 300,
                height: 150,
            },
            CanvasNode {
                node_type: "text".to_string(),
                text: "003 - This is a medium severity bug".to_string(),
                id: "83eb17f90d7e7a32".to_string(),
                x: 720,
                y: 0,
                width: 300,
                height: 150,
            },
        ];

        assert_eq!(generated_canvas_nodes, expected_result);
    }

    #[test]
    fn it_should_generate_canvas_file_content_given_directory() {
        let path_to_directory = "tests/test_data/directory_of_issues";
        let generated_canvas_file_content =
            generate_canvas_file_content(path_to_directory).unwrap();

        let expected_result = fs::read_to_string("tests/test_data/test1.canvas").unwrap();

        // Normalize whitespace by deleting it
        let generated_normalized = generated_canvas_file_content.replace(char::is_whitespace, "");
        let expected_normalized = expected_result.replace(char::is_whitespace, "");

        assert_eq!(generated_normalized, expected_normalized);
    }
}
