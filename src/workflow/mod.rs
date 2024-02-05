/// highly inspired and mostly copied from
/// https://github.com/warpdotdev/workflows/blob/main/workflow-types/src/lib.rs

mod file_format;
mod repository;

#[cfg(test)]
mod tests {
    use crate::workflow::file_format::Workflow;
    use serde_yaml;
    use std::fs::File;
    use std::io::BufReader;
    #[test]
    fn test_load_workflow_from_yaml() {

        let file = File::open("tests/fixtures/sample.yaml").unwrap();
        let reader = BufReader::new(file);
        let workflow: Workflow = serde_yaml::from_reader(reader).unwrap();
        assert_eq!(workflow.name, "Attach a header to an HTTP request with cURL");
        assert_eq!(workflow.command, "curl --header \"{{header}}\" {{url}}\ndemo\n");
        assert_eq!(workflow.tags, vec!["curl".to_string()]);
    }

    #[test]
    fn test_render_workflow() {
        let workflow = Workflow {
            command: "curl --header {{header}} \"{{url}}\"".to_string(),
            ..Workflow::default()
        };
        let mut values: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        values.insert("header", "Authorization: Bearer 123");
        values.insert("url", "https://www.google.de");
        assert_eq!(workflow.render(values), "curl --header Authorization: Bearer 123 \"https://www.google.de\"");
    }
}