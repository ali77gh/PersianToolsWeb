pub mod add_commas;
pub mod remove_commas;

const BASE_DOC_PATH: &'static str = "https://docs.rs/rust-persian-tools/latest/rust_persian_tools";

pub trait Tool {
    fn route(&self) -> &'static str; // seeable on list and search
    fn getName(&self) -> &'static str; // seeable on list and search
    fn getDescription(&self) -> &'static str; // expand to see
    fn getDocLink(&self) -> String {
        format!("{}/{}/{}", BASE_DOC_PATH, &self.getDocPath(), "index.html")
    } // expand to see
    fn getDocPath(&self) -> &'static str; // expand to see
    fn getTags(&self) -> Vec<&'static str>; // used for search
    fn exe(&self, input: String) -> Result<String, String>; // string in string out
}

pub fn getAllTools<T>() -> Vec<Box<dyn Tool>> {
    use crate::core::add_commas::AddCommas;
    use crate::core::remove_commas::RemoveCommas;

    let mut list: Vec<Box<dyn Tool>> = Vec::new();

    list.push(Box::new(AddCommas));
    list.push(Box::new(RemoveCommas));

    list
}
