use hyper::{
    Method,
    StatusCode,
};
use hyper::server::{
    Http,
    Request,
    Response,
    Service,
};

pub struct Application;

impl Application {
    pub fn new() -> Self {
        Application
    }

    pub fn execute(&self, config: crate::Config) {
        self.load_modules(&config);

        let port = config.global.port
            .unwrap_or(9_000);

        let addr = format!("127.0.0.1:{}", port)
            .parse()
            .unwrap();

        let server = Http::new().bind(&addr, || Ok(Core))
            .unwrap();

        server.run()
            .unwrap();
    }

    fn load_modules(&self, config: &crate::Config) {
        let ref plugins = match config.plugins {
            Some(ref plugins) => plugins,
            None => return,
        };

        for (_, plugin) in plugins.iter() {
            let filename = match plugin {
                &crate::config::Plugin::Simple(ref filename) => filename.clone(),
                &crate::config::Plugin::Detailed(ref detail) => detail.load.clone(),
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

impl Service for Core {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut response = Response::new();

        match request.method() {
            &Method::Options => {
                let mut headers = response.headers_mut();

                headers.set_raw("Allow", "HEAD,GET,PUT,DELETE,OPTIONS");
                headers.set_raw("Access-Control-Allow-Headers", "access-control-allow-origin,x-requested-with");

                if let Some(cors_method) = request.headers().get_raw("HTTP_CORS_METHOD") {
                    headers.set_raw("Access-Control-Allow-Method", cors_method.clone());
                }
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        }

        futures::future::ok(response)
    }
}
