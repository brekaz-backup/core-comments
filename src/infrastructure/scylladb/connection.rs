use std::env;

use scylla::{Session, SessionBuilder};

pub struct ScyllaConfig;

impl ScyllaConfig {
    pub async fn create_scylla_session() -> Session {
        let user = env::var("DATABASE_USERNAME").expect("Can't get DB URL");
        let password = env::var("DATABASE_PASSWORD").expect("Can't get DB URL");
        let nodes = env::var("DATABASE_URI").expect("Can't get NODES");
        let session = SessionBuilder::new()
            .known_nodes(&nodes.split(",").collect::<Vec<&str>>())
            .user(user, password)
            .build()
            .await
            .expect("Can connect To SycllaDB");

        session
    }
}
