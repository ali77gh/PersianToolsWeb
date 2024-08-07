use std::str::FromStr;

use phone_number::operators::Operator;
use rust_persian_tools::*;

pub fn add_commas(s: &str) -> Result<String, String> {
    Ok(commas::add_commas::add_commas(s))
}

pub fn remove_commas(s: &str) -> Result<String, String> {
    Ok(commas::remove_commas::remove_commas(s))
}

pub fn add_ordinal_suffix(s: &str) -> Result<String, String> {
    Ok(add_ordinal_suffix::add_ordinal_suffix(s))
}
pub fn remove_ordinal_suffix(s: &str) -> Result<String, String> {
    Ok(remove_ordinal_suffix::remove_ordinal_suffix(s))
}
pub fn to_arabic_chars(s: &str) -> Result<String, String> {
    let has_arabic = bool_to_persian_string(arabic_chars::has_arabic(s));
    let is_arabic = bool_to_persian_string(arabic_chars::is_arabic(s));
    let to_arabic = arabic_chars::to_arabic_chars(s);
    Ok(format!(
        "عربی دارد: {}\n عربی است: {}\n تبدیل به عربی: \n{}",
        has_arabic, is_arabic, to_arabic
    ))
}
pub fn get_bill_info(s: &str) -> Result<String, String> {
    let b = match bill::Bill::from_str(s) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };

    Ok(format!(
        "نوع:{:?},\n مبلغ:{}",
        b.bill_id.r#type,
        b.amount(bill::CurrencyType::Rials)
    ))
}

pub fn digit_converter_fa_en(s: &str) -> Result<String, String> {
    if persian_chars::is_persian(s, false) {
        Ok(digits::fa_to_en(s))
    } else {
        Ok(digits::en_to_fa(s))
    }
}

pub fn extract_card_number(s: &str) -> Result<String, String> {
    let r = extract_card_number::extract_card_number(s)
        .iter()
        .map(|x| x.get_base().to_string())
        .map(|x| {
            let is_valid = match verity_card_number::verify_card_number(&x) {
                Ok(_) => "معتبر".to_string(),
                Err(_) => "نامعتبر".to_string(),
            };
            let bank_name = match get_bank_name_by_card_number::get_bank_name_by_card_number(&x) {
                Ok(x) => x.to_string(),
                Err(_) => "نامشخص".to_string(),
            };

            format!("{}, {}, {}", &x, is_valid, bank_name)
        })
        .collect::<Vec<String>>();

    if r.is_empty() {
        Err("یافت نشد".to_string())
    } else {
        Ok(r.join("\n"))
    }
}

pub fn find_capital_by_province(s: &str) -> Result<String, String> {
    r_to_r(find_capital_by_province::find_capital_by_province(s).ok_or("یافت نشد"))
}

pub fn national_id(s: &str) -> Result<String, String> {
    let is_valid = match national_id::verify_iranian_national_id(s) {
        Ok(_) => "معتبر",
        Err(_) => "نامعتبر",
    };
    let (city, province) = match get_place_by_iran_national_id::get_place_by_iran_national_id(s) {
        Ok(x) => ((x).get_city(), (x).get_province()),
        Err(_) => ("شهر نامشخص", "استان نامشخص"),
    };
    Ok(format!("{}\n{}\n{}", is_valid, city, province))
}

pub fn remove_half_space(s: &str) -> Result<String, String> {
    Ok(half_space::remove_half_space(s))
}
pub fn add_half_space(s: &str) -> Result<String, String> {
    Ok(half_space::add_half_space(s))
}
pub fn verify_iranian_legal_id(s: &str) -> Result<String, String> {
    r_to_v(legal_id::verify_iranian_legal_id(s))
}

