// #![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(warnings)]

use actix_web::{
    error::JsonPayloadError, http::Method, web, Error, FromRequest, HttpMessage, HttpRequest,
    HttpResponse,
};
use juniper::{
    http::{
        graphiql::graphiql_source, playground::playground_source, GraphQLBatchRequest,
        GraphQLRequest,
    },
    ScalarValue,
};
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
struct GetGraphQLRequest {
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    variables: Option<String>,
}

impl<S> From<GetGraphQLRequest> for GraphQLRequest<S>
where
    S: ScalarValue,
{
    fn from(get_req: GetGraphQLRequest) -> Self {
        let GetGraphQLRequest {
            query,
            operation_name,
            variables,
        } = get_req;
        let variables = variables.map(|s| serde_json::from_str(&s).unwrap());
        Self::new(query, operation_name, variables)
    }
}

/// Actix Web GraphQL Handler for GET and POST requests
pub async fn graphql_handler<Query, Mutation, Subscription, CtxT, S>(
    schema: &juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context: &CtxT,
    req: HttpRequest,
    payload: actix_web::web::Payload,
) -> Result<HttpResponse, Error>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Query::TypeInfo: Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Mutation::TypeInfo: Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
    Subscription::TypeInfo: Sync,
    CtxT: Sync,
    S: ScalarValue + Send + Sync,
{
    match *req.method() {
        Method::POST => post_graphql_handler(schema, context, req, payload).await,
        Method::GET => get_graphql_handler(schema, context, req).await,
        _ => Err(actix_web::error::UrlGenerationError::ResourceNotFound.into()),
    }
}
/// Actix GraphQL Handler for GET requests
pub async fn get_graphql_handler<Query, Mutation, Subscription, CtxT, S>(
    schema: &juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context: &CtxT,
    req: HttpRequest,
) -> Result<HttpResponse, Error>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Query::TypeInfo: Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Mutation::TypeInfo: Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
    Subscription::TypeInfo: Sync,
    CtxT: Sync,
    S: ScalarValue + Send + Sync,
{
    let get_req = web::Query::<GetGraphQLRequest>::from_query(req.query_string())?;
    let req = GraphQLRequest::from(get_req.into_inner());
    let gql_response = req.execute(schema, context).await;
    let body_response = serde_json::to_string(&gql_response)?;
    let mut response = match gql_response.is_ok() {
        true => HttpResponse::Ok(),
        false => HttpResponse::BadRequest(),
    };
    Ok(response
        .content_type("application/json")
        .body(body_response))
}

/// Actix GraphQL Handler for POST requests
pub async fn post_graphql_handler<Query, Mutation, Subscription, CtxT, S>(
    schema: &juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context: &CtxT,
    req: HttpRequest,
    payload: actix_web::web::Payload,
) -> Result<HttpResponse, Error>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Query::TypeInfo: Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT>,
    Mutation::TypeInfo: Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT>,
    Subscription::TypeInfo: Sync,
    CtxT: Sync,
    S: ScalarValue + Send + Sync,
{
    let req = match req.content_type() {
        "application/json" => {
            let body = String::from_request(&req, &mut payload.into_inner()).await?;
            serde_json::from_str::<GraphQLBatchRequest<S>>(&body)
                .map_err(JsonPayloadError::Deserialize)
        }
        "application/graphql" => {
            let body = String::from_request(&req, &mut payload.into_inner()).await?;
            Ok(GraphQLBatchRequest::Single(GraphQLRequest::new(
                body, None, None,
            )))
        }
        _ => Err(JsonPayloadError::ContentType),
    }?;
    let gql_batch_response = req.execute(schema, context).await;
    let gql_response = serde_json::to_string(&gql_batch_response)?;
    let mut response = match gql_batch_response.is_ok() {
        true => HttpResponse::Ok(),
        false => HttpResponse::BadRequest(),
    };
    Ok(response.content_type("application/json").body(gql_response))
}

/// Create a handler that replies with an HTML page containing GraphiQL. This does not handle routing, so you can mount it on any endpoint
///
/// For example:
///
/// ```
/// # use juniper_actix::graphiql_handler;
/// # use actix_web::{web, App};
///
/// let app = App::new()
///          .route("/", web::get().to(|| graphiql_handler("/graphql", Some("/graphql/subscriptions"))));
/// ```
pub async fn graphiql_handler(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: Option<&'static str>,
) -> Result<HttpResponse, Error> {
    let html = graphiql_source(graphql_endpoint_url, subscriptions_endpoint_url);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

/// Create a handler that replies with an HTML page containing GraphQL Playground. This does not handle routing, so you cant mount it on any endpoint.
pub async fn playground_handler(
    graphql_endpoint_url: &str,
    subscriptions_endpoint_url: Option<&'static str>,
) -> Result<HttpResponse, Error> {
    let html = playground_source(graphql_endpoint_url, subscriptions_endpoint_url);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
