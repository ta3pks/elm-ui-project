use clap::{
    App,
    Arg,
};
use std::{
    fs::{
        DirBuilder,
        File,
    },
    io::{
        self,
        ErrorKind,
    },
};
pub fn cli() {
    let m = App::new("uiproject")
        .author("Nikos Efthias<nikos@mugsoft.io>")
        .version(clap::crate_version!())
        .arg(Arg::with_name("project").required(true))
        .get_matches();

    let folder_name = m.value_of("project").unwrap();
    File::open(folder_name)
        .and_then(folder_exists)
        .or_else(|_| create_folder(folder_name))
        .and_then(|_| init_git_repo(folder_name))
        .and_then(|_| {
            super::mkdirs(folder_name);
            Ok(())
        })
        .expect("cannot create project");
}

fn folder_exists(_: File) -> std::io::Result<()> {
    eprintln!("project exists with given name");
    Err(std::io::Error::from(ErrorKind::AlreadyExists))
}
fn create_folder(name: &str) -> std::io::Result<()> {
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    builder.create(name)
}
fn init_git_repo(name: &str) -> io::Result<()> {
    git2::Repository::init(name)
        .and_then(|_| Ok(()))
        .or_else(|_| Err(io::Error::from(ErrorKind::Other)))
}
