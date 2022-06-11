use libloading::Library;
use std::collections::HashMap;

pub struct Wrapper {
    lib: std::sync::Arc<Library>,
    pages: HashMap<String, Box<dyn hermod_module::Page>>,
}

impl Wrapper {
    pub fn process(&self, name: &str, request: hermod_module::Request) -> hermod_module::Response {
        self.pages.get(name).unwrap().process(request)
    }
}

impl hermod_module::Registrar for Wrapper {
    fn register(&mut self, name: &str, page: Box<dyn hermod_module::Page>) {
        self.pages.insert(name.to_string(), page);
    }
}

pub struct Loader;

impl Loader {
    pub fn load(path: &String) -> Result<Wrapper, String> {
        let lib = unsafe {
            match Library::new(path) {
                Ok(lib) => lib,
                Err(err) => return Err(format!("Unable to load {path:?}: {err}")),
            }
        };

        let mut wrapper = Wrapper {
            lib: std::sync::Arc::new(lib),
            pages: HashMap::new(),
        };

        Self::declaration(&mut wrapper);

        Ok(wrapper)
    }

    fn declaration(wrapper: &mut Wrapper) {
        let declaration = unsafe {
            wrapper.lib.get::<*mut hermod_module::Declaration>(b"declaration\0")
                .unwrap()
                .read()
        };

        if declaration.rustc_version != hermod_module::RUSTC_VERSION
            || declaration.core_version != hermod_module::CORE_VERSION
        {
            panic!("Version mismatch");
        }

        unsafe {
            (declaration.register)(wrapper);
        }
    }
}
