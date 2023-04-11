use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::http::GraphiQLSource;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use blumer_lib_auth_rs::User;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;

use super::state::AppState;
use super::{AppSchema, Mutation, Query};

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(
                web::get()
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .route(web::get().to(index_graphiql)),
    );
}

async fn index(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut query = req.into_inner();
    if let Ok(maybe_user) = User::get_user_from_headers(http_req) {
        if let Some(user) = maybe_user {
            query = query.data(user);
        }
    }

    schema.execute(query).await.into()
}

async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            //playground IDE
            GraphiQLSource::build().endpoint("/").finish(),
        ))
}

pub fn create_schema_with_context(
    app_state: AppState,
    post_authorization: PostAuthorization,
) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(app_state.clone())
        .data(post_authorization.clone())
        .finish()
}
