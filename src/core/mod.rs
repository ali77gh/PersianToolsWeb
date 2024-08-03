pub mod tool;
pub mod wrapper;

use wrapper::*;

use lazy_static::lazy_static;

use crate::core::tool::Tool;

lazy_static! {
    pub static ref TOOLS: [Tool; 21] = {
        [
            Tool::new(
                "fa_to_en","digits", digit_converter_fa_en, "۱۲۳۴",
                "تبدیل عدد فارسی به انگلیسی و برعکس", vec!["عدد","اعداد","تبدیل"],"۵۴۱ -> 541"
            ),
            Tool::new(
                "number_to_words","number_to_words", number_to_words, "300123987",
                "عدد به حروف و برعکس", vec!["حروف","عدد"],"عدد رو به حروف تبدیل میکند"
            ),
            Tool::new(
                "extract_card_number","extract_card_number", extract_card_number, "شماره کارتم رو برات نوشتم: ۵۰۲۲-2910-7۰۸۷-۳۴۶۶",
                "شماره کارت بانکی", vec!["بانک","مالی"],"استخراج, اعتبارسنجی و نام بانک شماره کارت بانکی را نمایش می دهد"
            ),
            Tool::new(
                "is_sheba_valid","sheba", sheba, "IR790610000000700796858044",
                "شماره شبا", vec!["بانک","مالی"],"اعتبار سنجی و نام بانک مربوط به شماره شبا"
            ),
            Tool::new(
                "get_city_by_iran_national_id", "get_city_by_iran_national_id", national_id, "0021000000",
                "کد ملی", vec![], "اعتبار سنجی کد ملی و شهر و استان مربوط به آن"
            ),
            Tool::new(
                "get_car_plate_province","get_plate_type",get_car_plate_province, "68",
                "استان - پلاک ماشین", vec!["وسیله ی نقلیه","ماشین" ,"خودرو"],"دو رقم سمت راست پلاک ماشین را میگیرد و استان مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "get_car_plate_category","get_plate_type",get_car_plate_category, "ش",
                "دسته بندی - پلاک ماشین", vec!["وسیله ی نقلیه","ماشین" ,"خودرو"],"حرف روی پلاک ماشین را میگیرد و دسته بندی مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "get_motorcycle_plate_province","get_plate_type",get_motorcycle_plate_province, "442",
                "استان - پلاک موتور", vec!["وسیله ی نقلیه","موتور"],"سه رقم بالای پلاک موتور را میگیرد و استان مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "is_phone_valid","phone_number",phone_number, "09121111111",
                "شماره موبایل", vec!["موبایل","تلفن"],"اعتبار سنجی و نمایش اپراتور و استان مربوط به شماره تلفن همراه"
            ),
            Tool::new(
                "add_commas","commas/add_commas",add_commas,"80000000",
                "اضافه کردن کاما", vec![ "عدد", "خوانایی"],"این ابزار هر سه تا رقم سه تا رقم کاما اضافه میکنه و برای راحت تر خونده شدن عدد ها کاربرد داره"
            ),
            Tool::new(
                "to_persian","persian_chars",to_persian_chars, "ملك",
                "تبدیل به حروف ویژه فارسی", vec!["فارسی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی) \nحروف ویژه ی عربی را به عنوان حروف غیر فارسی در نظر میگیرد"
            ),
            Tool::new(
                "verify_iranian_legal_id","verify_iranian_legal_id",verify_iranian_legal_id, "10380284790",
                "شناسه حقوقی", vec!["اعتبار سنجی"],"شناسه حقوقی را اعتبار سنجی میکند"
            ),
            Tool::new(
                "find_capital_by_province","find_capital_by_province",find_capital_by_province, "البرز",
                "مرکز استان", vec!["شهر"],"استان را به عنوان ورودی گرفته و مرکز استان را نمایش میدهد"
            ),
            Tool::new(
                "get_bill_info","bill",get_bill_info, "77483178001420000001770160",
                "اطلاعات قبض", vec!["قبض"],"نوع و مبلغ قبض رو نمایش میدهد"
            ),
            Tool::new(
                "url_fix","url_fix",url_fix, "https://fa.wikipedia.org/wiki/%D9%85%D8%AF%DB%8C%D8%A7%D9%88%DB%8C%DA%A9%DB%8C:Gadget-Extra-Editbuttons-botworks.js",
                "خوانایی url", vec!["خوانایی"],"url های فارسی ناخونا رو به فرمتی خوانا تبدیل میکند"
            ),
            Tool::new(
                "remove_half_space","remove_half_space",remove_half_space, "نمی‌خواهی درخت‌ها را ببینیم؟",
                "حذف نیم فاصله", vec!["فرمت"],"نیم فاصله ها را با فاصله جایگزین میکند"
            ),
            Tool::new(
                "add_half_space","add_half_space",add_half_space, "نمی خواهی درخت ها را ببینیم؟",
                "نیم فاصله", vec!["تصحیح"],"در صورت نیاز فاصله ها را با نیم فاصله جایگزین میکند"
            ),
            Tool::new(
                "to_arabic","arabic_chars",to_arabic_chars, "ملک",
                "تبدیل به حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "remove_commas","commas/remove_commas",remove_commas,"80,000,000",
                "حذف کردن کاما", vec![ "عدد", "خوانایی"],"این ابزار کاما های یک متن را حذف میکند"
            ),
            Tool::new(
                "add_ordinal_suffix","add_ordinal_suffix",add_ordinal_suffix,"پنج",
                "اضافه کردن پسوند ترتیبی", vec![ "عدد", "ترتیب","فارسی", "حروف"],"پنج را به پنجم تبدیل میکند"
            ),
            Tool::new(
                "remove_ordinal_suffix","remove_ordinal_suffix",remove_ordinal_suffix,"پنجم",
                "حذف کردن پسوند ترتیبی", vec![ "عدد", "ترتیب","فارسی", "حروف"],"پنجم را به پنج تبدیل میکند"
            ),
            // Tool::new(
            //     "time_diff","time_diff",time_diff, "?",
            //     "اختلاف زمان", vec!["زمان", "تاریخ"],"?"
            // ),
        ]
    };
}

pub fn find_by_route(route: &str) -> Option<Tool> {
    TOOLS.iter().find(|tool| tool.route == route).cloned()
}
