use std::fs::DirBuilder;
pub fn mkdirs()
{
    let mut builder = DirBuilder::new();
    let builder = builder.recursive(true);
    vec![
        "public/css",
        "public/scripts",
        "public/img",
        "Src/Partials",
        "Src/Pages",
        "Src/Utils",
        ".scripts",
    ]
    .iter()
    .for_each(|dir| builder.create(dir).unwrap());
}
