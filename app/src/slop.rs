use anyhow::Result;

pub fn recipe_title(source: &str) -> Option<String> {
    let (src_file, _errors) = slop::parse(source);
    if let Some(title) = &src_file.recipes[0].title {
        return Some(title.clone());
    }
    None
}

pub fn recipe_svg(source: &str) -> Result<String> {
    let (svg, errors) = slop::to_svg(source);
    if errors.0.is_empty() {
        return Err(errors.into());
    }
    Ok(String::from_utf8(svg)?)
}
