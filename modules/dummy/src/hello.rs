pub struct Page;

impl hermod_module::Page for Page {
    fn process(&self, _: hermod_module::Request) -> hermod_module::Response {
        hermod_module::Response::new("Hello !".into())
    }
}
