use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, PartialOrd)]
pub enum Shell {
    #[serde(alias = "fish")]
    Fish,
    #[serde(alias = "bash")]
    Bash,
    #[serde(alias = "zsh")]
    Zsh,
}

/// Arguments are the parameters that a workflow can take
/// They are used to generate a form for the user to fill out
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Argument {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
}
#[allow(dead_code)]
impl Argument {
    pub fn new(name: impl Into<String>) -> Self {
        Argument {
            description: None,
            name: name.into(),
            default_value: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn default_value(&self) -> &Option<String> {
        &self.default_value
    }
}

/// A workflow is a command that can be run
/// It has a name, a command, tags, and a source URL
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    pub source_url: Option<String>,
    pub author: Option<String>,
    pub author_url: Option<String>,
    #[serde(default)]
    pub shells: Vec<Shell>,
}


impl Display for Workflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nCommand: {}\nTags: {:?}\nDescription: {:?}\nArguments: {:?}\nSource URL: {:?}\nAuthor: {:?}\nAuthor URL: {:?}\nShells: {:?}",
               self.name,
               self.command,
               self.tags,
               self.description,
               self.arguments,
               self.source_url,
               self.author,
               self.author_url,
               self.shells
        )
    }
}

#[allow(dead_code)]
impl Workflow {
    pub fn render(&self, values: HashMap<&str, &str>) -> String {
        let mut command = self.command.clone();
        for (key, arg) in values {
            command = command.replace(&format!("{{{{{}}}}}", key), arg);
        }
        command
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn source_url(&self) -> &Option<String> {
        &self.source_url
    }

    pub fn author_name(&self) -> &Option<String> {
        &self.author
    }

    pub fn shells(&self) -> &Vec<Shell> {
        &self.shells
    }

    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Workflow {
            name: name.into(),
            command: command.into(),
            tags: vec![],
            description: None,
            arguments: vec![],
            source_url: None,
            author: None,
            author_url: None,
            shells: vec![],
        }
    }

    pub fn with_arguments(mut self, arguments: Vec<Argument>) -> Self {
        self.arguments = arguments;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
