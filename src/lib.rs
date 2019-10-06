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
    create_files(base);
}
pub fn create_files(base: &str) {
    vec![
        (
            "public/index.html",
            include_str!("../static_includes/index.html"),
        ),
        ("src/Main.elm", include_str!("../static_includes/Main.elm")),
        ("Makefile", include_str!("../static_includes/Makefile")),
        (
            ".scripts/filehash.sh",
            include_str!("../static_includes/filehash.sh"),
        ),
        (
            "elm.json",
            &include_str!("../static_includes/elm.json").replace("{{version}}", &elm_version()),
        ),
    ]
    .iter()
    .for_each(|(f, c)| {
        std::fs::write(format!("{}/{}", base, f), c).expect("cannot write needed files")
    });
}
fn elm_version() -> String {
    String::from_utf8_lossy(
        std::process::Command::new("elm")
            .arg("--version")
            .output()
            .expect("elm is not installed or not in the path")
            .stdout
            .as_slice(),
    )
    .trim()
    .into()
}
