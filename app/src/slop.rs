use anyhow::Result;

pub fn recipe_title(source: &str) -> Option<String> {
    let (src_file, _errors) = slop::parse(source);
    if let Some(title) = &src_file.recipes[0].title {
        return Some(title.clone());
    }
    None
}

pub fn recipe_svgs(source: &str) -> Result<Vec<String>> {
    let (svgs, errors) = slop::to_svgs(source);
    if errors.0.is_empty() {
        return Err(errors.into());
    }
    Ok(svgs)
}
