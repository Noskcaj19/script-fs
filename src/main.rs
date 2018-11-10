mod args;
mod core;

fn main() {
    let template_root_path = args::parse_args();

    let core = core::Core::new_from_path(template_root_path);
    println!("{:#?}", core)
}
