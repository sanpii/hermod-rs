use hyper::{
    Method,
    StatusCode,
};
use hyper::{
    Request,
    Response,
    Server,
    service::Service,
};

pub struct Application;

impl Application {
    pub fn new() -> Self {
        Application
    }

    pub async fn execute(&self, config: crate::Config) -> Result<(), hyper::Error> {
        self.load_modules(&config);

        let port = config.global.port
            .unwrap_or(9_000);

        let addr = format!("127.0.0.1:{}", port)
            .parse()
            .unwrap();

        let server = Server::bind(&addr);

        server.serve(hyper::service::make_service_fn(|_| async {
            Ok::<_, std::convert::Infallible>(Core)
        })).await
    }

    fn load_modules(&self, config: &crate::Config) {
        let plugins = match config.plugins {
            Some(ref plugins) => plugins,
            None => return,
        };

        for (_, plugin) in plugins.iter() {
            let filename = match plugin {
                crate::config::Plugin::Simple(ref filename) => filename.clone(),
                crate::config::Plugin::Detailed(ref detail) => detail.load.clone(),
            };

            let path = format!("{}/{}", config.global.plugins_directory, filename);

            let module = match crate::module::Loader::load(&path) {
                Ok(module) => module,
                Err(err) => {
                    log::error!("{}", err);
                    continue;
                },
            };
        }
    }
}

struct Core;

impl Service<Request<hyper::Body>> for Core {
    type Response = Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<hyper::Body>) -> Self::Future {
        let mut response = Response::default();

        match request.method() {
            &Method::OPTIONS => {
                let headers = response.headers_mut();

                headers.append(hyper::header::ALLOW, "HEAD,GET,PUT,DELETE,OPTIONS".parse().unwrap());
                headers.append(hyper::header::ACCESS_CONTROL_ALLOW_HEADERS, "access-control-allow-origin,x-requested-with".parse().unwrap());

                if let Some(cors_method) = request.headers().get("HTTP_CORS_METHOD") {
                    headers.append("Access-Control-Allow-Method", cors_method.clone());
                }
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }

        Box::pin(async { Ok(response) })
    }
}
