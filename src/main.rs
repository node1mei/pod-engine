mod framework;

use framework::common::base_application::BaseApplication;
use framework::traits::{application::IApplication, runtime::IRuntime};
fn main() {
    let app = BaseApplication::new();
    app.initialize();

    while !app.is_quit() {
        println!("run");
        app.tick();
    }

    app.finalize();

    println!("Hello, world!");
}
