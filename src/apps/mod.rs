use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub mod hello;

pub trait App {
    fn run(&self);
    fn name(&self) -> String;
}

//#[non_exhaustive]
//pub struct Apps;

//impl Apps {
//    pub const HELLO: Box<App> = Box::new(crate::apps::hello::HelloApp);
//}

pub async fn run_app(app_name: &str) {
    let apps: [Box<dyn App>; 1] = [Box::new(crate::apps::hello::HelloApp)];
    for app in apps {
        if app.name() == app_name {
            app.run();
        }
    }
}