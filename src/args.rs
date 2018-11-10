use std::path::PathBuf;

pub fn parse_args() -> PathBuf {
    if let Some(template_dir) = std::env::args().skip(1).next() {
        let dir = PathBuf::from(template_dir);

        if dir.is_dir() {
            if dir.join("sfs.toml").exists() {
                return dir;
            } else {
                eprintln!("Invalid template: No sfs.toml file found in the template root")
            }
        } else {
            eprintln!("{:?} is not a directory", dir);
        }
    } else {
        eprintln!("usage: sfs template_dir");
        eprintln!("No template dir provided");
    }
    std::process::exit(1);
}
