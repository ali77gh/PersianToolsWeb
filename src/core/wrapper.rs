use std::str::FromStr;

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
pub fn has_arabic(s: &str) -> Result<String, String> {
    Ok(arabic_chars::has_arabic(s).to_string())
}
pub fn is_arabic(s: &str) -> Result<String, String> {
    Ok(arabic_chars::is_arabic(s).to_string())
}
pub fn to_arabic(s: &str) -> Result<String, String> {
    Ok(arabic_chars::to_arabic_chars(s))
}
pub fn get_bill_info(s: &str) -> Result<String, String> {
    let b = match bill::Bill::from_str(s) {
        Ok(x) => x,
        Err(e) => return Err(e.to_string()),
    };

    Ok(format!(
        "type:{:?},\nname:{}",
        b.bill_id.r#type,
        b.amount(bill::CurrencyType::Rials)
    ))
}

pub fn fa_to_en(s: &str) -> Result<String, String> {
    Ok(digits::fa_to_en(s))
}
pub fn en_to_fa(s: &str) -> Result<String, String> {
    Ok(digits::en_to_fa(s))
}
pub fn en_to_ar(s: &str) -> Result<String, String> {
    Ok(digits::en_to_ar(s))
}
pub fn ar_to_en(s: &str) -> Result<String, String> {
    Ok(digits::ar_to_en(s))
}
pub fn fa_to_ar(s: &str) -> Result<String, String> {
    Ok(digits::fa_to_ar(s))
}
pub fn ar_to_fa(s: &str) -> Result<String, String> {
    Ok(digits::ar_to_fa(s))
}
pub fn extract_card_number(s: &str) -> Result<String, String> {
    Ok(extract_card_number::extract_card_number(s)
        .iter()
        .map(|x| x.get_base())
        .collect::<Vec<&str>>()
        .join(", "))
}
pub fn find_capital_by_province(s: &str) -> Result<String, String> {
    r_to_r(find_capital_by_province::find_capital_by_province(s).ok_or("not found"))
}

pub fn get_bank_name_by_card_number(s: &str) -> Result<String, String> {
    r_to_r(get_bank_name_by_card_number::get_bank_name_by_card_number(
        s,
    ))
}
pub fn get_city_by_iran_national_id(s: &str) -> Result<String, String> {
    match get_place_by_iran_national_id::get_place_by_iran_national_id(s) {
        Ok(x) => Ok(x.get_city().to_string()),
        Err(e) => Err(e.to_string()),
    }
}
pub fn get_province_by_iran_national_id(s: &str) -> Result<String, String> {
    match get_place_by_iran_national_id::get_place_by_iran_national_id(s) {
        Ok(x) => Ok(x.get_province().to_string()),
        Err(e) => Err(e.to_string()),
    }
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
pub fn verify_iranian_national_id(s: &str) -> Result<String, String> {
    r_to_v(national_id::verify_iranian_national_id(s))
}
pub fn get_plate_type(s: &str) -> Result<String, String> {
    match rust_persian_tools::number_plate::get_plate_info(s) {
        Ok(plate) => Ok(format!("{:?}", plate.plate_type)),
        Err(e) => Err(e.to_string()),
    }
}
pub fn get_plate_province(s: &str) -> Result<String, String> {
    match rust_persian_tools::number_plate::get_plate_info(s) {
        Ok(plate) => Ok(plate.province),
        Err(e) => Err(e.to_string()),
    }
}
pub fn get_plate_category(s: &str) -> Result<String, String> {
    match rust_persian_tools::number_plate::get_plate_info(s) {
        Ok(plate) => r_to_r(plate.category.ok_or("not found")),
        Err(e) => Err(e.to_string()),
    }
}
pub fn number_to_words(s: &str) -> Result<String, String> {
    r_to_r(number_to_words::number_to_words_str(s))
}
pub fn has_persian(s: &str) -> Result<String, String> {
    Ok(persian_chars::has_persian(s, false).to_string())
}
pub fn is_persian(s: &str) -> Result<String, String> {
    Ok(persian_chars::is_persian(s, false).to_string())
}
pub fn to_persian_chars(s: &str) -> Result<String, String> {
    Ok(persian_chars::to_persian_chars(s))
}
pub fn is_phone_valid(s: &str) -> Result<String, String> {
    r_to_v(phone_number::is_phone_valid(s))
}
pub fn get_operator_prefix(s: &str) -> Result<String, String> {
    r_to_r(phone_number::get_operator_prefix(s))
}
pub fn get_phone_operator(s: &str) -> Result<String, String> {
    match phone_number::operators::get_phone_details(s) {
        Ok(x) => Ok(format!("{:?}", x.operator())),
        Err(e) => Err(e.to_string()),
    }
}
pub fn get_phone_province(s: &str) -> Result<String, String> {
    match phone_number::operators::get_phone_details(s) {
        Ok(x) => Ok(format!("{:?}", x.base())),
        Err(e) => Err(e.to_string()),
    }
}
pub fn is_sheba_valid(s: &str) -> Result<String, String> {
    r_to_v(sheba::is_sheba_valid(s))
}
pub fn sheba_to_bank_name(s: &str) -> Result<String, String> {
    match sheba::get_sheba_info(s) {
        Ok(x) => Ok(x.get_name().to_string()),
        Err(e) => Err(e.to_string()),
    }
}
pub fn sheba_to_persian_bank_name(s: &str) -> Result<String, String> {
    match sheba::get_sheba_info(s) {
        Ok(x) => Ok(x.get_persian_name().to_string()),
        Err(e) => Err(e.to_string()),
    }
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
pub fn verify_card_number(s: &str) -> Result<String, String> {
    r_to_v(verity_card_number::verify_card_number(s))
}
// ---
pub fn words_to_number(s: &str) -> Result<String, String> {
    r_to_r(words_to_number::words_to_number_str(
        s,
        &rust_persian_tools::words_to_number::Options::default(),
    ))
}

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
