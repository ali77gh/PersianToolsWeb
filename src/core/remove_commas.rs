use super::Tool;
pub struct RemoveCommas;
impl Tool for RemoveCommas {
    fn get_route(&self) -> &'static str {
        "/remove_commas"
    }
    fn get_name(&self) -> &'static str {
        "پاک کردن کاما ها از یک رشته"
    }
    fn get_description(&self) -> &'static str {
        ""
    }
    fn get_doc_path(&self) -> &'static str {
        "commas/remove_commas"
    }
    fn get_tags(&self) -> Vec<&'static str> {
        vec!["کاما", "عدد", "خوانایی"]
    }
    fn exe(&self, input: String) -> Result<String, String> {
        Ok(rust_persian_tools::commas::remove_commas(input))
    }
}
