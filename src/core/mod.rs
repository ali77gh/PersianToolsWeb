pub mod tool;
pub mod wrapper;

use wrapper::*;

use lazy_static::lazy_static;

use crate::core::tool::Tool;

lazy_static! {
    pub static ref TOOLS: [Tool; 26] = {
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
                "is_sheba_valid","sheba", sheba, "IR012345678A01234567890123",
                "شماره شبا", vec!["بانک","مالی"],"اعتبار سنجی و نام بانک مربوط به شماره شبا"
            ),
            Tool::new(
                "get_city_by_iran_national_id", "get_city_by_iran_national_id", national_id, "0021000000",
                "کد ملی", vec![], "اعتبار سنجی کد ملی و شهر و استان مربوط به آن"
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
                "has_persian","persian_chars",has_persian, "abcdefمghij",
                "وجود حروف ویژه ی فارسی", vec!["فارسی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی) \nحروف ویژه ی عربی را به عنوان حروف غیر فارسی در نظر میگیرد"
            ),
            Tool::new(
                "is_persian","persian_chars",is_persian, "ابپتثaجچه",
                "تمامیت حروف ویژه فارسی", vec!["فارسی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی) \nحروف ویژه ی عربی را به عنوان حروف غیر فارسی در نظر میگیرد"
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
                "to_arabic","arabic_chars",to_arabic, "ملک",
                "تبدیل به حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "has_arabic","arabic_chars",has_arabic, "ملك",
                "وجود حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "is_arabic","arabic_chars",is_arabic, "ملك",
                "تمامیت حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "en_to_ar","digits",en_to_ar, "1234",
                "تبدیل عدد انگلیسی به عربی", vec!["عدد","اعداد","تبدیل"],"451 -> ٤٥۱"
            ),
            Tool::new(
                "ar_to_en","digits",ar_to_en, "٤٥",
                "تبدیل عدد عربی به انگلیسی", vec!["عدد","اعداد","تبدیل"],"٤٥۱ -> 451"
            ),
            Tool::new(
                "fa_to_ar","digits" ,fa_to_ar, "۴۵",
                "تبدیل عدد فارسی به عربی", vec!["عدد","اعداد","تبدیل"],"۴۵۱ -> ٤٥۱"
            ),
            Tool::new(
                "ar_to_fa","digits",ar_to_fa, "٤٥",
                "تبدیل عدد عربی به فارسی", vec!["عدد","اعداد","تبدیل"],"٤٥۱ -> 451"
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
            //------------
            // Tool::new(
            //     "get_plate_type","get_plate_type",get_plate_type, "12ب145ایران47",
            //     "نوع پلاک", vec!["وسیله ی نقلیه","موتور","ماشین" ,"خودرو"],"این ابزار پلاک ماشین یا موتور را از ورودی خوانده و نوع آن را نمایش میدهد"
            // ),
            // Tool::new(
            //     "get_plate_province","get_plate_province",get_plate_province, "12ب145ایران47",
            //     "استان پلاک", vec!["وسیله ی نقلیه","موتور","ماشین" ,"خودرو"],"این ابزار پلاک ماشین یا موتور را از ورودی خوانده و استان آن را نمایش میدهد"
            // ),
            // Tool::new(
            //     "get_plate_category","get_plate_category",get_plate_category, "12ب145ایران47",
            //     "دسته بندی پلاک", vec!["وسیله ی نقلیه","موتور","ماشین" ,"خودرو"],"این ابزار پلاک ماشین یا موتور را از ورودی خوانده و دسته بندی آن را نمایش میدهد"
            // ),
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
