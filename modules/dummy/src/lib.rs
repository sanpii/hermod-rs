pub mod hello;

hermod_module::export!(register);

extern "C" fn register(registrar: &mut dyn hermod_module::Registrar) {
    registrar.register("hello", Box::new(hello::Page));
}
