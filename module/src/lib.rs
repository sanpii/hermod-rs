pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub struct Declaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn Registrar),
}

#[macro_export]
macro_rules! export {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static declaration: $crate::Declaration = $crate::Declaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}

pub trait Registrar {
    fn register(&mut self, name: &str, page: Box<dyn Page>);
}

pub type Request = hyper::Request<hyper::Body>;
pub type Response = hyper::Response<hyper::Body>;

pub trait Page: Send {
    fn process(&self, request: Request) -> Response;
}
