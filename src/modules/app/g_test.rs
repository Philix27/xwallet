#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use actix_http::body::MessageBody;
    use actix_web::{
        dev::ServiceResponse,
        http,
        http::header::{ACCEPT, CONTENT_TYPE},
        test::{self, TestRequest},
        web::Data,
        App,
    };
    use futures::future;
    use juniper::{
        http::tests::{run_http_test_suite, HttpIntegration, TestResponse},
        tests::fixtures::starwars::schema::{Database, Query},
        EmptyMutation, EmptySubscription, RootNode,
    };

    use super::*;

    type Schema =
        juniper::RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

    async fn take_response_body_string(resp: ServiceResponse) -> String {
        let mut body = resp.into_body();
        String::from_utf8(
            future::poll_fn(|cx| Pin::new(&mut body).poll_next(cx))
                .await
                .unwrap()
                .unwrap()
                .to_vec(),
        )
        .unwrap()
    }

    async fn index(
        req: HttpRequest,
        payload: actix_web::web::Payload,
        schema: web::Data<Schema>,
    ) -> Result<HttpResponse, Error> {
        let context = Database::new();
        graphql_handler(&schema, &context, req, payload).await
    }

    #[actix_web::rt::test]
    async fn graphiql_response_does_not_panic() {
        let result = graphiql_handler("/abcd", None).await;
        assert!(result.is_ok())
    }

    #[actix_web::rt::test]
    async fn graphiql_endpoint_matches() {
        async fn graphql_handler() -> Result<HttpResponse, Error> {
            graphiql_handler("/abcd", None).await
        }
        let mut app =
            test::init_service(App::new().route("/", web::get().to(graphql_handler))).await;
        let req = TestRequest::get()
            .uri("/")
            .append_header((ACCEPT, "text/html"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::rt::test]
    async fn graphiql_endpoint_returns_graphiql_source() {
        async fn graphql_handler() -> Result<HttpResponse, Error> {
            graphiql_handler("/dogs-api/graphql", Some("/dogs-api/subscriptions")).await
        }
        let mut app =
            test::init_service(App::new().route("/", web::get().to(graphql_handler))).await;
        let req = TestRequest::get()
            .uri("/")
            .append_header((ACCEPT, "text/html"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap(),
            "text/html; charset=utf-8"
        );
        let body = take_response_body_string(resp).await;
        assert!(body.contains("var JUNIPER_URL = '/dogs-api/graphql';"));
        assert!(body.contains("var JUNIPER_SUBSCRIPTIONS_URL = '/dogs-api/subscriptions';"))
    }

    #[actix_web::rt::test]
    async fn playground_endpoint_matches() {
        async fn graphql_handler() -> Result<HttpResponse, Error> {
            playground_handler("/abcd", None).await
        }
        let mut app =
            test::init_service(App::new().route("/", web::get().to(graphql_handler))).await;
        let req = TestRequest::get()
            .uri("/")
            .append_header((ACCEPT, "text/html"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::rt::test]
    async fn playground_endpoint_returns_playground_source() {
        async fn graphql_handler() -> Result<HttpResponse, Error> {
            playground_handler("/dogs-api/graphql", Some("/dogs-api/subscriptions")).await
        }
        let mut app =
            test::init_service(App::new().route("/", web::get().to(graphql_handler))).await;
        let req = TestRequest::get()
            .uri("/")
            .append_header((ACCEPT, "text/html"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap(),
            "text/html; charset=utf-8"
        );
        let body = take_response_body_string(resp).await;
        assert!(body.contains(
            "endpoint: '/dogs-api/graphql', subscriptionEndpoint: '/dogs-api/subscriptions'",
        ));
    }

    #[actix_web::rt::test]
    async fn graphql_post_works_json_post() {
        let schema: Schema = RootNode::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        );

        let req = TestRequest::post()
            .append_header(("content-type", "application/json; charset=utf-8"))
            .set_payload(
                r#"{ "variables": null, "query": "{ hero(episode: NEW_HOPE) { name } }" }"#,
            )
            .uri("/")
            .to_request();

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(schema))
                .route("/", web::post().to(index)),
        )
        .await;

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json",
        );
        assert_eq!(
            take_response_body_string(resp).await,
            r#"{"data":{"hero":{"name":"R2-D2"}}}"#
        );
    }

    #[actix_web::rt::test]
    async fn graphql_get_works() {
        let schema: Schema = RootNode::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        );

        let req = TestRequest::get()
            .append_header(("content-type", "application/json"))
            .uri("/?query=%7B%20hero%28episode%3A%20NEW_HOPE%29%20%7B%20name%20%7D%20%7D&variables=null")
            .to_request();

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(schema))
                .route("/", web::get().to(index)),
        )
        .await;

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json",
        );
        assert_eq!(
            take_response_body_string(resp).await,
            r#"{"data":{"hero":{"name":"R2-D2"}}}"#
        );
    }

    #[actix_web::rt::test]
    async fn batch_request_works() {
        use juniper::{
            tests::fixtures::starwars::schema::{Database, Query},
            EmptyMutation, EmptySubscription, RootNode,
        };

        let schema: Schema = RootNode::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        );

        let req = TestRequest::post()
            .append_header(("content-type", "application/json"))
            .set_payload(
                r#"[
                     { "variables": null, "query": "{ hero(episode: NEW_HOPE) { name } }" },
                     { "variables": null, "query": "{ hero(episode: EMPIRE) { id name } }" }
                 ]"#,
            )
            .uri("/")
            .to_request();

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(schema))
                .route("/", web::post().to(index)),
        )
        .await;

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/json",
        );
        assert_eq!(
            take_response_body_string(resp).await,
            r#"[{"data":{"hero":{"name":"R2-D2"}}},{"data":{"hero":{"id":"1000","name":"Luke Skywalker"}}}]"#
        );
    }

    #[test]
    fn batch_request_deserialization_can_fail() {
        let json = r#"blah"#;
        let result: Result<GraphQLBatchRequest, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    pub struct TestActixWebIntegration;

    impl TestActixWebIntegration {
        fn make_request(&self, req: TestRequest) -> TestResponse {
            actix_web::rt::System::new().block_on(async move {
                let schema = Schema::new(
                    Query,
                    EmptyMutation::<Database>::new(),
                    EmptySubscription::<Database>::new(),
                );

                let mut app = test::init_service(
                    App::new()
                        .app_data(Data::new(schema))
                        .route("/", web::to(index)),
                )
                .await;

                let resp = test::call_service(&mut app, req.to_request()).await;
                make_test_response(resp).await
            })
        }
    }

    impl HttpIntegration for TestActixWebIntegration {
        fn get(&self, url: &str) -> TestResponse {
            self.make_request(TestRequest::get().uri(url))
        }

        fn post_json(&self, url: &str, body: &str) -> TestResponse {
            self.make_request(
                TestRequest::post()
                    .append_header(("content-type", "application/json"))
                    .set_payload(body.to_owned())
                    .uri(url),
            )
        }

        fn post_graphql(&self, url: &str, body: &str) -> TestResponse {
            self.make_request(
                TestRequest::post()
                    .append_header(("content-type", "application/graphql"))
                    .set_payload(body.to_owned())
                    .uri(url),
            )
        }
    }

    async fn make_test_response(resp: ServiceResponse) -> TestResponse {
        let status_code = resp.status().as_u16();
        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap()
            .into();
        let body = take_response_body_string(resp).await;
        TestResponse {
            status_code: status_code as i32,
            body: Some(body),
            content_type,
        }
    }

    #[test]
    fn test_actix_web_integration() {
        run_http_test_suite(&TestActixWebIntegration);
    }
}

#[cfg(feature = "subscriptions")]
#[cfg(test)]
mod subscription_tests {