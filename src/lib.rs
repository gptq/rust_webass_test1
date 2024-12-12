use wasm_bindgen::prelude::*;
use chrono::{NaiveDateTime, Datelike, Utc};
use chinese_lunisolar_calendar::{SolarDate, LunisolarDate};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimeInfo {
    lunar_date: String,
    zodiac_year: String,
}

#[wasm_bindgen]
pub fn get_lunar_info(time_str: &str) -> String {
    // 解析输入的ISO时间字符串
    let date_time = NaiveDateTime::parse_from_str(
        &time_str.replace("+08:00", "Z"),
        "%Y-%m-%dT%H:%M:%S%.3fZ"
    ).unwrap_or_else(|_| Utc::now().naive_utc());

    // 获取年月日
    let year = date_time.year();
    let month = date_time.month();
    let day = date_time.day();

    // 转换为农历
    let solar_date = match SolarDate::from_ymd(
        year.try_into().unwrap_or(2024),
        month as u8,
        day as u8
    ) {
        Ok(date) => date,
        Err(_) => return "{}".to_string()
    };

    let lunisolar_date = match LunisolarDate::from_solar_date(solar_date) {
        Ok(date) => date,
        Err(_) => return "{}".to_string()
    };

    // 获取农历日期字符串（使用简体中文）
    let lunar_str = format!("{:#}", lunisolar_date);

    // 获取生肖年份
    let zodiac_year = format!("{}年", lunisolar_date.to_string());

    // 构造返回信息
    let info = TimeInfo {
        lunar_date: lunar_str,
        zodiac_year,
    };

    // 序列化为JSON字符串
    serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
