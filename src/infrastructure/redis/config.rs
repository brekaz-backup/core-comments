use redis::cluster::ClusterClient;

pub struct RedisConfig;

impl RedisConfig {
    pub async fn connection() -> ClusterClient {
        let nodes = std::env::var("REDIS_URL").expect("Cant get Redis URL");
        ClusterClient::new(nodes.split(',').collect::<Vec<&str>>()).expect("Cant open redis client")
    }
}