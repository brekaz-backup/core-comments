use scylla::Session;

pub struct ScyllaInit;

impl ScyllaInit {
    pub async fn create_keyspaces(session: &Session) {
        let keyspaces = vec![include_str!("scripts/ks/create_comments_keyspace.cql")];

        for ks in keyspaces {
            session.query(ks, &[]).await.expect("Can't create keyspace");
        }
    }

    pub async fn create_tables(session: &Session) {
        let tables = vec![
            include_str!("scripts/tables/create_table_comments.cql"),
            include_str!("scripts/tables/create_table_inactive_comments.cql"),
            include_str!("scripts/tables/create_table_comments_reply.cql"),
            include_str!("scripts/tables/create_table_inactive_comments_reply.cql"),
        ];

        for t in tables {
            session
                .query(t, &[])
                .await
                .expect(&format!("Can't create tables - {}", t));
        }
    }

    pub async fn create_materialized_views(session: &Session) {
        let tables: Vec<&str> = vec![];

        for t in tables {
            session
                .query(t, &[])
                .await
                .expect(&format!("Can't create Materialized View - {}", t));
        }
    }
}
