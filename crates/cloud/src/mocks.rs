use uuid::Uuid;
use cloud_openapi::models::{Database as OpenAPIDatabase, Link};

#[derive(Clone, PartialEq)]
pub struct DatabaseLink {
    pub app_label: Link,
    pub database: String,
}

impl DatabaseLink {
    pub fn new(app_label: Link, database: String) -> Self {
        Self {
            app_label,
            database,
        }
    }

    pub fn has_label(&self, label: &str) -> bool {
        self.app_label.label == label
    }
}

pub struct Database {
    pub inner: OpenAPIDatabase,
}

impl Database {
    pub fn has_label(&self, label: &str) -> bool {
        self.inner.links.iter().any(|l| l.label == label)
    }

    pub fn has_link(&self, link: &Link) -> bool {
        self.inner.links.iter().any(|l| l == link)
    }
}

impl Database {
    pub fn new(name: String, links: Vec<Link>) -> Database {
        Database {inner: OpenAPIDatabase { name, links }}
    }
}

// TODO: remove all of this mocking code
fn create_link(label: String) -> Link {
    let app_id = Uuid::new_v4();
    let app_name = format!("myapp-{}", app_id.as_simple());
    Link {
        label,
        app_id,
        app_name,
    }
}

pub fn mock_databases_list() -> Vec<Database> {
    let db1_links = vec![
        create_link("foo".to_string()),
        create_link("yee".to_string()),
    ];
    let db2_links = vec![create_link("bar".to_string())];
    vec![
        Database::new("db1".to_string(), db1_links),
        Database::new("db2".to_string(), db2_links),
        Database::new("db3".to_string(), vec![]),
    ]
}

pub fn mock_links_list() -> Vec<DatabaseLink> {
    vec![
        DatabaseLink::new(create_link("foo".to_string()), "db1".to_string()),
        DatabaseLink::new(create_link("yee".to_string()), "db2".to_string()),
    ]
}
