use ::axum_server::Handle;

pub fn set_panic_hook(handle: Option<Handle>) {
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("---------------------PANIC!---------------------");
        
        if let Some(location) = panic_info.location() {
            eprintln!("> file: {}", location.file());
            eprintln!("> line: {}, column {}", location.line(), location.column());
        }
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("{message}");
        }

        if let Some(handle) = &handle {
            handle.shutdown();
        }

        eprintln!("------------------------------------------------");
    }));
}