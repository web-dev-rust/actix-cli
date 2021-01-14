use clap::arg_enum;
use convert_case::{Case, Casing};

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum PossibleContexts {
        InMemory
    }
}

pub fn possible_contexts(_: PossibleContexts, model_name: String, name: String) -> String {
    in_memory()
        .replace("@", &model_name.to_case(Case::Pascal))
        .replace("{name}", &name.to_case(Case::Snake))
}

const fn in_memory() -> &'static str {
"   use {name}::model::{name}::@;
    let context: std::collections::HashMap<String, @> = std::collections::HashMap::new();"
}