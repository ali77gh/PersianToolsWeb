pub mod tool;
pub mod wrapper;

use wrapper::*;

use lazy_static::lazy_static;

use crate::core::tool::Tool;

lazy_static! {
    pub static ref TOOLS: [Tool; 2] = {
        [
            Tool::new(
                "add_commas","commas/add_commas",add_commas,"80000000",
                "اضافه کردن کاما", vec![ "عدد", "خوانایی"],"این ابزار هر سه تا رقم سه تا رقم کاما اضافه میکنه برای راحت تر خونده شدن عدد ها کاربرد داره"
            ),
            Tool::new(
                "remove_commas","commas/remove_commas",remove_commas,"80,000,000",
                "حذف کردن کاما", vec![ "عدد", "خوانایی"],"این ابزار کاما های یک متن را حذف میکند"
            ),
        ]
    };
}

pub fn find_by_route(route: &str) -> Option<Tool> {
    TOOLS.iter().find(|tool| tool.route == route).cloned()
}
