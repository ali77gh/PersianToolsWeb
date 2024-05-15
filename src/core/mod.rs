pub mod tool;
pub mod wrapper;

use wrapper::*;

use lazy_static::lazy_static;

use crate::core::tool::Tool;

lazy_static! {
    pub static ref TOOLS: [Tool; 40] = {
        [
            Tool::new(
                "add_commas","commas/add_commas",add_commas,"80000000",
                "اضافه کردن کاما", vec![ "عدد", "خوانایی"],"این ابزار هر سه تا رقم سه تا رقم کاما اضافه میکنه و برای راحت تر خونده شدن عدد ها کاربرد داره"
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
            Tool::new(
                "has_arabic","arabic_chars",has_arabic, "ملك",
                "وجود حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "is_arabic","arabic_chars",is_arabic, "ملك",
                "تمامیت حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "to_arabic","arabic_chars",to_arabic, "ملک",
                "تبدیل به حروف عربی", vec!["عربی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی)"
            ),
            Tool::new(
                "get_bill_info","bill",get_bill_info, "77483178001420000001770160",
                "اطلاعات قبض", vec!["قبض"],"نوع و مبلغ قبض رو نمایش میدهد"
            ),
            Tool::new(
                "fa_to_en","digits",fa_to_en, "۱۲۳۴",
                "تبدیل عدد فارسی به انگلیسی", vec!["عدد","اعداد","تبدیل"],"۵۴۱ -> 541"
            ),
            Tool::new(
                "en_to_fa","digits",en_to_fa, "1234",
                "تبدیل عدد انگلیسی به فارسی", vec!["عدد","اعداد","تبدیل"],"541 -> ۵۴۱"
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
                "extract_card_number","extract_card_number",extract_card_number, "شماره کارتم رو برات نوشتم: ۵۰۲۲-2910-7۰۸۷-۳۴۶۶",
                "استخراج شماره کارت", vec!["بانک","مالی"],"در متن وارد شده شماره کارت پیدا میکند و به حروف انگلیسی تبدیل میکند"
            ),
            Tool::new(
                "find_capital_by_province","find_capital_by_province",find_capital_by_province, "البرز",
                "مرکز استان", vec!["شهر"],"استان را به عنوان ورودی گرفته و مرکز استان را نمایش میدهد"
            ),
            Tool::new(
                "get_bank_name_by_card_number","get_bank_name_by_card_number",get_bank_name_by_card_number, "6219861000000000",
                "بانک از کارت", vec!["بانک","مالی"],"شماره کارت را از ورودی گرفته و نام بانک مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "get_city_by_iran_national_id","get_city_by_iran_national_id",get_city_by_iran_national_id, "0021000000",
                "کد ملی شهر", vec!["شهر"],"کد ملی را از ورودی گرفته و شهر مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "get_province_by_iran_national_id","get_province_by_iran_national_id",get_province_by_iran_national_id, "0021000000",
                "کد ملی استان", vec!["استان"],"کد ملی را از ورودی گرفته و استان مربوط به آن را نمایش میدهد"
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
                "verify_iranian_legal_id","verify_iranian_legal_id",verify_iranian_legal_id, "10380284790",
                "شناسه حقوقی", vec!["اعتبار سنجی"],"شناسه حقوقی را اعتبار سنجی میکند"
            ),
            Tool::new(
                "verify_iranian_national_id","verify_iranian_national_id",verify_iranian_national_id, "68415941",
                "کد ملی", vec!["اعتبار سنجی"],"این ابزار کد ملی را اعتبار سنجی میکند"
            ),
            Tool::new(
                "get_plate_type","get_plate_type",get_plate_type, "12ب145ایران47",
                "نوع پلاک", vec!["وسیله ی نقلیه","موتور","ماشین" ,"خودرو"],"این ابزار پلاک ماشین یا موتور را از ورودی خوانده و نوع آن را نمایش میدهد"
            ),
            Tool::new(
                "get_plate_province","get_plate_province",get_plate_province, "12ب145ایران47",
                "استان پلاک", vec!["وسیله ی نقلیه","موتور","ماشین" ,"خودرو"],"این ابزار پلاک ماشین یا موتور را از ورودی خوانده و استان آن را نمایش میدهد"
            ),
            Tool::new(
                "get_plate_category","get_plate_category",get_plate_category, "12ب145ایران47",
                "دسته بندی پلاک", vec!["وسیله ی نقلیه","موتور","ماشین" ,"خودرو"],"این ابزار پلاک ماشین یا موتور را از ورودی خوانده و دسته بندی آن را نمایش میدهد"
            ),
            Tool::new(
                "number_to_words","number_to_words",number_to_words, "300123987",
                "عدد به حروف", vec!["حروف","عدد"],"عدد رو به حروف تبدیل میکند"
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
                "to_persian","persian_chars",to_persian_chars, "ملك",
                "تبدیل به حروف ویژه فارسی", vec!["فارسی","حروف ویژه"],"حروف ویژه ی عربی (ك یا ي یا ی) \nحروف ویژه ی عربی را به عنوان حروف غیر فارسی در نظر میگیرد"
            ),
            Tool::new(
                "is_phone_valid","phone_number",is_phone_valid, "09121111111",
                "اعتبار سنجی شماره تلفن", vec!["موبایل","تلفن"],"نکته: این ابزار نیز مانند تمامی ابزار های دیگر این برنامه آفلاین است و منظور از اعتبار سنجی صرفا چک کردن مواردی است که به عنوان استاندارد برای شماره تلفن همراه مشخص شده"
            ),
            Tool::new(
                "get_operator_prefix","phone_number",get_operator_prefix, "09121111111",
                "پیش شماره", vec!["تلفن","موبایل"],"شماره تلفن همراه را از ورودی گرفته و پیش شماره را نمایش میدهد"
            ),
            Tool::new(
                "get_phone_operator","phone_number",get_phone_operator, "09121111111",
                "اپراتور از شماره تلفن", vec!["تلفن","موبایل"],"شماره تلفن همراه را از ورودی گرفته و اپراتور را نمایش میدهد"
            ),
            Tool::new(
                "get_phone_province","phone_number",get_phone_province, "09121111111",
                "استان شماره تلفن", vec!["تلفن","موبایل"],"شماره تلفن همراه را از ورودی گرفته و در صورت امکان استان مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "is_sheba_valid","sheba",is_sheba_valid, "IR012345678A01234567890123",
                "اعتبار سنجی شبا", vec!["بانک","مالی"],"شماره شبا را از ورودی خوانده و ان را اعتبار سنجی میکند"
            ),
            Tool::new(
                "sheba_to_bank_name","sheba",sheba_to_bank_name, "IR012345678A01234567890123",
                "بانک از شماره شب(انگلیسی)", vec!["بانک","مالی"],"شماره شبا را از ورودی خوانده و نام انگلیسی بانک مربوط به آن را نمایش میدهد"
            ),
            Tool::new(
                "sheba_to_persian_bank_name","sheba",sheba_to_persian_bank_name, "IR012345678A01234567890123",
                "(فارسی)بانک از شماره شبا", vec!["مالی","بانک"],"شماره شبا را از ورودی خوانده و نام فارسی بانک مربوط به آن را نمایش میدهد"
            ),
            // Tool::new(
            //     "time_diff","time_diff",time_diff, "?",
            //     "اختلاف زمان", vec!["زمان", "تاریخ"],"?"
            // ),
            Tool::new(
                "url_fix","url_fix",url_fix, "https://fa.wikipedia.org/wiki/%D9%85%D8%AF%DB%8C%D8%A7%D9%88%DB%8C%DA%A9%DB%8C:Gadget-Extra-Editbuttons-botworks.js",
                "خوانایی url", vec!["خوانایی"],"url های فارسی ناخونا رو به فرمتی خوانا تبدیل میکند"
            ),
            Tool::new(
                "verify_card_number","verify_card_number",verify_card_number, "6037701689095443",
                "اعتبار سنجی شماره کارت", vec!["بانک"],"شماره کارت از ورودی میگیرد و اعتبار سنجی میکند"
            ),
            Tool::new(
                "words_to_number","words_to_number",words_to_number, "پنجاه و سه",
                "حروف به عدد", vec![],"حروف را به عدد تبدیل میکند"
            ),
        ]
    };
}

pub fn find_by_route(route: &str) -> Option<Tool> {
    TOOLS.iter().find(|tool| tool.route == route).cloned()
}
