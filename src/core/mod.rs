pub mod add_commas;
pub mod remove_commas;

pub trait Tool {
    fn get_route(&self) -> &'static str;
    fn get_name(&self) -> &'static str; // seeable on list and search
    fn get_description(&self) -> &'static str; // expand to see
    fn get_doc_link(&self) -> String {
        format!(
            "{}/{}/{}",
            "https://docs.rs/rust-persian-tools/latest/rust_persian_tools",
            &self.get_doc_path(),
            "index.html"
        )
    } // expand to see
    fn get_doc_path(&self) -> &'static str; // expand to see
    fn get_tags(&self) -> Vec<&'static str>; // used for search
    fn exe(&self, input: String) -> Result<String, String>; // string in string out
}

pub fn get_all_tools() -> [Box<dyn Tool>; 2] {
    [
        Box::new(crate::core::add_commas::AddCommas),
        Box::new(crate::core::remove_commas::RemoveCommas),
    ]
}