pub fn get_car_plate_province(s: &str) -> Result<String, String> {
    let s = digits::fa_to_en(s);
    let ds = number_plate::codes::car_dataset();
    let result = u32::from_str(&s).map_err(|_| "عدد وارد کنید".to_string())?;
    if s.len() != 2 {
        return Err("دو رقم وارد کنید".to_string());
    }
    let result = ds.get(&result).ok_or("یافت نشد".to_string())?;
    Ok(result.to_string())
}
pub fn get_car_plate_category(s: &str) -> Result<String, String> {
    let car_ds = number_plate::codes::category_dataset();
    let result = car_ds.get(&s).ok_or("یافت نشد".to_string())?;
    Ok(result.to_string())
}
pub fn get_motorcycle_plate_province(s: &str) -> Result<String, String> {
    let s = digits::fa_to_en(s);
    let car_ds = number_plate::codes::motorcycle_dataset();
    let result = u32::from_str(&s).map_err(|_| "عدد وارد کنید".to_string())?;
    if s.len() != 3 {
        return Err("سه رقم وارد کنید".to_string());
    }
    let result = car_ds.get(&result).ok_or("یافت نشد".to_string())?;
    Ok(result.to_string())
}

pub fn number_to_words(s: &str) -> Result<String, String> {
    let n = digits::fa_to_en(digits::ar_to_en(s));
    if n.parse::<i64>().is_ok() {
        r_to_r(number_to_words::number_to_words_str(n))
    } else {
        r_to_r(words_to_number::words_to_number_str(
            s,
            &words_to_number::Options::default(),
        ))
    }
}

pub fn to_persian_chars(s: &str) -> Result<String, String> {
    let has_persian = bool_to_persian_string(persian_chars::has_persian(s, false));
    let is_persian = bool_to_persian_string(persian_chars::is_persian(s, false));
    let to_persian = persian_chars::to_persian_chars(s);
    Ok(format!(
        "فارسی دارد: {}\n فارسی است: {}\n تبدیل به فارسی: \n{}",
        has_persian, is_persian, to_persian
    ))
}

pub fn phone_number(s: &str) -> Result<String, String> {
    let is_valid = match phone_number::is_phone_valid(s) {
        Ok(_) => "معتبر",
        Err(_) => return Err("نامعتبر".to_string()), // TODO do same thing (return error on other functions (merged ones))
    };
    let (operator, province) = match phone_number::operators::get_phone_details(s) {
        Ok(x) => ((x).operator(), x.base()),
        Err(_) => return Ok(is_valid.to_string()),
    };
    Ok(format!(
        "{}\n{}\n{}",
        is_valid,
        get_operator_persian_name(&operator),
        province
    ))
}

//TODO move this to rust persian tools
fn get_operator_persian_name(op: &Operator) -> &'static str {
    match op {
        Operator::ShatelMobile => "شاتل",
        Operator::MCI => "همراه اول",
        Operator::Irancell => "ایرانسل",
        Operator::Taliya => "تالیا",
        Operator::RightTel => "رایتل",
    }
}

pub fn sheba(s: &str) -> Result<String, String> {
    let is_valid = match sheba::is_sheba_valid(s) {
        Ok(()) => "معتبر".to_string(),
        Err(_) => "نامعتبر".to_string(),
    };
    let (name, persian_name) = match sheba::get_sheba_info(s) {
        Ok(info) => (info.get_name(), info.get_persian_name()),
        Err(_) => ("بانک نا مشخص", ""),
    };
    Ok(format!("{}\n{}\n{}", is_valid, name, persian_name))
}

// pub fn time_diff(s: &str) -> Result<String, String> {
//     match rust_persian_tools::time_diff::time_diff_now(s) {
//         Ok(time_diff) => Ok(time_diff.long_form()),
//         Err(e) => Err(e.to_string()),
//     }
// }
pub fn url_fix(s: &str) -> Result<String, String> {
    r_to_r(url_fix::url_fix(s, None))
}
// ---

fn r_to_r<T, E>(i: Result<T, E>) -> Result<String, String>
where
    T: ToString,
    E: ToString,
{
    match i {
        Ok(x) => Ok(x.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
fn r_to_v<E>(i: Result<(), E>) -> Result<String, String>
where
    E: ToString,
{
    match i {
        Ok(_) => Ok("Ok".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn bool_to_persian_string(b: bool) -> &'static str {
    if b {
        "بله"
    } else {
        "خیر"
    }
}
