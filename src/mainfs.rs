use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use libc::ENOENT;
use signal::trap::Trap;
use std::env;
use std::ffi::OsStr;
use time::Timespec;

fn wait(handle: fuse::BackgroundSession) {
    use nix::sys::signal;
    let trap = Trap::trap(&[signal::SIGTERM, signal::SIGINT, signal::SIGCHLD]);
    for _ in trap {
        break;
    }
    drop(handle)
}

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=script-fs"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let handle = unsafe {
        /*fuse::spawn_mount(ScriptFS, &mountpoint, &options).unwrap()*/
    };

    wait(handle);
}
