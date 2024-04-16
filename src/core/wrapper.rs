use rust_persian_tools::*;

pub fn add_commas(s: &str) -> Result<String, String> {
    Ok(commas::add_commas::add_commas(s))
}

pub fn remove_commas(s: &str) -> Result<String, String> {
    Ok(commas::remove_commas::remove_commas(s))
}
