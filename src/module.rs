use libloading::Library;
use libloading::os::unix::{
    Symbol,
};

pub struct Wrapper {
    _lib: Library,
    module: Option<Box<::hermod_module::Module>>,
    destroy_object_fn: Symbol<extern fn(&::hermod_module::Module)>,
}

impl Wrapper {
}

impl ::hermod_module::Module for Wrapper {
}

pub struct Loader;

impl Loader {
    pub fn load(path: &String) -> Result<Wrapper, String> {
        let lib = match Library::new(path) {
            Ok(lib) => lib,
            Err(err) => return Err(format!("Unable to load {:?}: {}", path, err)),
        };

        let create_object_fn = Self::create_object_fn(&lib);
        let destroy_object_fn = Self::destroy_object_fn(&lib);

        let module = create_object_fn();

        Ok(Wrapper {
            _lib: lib,
            module: Some(module),
            destroy_object_fn: destroy_object_fn,
        })
    }

    fn create_object_fn(lib: &Library) -> Symbol<extern fn() -> Box<::hermod_module::Module>> {
        unsafe {
            lib.get::<extern fn() -> Box<::hermod_module::Module>>(b"create_object\0")
                .unwrap()
                .into_raw()
        }
    }

    fn destroy_object_fn(lib: &Library) -> Symbol<extern fn(&::hermod_module::Module)> {
        unsafe {
            lib.get::<extern fn(&::hermod_module::Module)>(b"destroy_object\0")
                .unwrap()
                .into_raw()
        }
    }
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        let module = self.module.take().unwrap();

        (self.destroy_object_fn)(module.as_ref());

        // Workaround for https://github.com/rust-lang/rust/issues/28794
        ::std::mem::forget(module);
    }
}
