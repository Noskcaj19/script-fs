use crate::core::Core;
use crate::core::CoreFile;
use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use libc::ENOENT;
use signal::trap::Trap;
use std::ffi::OsStr;
use time::Timespec;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 }; // 1 second

const CREATE_TIME: Timespec = Timespec {
    sec: 1_381_237_736,
    nsec: 0,
}; // 2013-10-08 08:56

// TODO: File size
const fn dir_attr(inode: u64) -> FileAttr {
    FileAttr {
        ino: inode,
        size: 0,
        blocks: 0,
        atime: CREATE_TIME,
        mtime: CREATE_TIME,
        ctime: CREATE_TIME,
        crtime: CREATE_TIME,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    }
}

// TODO: File size
const fn file_attr(inode: u64) -> FileAttr {
    FileAttr {
        ino: inode,
        size: 13,
        blocks: 1,
        atime: CREATE_TIME,
        mtime: CREATE_TIME,
        ctime: CREATE_TIME,
        crtime: CREATE_TIME,
        kind: FileType::RegularFile,
        perm: 0o777,
        nlink: 1,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
    }
}

const HELLO_TXT_CONTENT: &str = "Hello World!\n";

struct ScriptFS {
    core: Core,
}

impl ScriptFS {
    fn new(core: Core) -> ScriptFS {
        ScriptFS { core }
    }
}

impl Filesystem for ScriptFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("L:{:?}", name);
        let dir = match self.core.entries.get_index(parent as usize - 1) {
            Some(e) => match e {
                (path, CoreFile::Dir(files)) => (path, files),
                _ => {
                    log::error!("1");
                    reply.error(ENOENT);
                    return;
                }
            },
            None => {
                log::error!("2");
                reply.error(ENOENT);
                return;
            }
        };

        for &i in dir.1 {
            if let Some((k, v)) = self.core.entries.get_index(i) {
                if k.file_name().unwrap() == name {
                    match v {
                        CoreFile::Dir(_) => {
                            reply.entry(&TTL, &dir_attr(i as u64 + 1), 0);
                        }
                        CoreFile::File(_) => {
                            reply.entry(&TTL, &file_attr(i as u64 + 1), 0);
                        }
                    }

                    return;
                }
            }
        }

        reply.error(ENOENT);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("G:{:?}", _req);

        if let Some((_key, val)) = self.core.entries.get_index(ino as usize - 1) {
            println!("{:#?}", val);
            match val {
                CoreFile::File(_) => reply.attr(&TTL, &file_attr(ino)),
                CoreFile::Dir(_) => reply.attr(&TTL, &dir_attr(ino)),
            }
        } else {
            reply.error(ENOENT)
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {
        log::debug!("R");
        // if ino == 2 {
        reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        // } else {
        // reply.error(ENOENT);
        // }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        println!("D");
        let dir = match self.core.entries.get_index(ino as usize - 1) {
            Some(e) => match e {
                (path, CoreFile::Dir(files)) => (path, files),
                _ => {
                    reply.error(ENOENT);
                    return;
                }
            },
            None => {
                reply.error(ENOENT);
                return;
            }
        };
        let mut items = Vec::new();
        let parent_inode = if ino == 1 {
            ino
        } else {
            self.core
                .entries
                .get_full(dir.0.parent().unwrap())
                .unwrap()
                .0 as u64
        };
        items.push((ino, FileType::Directory, std::ffi::OsStr::new(".")));
        items.push((
            parent_inode as u64,
            FileType::Directory,
            std::ffi::OsStr::new(".."),
        ));

        for &file in dir.1 {
            if let Some(entry) = self.core.entries.get_index(file) {
                let entry_type = match entry.1 {
                    CoreFile::File(_) => FileType::RegularFile,
                    CoreFile::Dir(_) => FileType::Directory,
                };
                items.push((file as u64 + 2, entry_type, entry.0.file_name().unwrap()))
            }
        }

        // Offset of 0 means no offset.
        // Non-zero offset means the passed offset has already been seen, and we should start after
        // it.
        let to_skip = if offset == 0 { offset } else { offset + 1 } as usize;
        for (i, (ino, k, n)) in items.into_iter().enumerate().skip(to_skip) {
            reply.add(ino as u64, i as i64, k, n);
        }
        reply.ok();
    }
}

fn wait(handle: fuse::BackgroundSession) {
    use nix::sys::signal;
    let trap = Trap::trap(&[signal::SIGTERM, signal::SIGINT, signal::SIGCHLD]);
    for _ in trap {
        break;
    }
    drop(handle)
}

pub fn mount(core: Core) {
    let mount_point = core.root_config.mount.clone();
    let options = ["-o", "ro", "-o", "fsname=script-fs"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let handle = unsafe { fuse::spawn_mount(ScriptFS::new(core), &mount_point, &options).unwrap() };

    wait(handle);
}
