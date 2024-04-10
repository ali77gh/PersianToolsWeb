use super::Tool;
pub struct AddCommas;
impl Tool for AddCommas {
    fn get_route(&self) -> &'static str {
        "/add_commas"
    }
    fn get_name(&self) -> &'static str {
        "اضافه کردن کاما"
    }
    fn get_description(&self) -> &'static str {
        "این ماژول هر سه تا رقم سه تا رقم کاما اضافه میکنه برای راحت تر خونده شدن عدد ها کاربرد داره"
    }
    fn get_doc_path(&self) -> &'static str {
        "commas/add_commas"
    }
    fn get_tags(&self) -> Vec<&'static str> {
        vec!["کاما", "عدد", "خوانایی"]
    }
    fn exe(&self, input: String) -> Result<String, String> {
        Ok(rust_persian_tools::commas::add_commas(input))
    }
}
