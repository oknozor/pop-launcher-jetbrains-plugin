pub mod trusted_project {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Application {
        #[serde(rename = "$value")]
        pub components: Vec<Component>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Component {
        pub option: ComponentOption,
        pub name: String
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct ComponentOption {
        pub map: Option<Map>,
        #[serde(skip)]
        pub list: Option<List>
    }

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct List {
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
