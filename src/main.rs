mod adapters;
mod comments;
mod infrastructure;
mod media_comments;
mod ports;
mod reply_comments;
mod utils;

use crate::adapters::kafka::{KafkaConsumer, KafkaProducer};
use crate::comments::domain::{CommentRedisRepository, CommentRepository};
use crate::infrastructure::graphql::config::{configure_service, create_schema_with_context};
use crate::infrastructure::graphql::state::AppState;
use crate::infrastructure::redis::config::RedisConfig;
use crate::infrastructure::scylladb::connection::ScyllaConfig;
use crate::infrastructure::scylladb::initialization::ScyllaInit;
use crate::media_comments::infrastructure::kafka::{
    kafka_consumer::KafkaConsumer as MediaCommentsConsumer, KAFKA_TOPIC_COMMENT_UPLOAD,
};
use crate::reply_comments::domain::{ReplyCommentRedisRepository, ReplyCommentRepository};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use dotenv::dotenv;
use redis::cluster::ClusterClient;
use scylla::Session;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //init libs
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //connect sources
    //scylla
    let scylla_session: Session = ScyllaConfig::create_scylla_session().await;
    ScyllaInit::create_keyspaces(&scylla_session).await;
    ScyllaInit::create_tables(&scylla_session).await;
    ScyllaInit::create_materialized_views(&scylla_session).await;
    let scylla_session: Arc<Session> = Arc::new(ScyllaConfig::create_scylla_session().await);
    //redis
    let redis_client: ClusterClient = RedisConfig::connection().await;
    // initialize authorization service client
    let post_authorization: PostAuthorization = PostAuthorization::new(
        std::env::var("POST_AUTHORIZATION_SERVICE_URL")
            .expect("Can't get POST_AUTHORIZATION_SERVICE_URL env var"),
    )
    .await
    .expect("Can't connect to post authorization RPC server");

    let state: AppState = AppState {
        comment_repository: CommentRepository::new(scylla_session.clone()),
        comment_redis_repository: CommentRedisRepository::new(redis_client.clone()),
        reply_comment_repository: ReplyCommentRepository::new(scylla_session.clone()),
        reply_comment_redis_repository: ReplyCommentRedisRepository::new(redis_client.clone()),
        kafka_producer: KafkaProducer::new(),
    };
    let schema = web::Data::new(create_schema_with_context(state, post_authorization));
    let host: String = std::env::var("GRAPHQL_HOST").expect("Can't get GRAPHQL_HOST env var");
    let port: u16 = std::env::var("GRAPHQL_PORT")
        .expect("Can't get GRAPHQL_PORT env var")
        .parse()
        .expect("GRAPHQL_PORT isn't a number");
    println!("GraphiQL IDE: http://{}:{}", host, port);

    let comments_consumer = KafkaConsumer::new(
        vec![KAFKA_TOPIC_COMMENT_UPLOAD],
        MediaCommentsConsumer::new(
            CommentRepository::new(scylla_session.clone()),
            ReplyCommentRepository::new(scylla_session.clone()),
        ),
    );
    tokio::spawn(async move { comments_consumer.consume().await });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(configure_service)
            .app_data(schema.clone())
    })
    .bind((host, port))?
    .run()
    .await
}
