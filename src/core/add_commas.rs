use super::Tool;
pub struct AddCommas;
impl Tool for AddCommas {
    fn route(&self) -> &'static str {
        "/add_commas"
    }
    fn getName(&self) -> &'static str {
        "اضافه کردن کاما"
    }
    fn getDescription(&self) -> &'static str {
        "این ماژول هر سه تا رقم سه تا رقم کاما اضافه میکنه برای راحت تر خونده شدن عدد ها کاربرد داره"
    }
    fn getDocPath(&self) -> &'static str {
        "commas/add_commas"
    }
    fn getTags(&self) -> Vec<&'static str> {
        vec!["کاما", "عدد", "خوانایی"]
    }
    fn exe(&self, input: String) -> Result<String, String> {
        Ok(rust_persian_tools::commas::add_commas(input))
    }
}
