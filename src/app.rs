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
use std::collections::HashMap;

type Modules = std::sync::Arc<std::sync::Mutex<HashMap<String, crate::module::Wrapper>>>;

pub struct Application {
    config: crate::Config,
    modules: Modules,
}

impl Application {
    pub fn new(config: crate::Config) -> Self {
        let modules = Self::load_modules(&config);

        Application {
            config,
            modules: std::sync::Arc::new(std::sync::Mutex::new(modules)),
        }
    }

    pub async fn execute(&self) -> Result<(), hyper::Error> {
        let port = self.config.global.port
            .unwrap_or(9_000);

        let addr = format!("127.0.0.1:{port}")
            .parse()
            .unwrap();

        let server = Server::bind(&addr);

        server.serve(hyper::service::make_service_fn(move |_| {
            let config = self.config.clone();
            let modules = self.modules.clone();

            async move {
                let core = Core::new(config, modules);
                Ok::<_, std::convert::Infallible>(core)
            }
        })).await
    }

    fn load_modules(config: &crate::Config) -> HashMap<String, crate::module::Wrapper> {
        let mut modules = HashMap::new();

        let plugins = match config.plugins {
            Some(ref plugins) => plugins,
            None => return modules,
        };

        for (name, plugin) in plugins.iter() {
            let filename = match plugin {
                crate::config::Plugin::Simple(filename) => filename.clone(),
                crate::config::Plugin::Detailed(detail) => detail.load.clone(),
            };

            let path = format!("{}/{filename}", config.global.plugins_directory);

            let module = match crate::module::Loader::load(&path) {
                Ok(module) => module,
                Err(err) => {
                    log::error!("{err}");
                    continue;
                },
            };

            modules.insert(name.clone(), module);
        }

        modules
    }
}

struct Core {
    config: crate::Config,
    modules: Modules,
}

impl Core {
    fn new(config: crate::Config, modules: Modules) -> Self {
        Self {
            config,
            modules,
        }
    }
}

impl Service<Request<hyper::Body>> for Core {
    type Response = Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<hyper::Body>) -> Self::Future {
        let mut response = Response::default();

        match *request.method() {
            Method::OPTIONS => {
                let headers = response.headers_mut();

                headers.append(hyper::header::ALLOW, "HEAD,GET,PUT,DELETE,OPTIONS".parse().unwrap());
                headers.append(hyper::header::ACCESS_CONTROL_ALLOW_HEADERS, "access-control-allow-origin,x-requested-with".parse().unwrap());

                if let Some(cors_method) = request.headers().get("HTTP_CORS_METHOD") {
                    headers.append("Access-Control-Allow-Method", cors_method.clone());
                }
            },
            Method::GET => {
                match self.config.route.get(request.uri().path().trim_start_matches('/')) {
                    Some(route) => {
                        let (module_name, page_name) = route.split_once(':').unwrap();
                        match self.modules.lock().unwrap().get(module_name) {
                            Some(module) => response = module.process(page_name, request),
                            None => *response.status_mut() = StatusCode::FOUND,
                        }
                    }
                    None => *response.status_mut() = StatusCode::NOT_FOUND,
                };
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }

        Box::pin(async { Ok(response) })
    }
}
