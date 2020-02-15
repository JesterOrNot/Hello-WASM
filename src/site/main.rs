#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::NamedFile;
use std::io::Result;
use std::path::{Path, PathBuf};

#[get("/www/<file..>")]
fn www(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/pkg/<file..>")]
fn pkg(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("pkg/").join(file)).ok()
}

#[get("/")]
fn index() -> Result<NamedFile> {
    NamedFile::open("www/index.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, www, pkg])
        .launch();
}
