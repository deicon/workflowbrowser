use std::collections::HashMap;
use std::iter::Map;
use serde::{Deserialize, Serialize};

/// Arguments are the parameters that a workflow can take
/// They are used to generate a form for the user to fill out
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Argument {
    name:String,
    description:String,
    default_value: String,
}

/// A workflow is a command that can be run
/// It has a name, a command, tags, and a source URL
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Workflow {
    name: String,
    command: String,
    tags: Vec<String>,
    source_url: String,
    author: String,
    author_url: String,
    shells: Vec<String>,
    description: String,
    arguments: Vec<Argument>,
}

impl Workflow {
    pub fn render(&self, values: HashMap<&str, &str>) -> String {
        let mut command = self.command.clone();
        for (key, arg) in values {
            command = command.replace(&format!("{{{{{}}}}}", key), arg);
        }
        command
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_load_workflow_from_yaml() {
        use serde_yaml;
        use std::fs::File;
        use std::io::BufReader;
        use super::Workflow;
        let file = File::open("tests/fixtures/sample.yaml").unwrap();
        let reader = BufReader::new(file);
        let workflow: Workflow = serde_yaml::from_reader(reader).unwrap();
        assert_eq!(workflow.name, "Attach a header to an HTTP request with cURL");
        assert_eq!(workflow.command, "curl --header \"{{header}}\" {{url}}\ndemo\n");
        assert_eq!(workflow.tags, vec!["curl".to_string()]);
    }

    #[test]
    fn test_render_workflow() {
        use super::Workflow;
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