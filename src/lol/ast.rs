use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Document {

    pub sections: Vec<Section>,

}

#[derive(Debug, Clone)]
pub struct Section {

    pub name: String,

    pub values: Vec<Value>,

}

#[derive(Debug, Clone)]
pub enum Value {

    Identifier(String),

    String(String),

    Number(f64),

    Boolean(bool),

    List(Vec<Value>),

    Object(HashMap<String, Value>),

    Call(Call),

}

#[derive(Debug, Clone)]
pub struct Call {

    pub name: String,

    pub arguments: Vec<Value>,

}

impl Document {

    pub fn new() -> Self {

        Self {
            sections: Vec::new(),
        }

    }

    pub fn section(
        &self,
        name: &str,
    ) -> Option<&Section> {

        self.sections
            .iter()
            .find(|s| s.name == name)

    }

}

impl Section {

    pub fn new(name: impl Into<String>) -> Self {

        Self {
            name: name.into(),
            values: Vec::new(),
        }

    }

}
