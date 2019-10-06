mod cli;
pub use cli::cli;
use std::fs::DirBuilder;
pub fn mkdirs(base: &str) {
    let mut builder = DirBuilder::new();
    let builder = builder.recursive(true);
    vec![
        "public/css",
        "public/scripts",
        "public/img",
        "src/Partials",
        "src/Pages",
        "src/Utils",
        ".scripts",
    ]
    .iter()
    .for_each(|dir| builder.create(format!("{}/{}", base, dir)).unwrap());
}
