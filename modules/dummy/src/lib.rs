extern crate hermod_module;

pub struct DummyModule;

impl ::hermod_module::Module for DummyModule {
}

#[no_mangle]
pub extern fn create_object() -> Box<DummyModule> {
    Box::new(DummyModule {})
}

#[no_mangle]
pub extern fn destroy_object(_: &DummyModule) {
}
