# BLUMER-MS-COMMENTS

This microservice is responsible for user comments on the different types of posts.

## Prerequisites
- Rust [install guide](https://www.rust-lang.org/es/tools/install)
- CapnProto [install guide](https://capnproto.org/install.html)
- GRPC [install guide]()
- Openssl

## Kafka Topics
### Consume
- `ms-comments-upload`

### Produce
- `ms-comments-create`
- `ms-comment-counts-create`
- `ms-comments-delete`
- `ms-comment-counts-delete`
- `ms-comments-reply-create`
- `ms-reply-counts-create`
- `ms-comment-replies-counter`
- `ms-comments-reply-delete`
- `ms-reply-counts-delete`

## Installation

Simply go over the following checklist:

1. Create a `deps` folder.
2. Git clone the following dependencies or libs inside `deps`.
- [blumer-lib-errors](https://github.com/blumerapi/blumer-lib-errors)
- [blumer-lib-auth-rs](https://github.com/blumerapi/blumer-lib-auth-rs)
- [blumer-lib-authorization-rs](https://github.com/blumerapi/blumer-lib-authorization-rs)


## Building

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```
## Tech Stack

- Rust[![Actix-Web](https://avatars.githubusercontent.com/u/5430905?s=48&v=4)](https://github.com/rust-lang/rust)
- Actix Web[![Actix-Web](https://avatars.githubusercontent.com/u/32776943?s=48&v=4)](https://github.com/actix/actix-web)
- Async Graphql[![AsyncGraphql](https://avatars.githubusercontent.com/u/12972006?s=48&v=4)](https://github.com/async-graphql/async-graphql)
- Redis[![Redis](https://avatars.githubusercontent.com/u/1529926?s=48&v=4)](https://github.com/redis/redis)
- ScyllaDB[![ScyllaDB](https://avatars.githubusercontent.com/u/14364730?s=48&v=4)](https://github.com/scylladb/scylladb)
- Kafka[![Kafka](https://kafka.apache.org/logos/kafka_logo--simple.png)](https://github.com/apache/kafka)
- Cap'n Proto[![Cap'n Proto](https://avatars.githubusercontent.com/u/29186932?s=48&v=4)](https://github.com/capnproto)


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file you can use it from .env.example

`APP_NAME` Microservice name and kafka group for receive messages 

`GRAPHQL_HOST` the host where will run the graphql server

`GRAPHQL_PORT` the port where will run the graphql server

`DATABASE_URI` The ScyllaDB server url, if you are using a cluster create a string separted by comma for each cluster node, example "node1,node2,node3"

`DATABASE_USERNAME` Scylladb username

`DATABASE_PASSWORD` Scylladb password

`REDIS_URL` Address to connect to redis server

`POST_AUTHORIZATION_SERVICE_URL` Authorization RPC server URL

`KAFKA_BROKER` The kafka server url, if you are using a cluster create a string separted by comma for each cluster node, example "node1,node2,node3"

`KAFKA_SECURITY_PROTOCOL` generally SASL_SSL

`KAFKA_SASL_MECHANISMS` generally SCRAM-SHA-512

`KAFKA_SASL_USERNAME` Kafka cluster username

`KAFKA_SASL_PASSWORD` Kafka cluster password

`AWS_CLOUDFRONT_URL` Address to connect to cloudfront server

`AWS_CLOUDFRONT_KEY_PAIR_ID` Cloudfront key pair id

`AWS_CLOUDFRONT_PRIVATE_KEY` Private key to connect to cloudfront server. IMPORTANT: The hash must be on a single line and explicitly include the /n and /r flags.

## Run Locally

Clone the project

```bash
  git clone https://github.com/blumerapi/blumer-ms-comments.git
```

Go to the project directory

```bash
  cd blumer-ms-comments
```

Complete installation step

Run the project

```bash
  cargo run
```


## Deployment

To deploy this project run

```bash
  docker compose -f docker-compose.yml up -d
```