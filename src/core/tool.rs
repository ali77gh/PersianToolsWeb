type Exe = fn(inp: &str) -> Result<String, String>;

#[derive(Clone, PartialEq)]
pub struct Tool {
    pub name: &'static str,        // seeable on list and search
    pub description: &'static str, // expand to see
    pub tags: Vec<&'static str>,   // used for search
    pub exe: Exe,

    pub doc_path: &'static str, // expand to see
    pub route: &'static str,
    pub doc_link: String,
    pub sample_input: &'static str,
}

// route, doc_path, exe,  name, tags, description
impl Tool {
    pub fn new(
        route: &'static str,
        doc_path: &'static str,
        exe: Exe,
        sample_input: &'static str,
        name: &'static str,
        tags: Vec<&'static str>,
        description: &'static str,
    ) -> Self {
        let doc_link = format!(
            "{}/{}/{}",
            "https://docs.rs/rust-persian-tools/latest/rust_persian_tools", doc_path, "index.html"
        );
        Self {
            name,
            description,
            tags,
            exe,
            doc_path,
            route,
            doc_link,
            sample_input,
        }
    }
}
