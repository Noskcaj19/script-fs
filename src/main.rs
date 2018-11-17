mod args;
mod core;
mod file_config;
mod fuse;

use log::info;

fn main() {
    env_logger::init();

    let template_root_path = args::parse_args();
    info!("Args parsed");
    let core = core::Core::new_from_path(template_root_path);
    println!("{:#?}", core);
    info!("Core constructed");
    info!("Mounting FUSE filesystem...");
    fuse::mount(core);
}
