use anyhow::Result;

pub fn recipe_title(source: &str) -> Option<String> {
    let src_file = slop::parse(source);
    if let Ok(src_file) = src_file {
        if let Some(title) = &src_file.recipes[0].title {
            return Some(title.clone());
        }
    }
    None
}

pub fn recipe_svg(source: &str) -> Result<String> {
    let src_ast = slop::parse(source)?;
    let recipe = slop::semantic::convert_source_file(src_ast);
    Ok(String::from_utf8(slop::svg::to_svg(&recipe))?)
}
