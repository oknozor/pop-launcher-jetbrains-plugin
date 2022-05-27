pub mod trusted_project {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Application {
        pub component: Component,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Component {
        pub option: Option,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Option {
        pub map: Map,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Map {
        #[serde(rename = "$value")]
        pub entries: Vec<Project>,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Project {
        pub key: String,
        pub value: String,
    }
}
