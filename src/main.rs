fn main() {
    println!("Hello, world!");
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
