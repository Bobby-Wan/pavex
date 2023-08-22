//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
#[allow(unused_imports)]
use std as alloc;
struct ServerState {
    router: pavex::routing::Router<u32>,
    application_state: ApplicationState,
}
pub struct ApplicationState {
    s0: app::HttpClient,
}
pub async fn build_application_state(v0: app::Config) -> crate::ApplicationState {
    let v1 = app::http_client(v0);
    crate::ApplicationState { s0: v1 }
}
pub async fn run(
    server_builder: pavex::hyper::server::Builder<
        pavex::hyper::server::conn::AddrIncoming,
    >,
    application_state: ApplicationState,
) -> Result<(), pavex::Error> {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router().map_err(pavex::Error::new)?,
        application_state,
    });
    let make_service = pavex::hyper::service::make_service_fn(move |_| {
        let server_state = server_state.clone();
        async move {
            Ok::<
                _,
                pavex::hyper::Error,
            >(
                pavex::hyper::service::service_fn(move |request| {
                    let server_state = server_state.clone();
                    async move {
                        let response = route_request(request, server_state).await;
                        let response = pavex::hyper::Response::from(response);
                        Ok::<_, pavex::hyper::Error>(response)
                    }
                }),
            )
        }
    });
    server_builder.serve(make_service).await.map_err(pavex::Error::new)
}
fn build_router() -> Result<pavex::routing::Router<u32>, pavex::routing::InsertError> {
    let mut router = pavex::routing::Router::new();
    router.insert("/home", 0u32)?;
    Ok(router)
}
async fn route_request(
    request: http::Request<pavex::hyper::body::Body>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    #[allow(unused)]
    let (request_head, request_body) = request.into_parts();
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.uri.path()) {
        Ok(m) => m,
        Err(_) => {
            return pavex::response::Response::not_found().box_body();
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::extract::route::RawRouteParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            match &request_head.method {
                &pavex::http::Method::GET => {
                    route_0::handler(
                            server_state.application_state.s0.clone(),
                            request_head,
                        )
                        .await
                }
                _ => {
                    let header_value = pavex::http::HeaderValue::from_static("GET");
                    pavex::response::Response::method_not_allowed()
                        .insert_header(pavex::http::header::ALLOW, header_value)
                        .box_body()
                }
            }
        }
        _ => pavex::response::Response::not_found().box_body(),
    }
}
pub mod route_0 {
    pub async fn handler(
        v0: app::HttpClient,
        v1: pavex::request::RequestHead,
    ) -> pavex::response::Response {
        let v2 = app::extract_path(v1).await;
        let v3 = match v2 {
            Ok(ok) => ok,
            Err(v3) => {
                return {
                    let v5 = {
                        let v4 = app::logger();
                        app::handle_extract_path_error(&v3, v4)
                    };
                    <pavex::response::Response as pavex::response::IntoResponse>::into_response(
                        v5,
                    )
                };
            }
        };
        let v5 = {
            let v4 = app::logger();
            app::stream_file(v3, v4, v0)
        };
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v5)
    }
}