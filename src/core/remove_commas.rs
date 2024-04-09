use super::Tool;
pub struct RemoveCommas;
impl Tool for RemoveCommas {
    fn route(&self) -> &'static str {
        "/remove_commas"
    }
    fn getName(&self) -> &'static str {
        "پاک کردن کاما ها از یک رشته"
    }
    fn getDescription(&self) -> &'static str {
        ""
    }
    fn getDocPath(&self) -> &'static str {
        "commas/remove_commas"
    }
    fn getTags(&self) -> Vec<&'static str> {
        vec!["کاما", "عدد", "خوانایی"]
    }
    fn exe(&self, input: String) -> Result<String, String> {
        Ok(rust_persian_tools::commas::remove_commas(input))
    }
}