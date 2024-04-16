pub mod tool;
pub mod wrapper;

use wrapper::*;

use lazy_static::lazy_static;

use crate::core::tool::Tool;

lazy_static! {
    pub static ref TOOLS: [Tool; 2] = {
        [
            Tool::new(
                "add_commas","commas/add_commas",add_commas,
                "اضافه کردن کاما", vec!["کاما", "عدد", "خوانایی"],"این ابزار هر سه تا رقم سه تا رقم کاما اضافه میکنه برای راحت تر خونده شدن عدد ها کاربرد داره"
            ),
            Tool::new(
                "remove_commas","commas/remove_commas",remove_commas,
                "حذف کردن کاما", vec!["کاما", "عدد", "خوانایی"],"این ابزار کاما های یک متن را حذف میکند"
            ),
        ]
    };
}
