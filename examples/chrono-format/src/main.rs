use chrono::{NaiveDate, DateTime, Weekday};
use chrono::{Utc, NaiveDateTime, Local};
use chrono::Datelike;

fn main() {
    let date = "18-2-2023";
    let format = "%d-%m-%Y";
    let map_date = NaiveDate::parse_from_str(date, format);
    if let Ok(map_date) = map_date {
        println!("The date is {}", map_date);
    } else {
        println!("Error parsing date");
    }

    // Obtener la fecha y hora actual
    let now = Utc::now();
    
    // Extraer el año
    let year = now.year_ce();
    println!("El año actual es: {:?}", year.1);

    // Checar si el año es bisiesto
    let year_parse = year.1 as i32;
    let leap_year: bool = NaiveDate::from_yo_opt(year_parse, 366).is_some();
    if leap_year {
        println!("El año actual es bisiesto");
    } else {
        println!("El año actual no es bisiesto");
    }

    let new_date = NaiveDate::from_yo_opt(2024, 166);
    if let Some(new_date) = new_date {
        println!("La fecha 166 dias de 2024 es: {}", new_date);
    } else {
        println!("Error al crear la fecha");
    }

    // También puedes extraer el año de una fecha específica
    let date_time = NaiveDateTime::parse_from_str("2024-03-22 15:30:00", "%Y-%m-%d %H:%M:%S")
        .expect("Error al analizar la fecha");
    let year_from_specific_date = date_time.year_ce();
    println!("El año de la fecha específica es: {}", year_from_specific_date.1);

    let week = NaiveDate::from_weekday_of_month_opt(2017, 3, Weekday::Fri, 2);
    if let Some(week) = week {
        println!("La fecha de la semana 2 del mes 3 de 2017 es: {}", week);
    } else {
        println!("Error al crear la fecha");
    }

    let new_date = "2014-5-17T12:34:56+09:30";
    let fmt_date = NaiveDate::parse_from_str(new_date, "%Y-%m-%dT%H:%M:%S%z");
    if let Ok(fmt_date) = fmt_date {
        println!("La fecha es: {}", fmt_date);
    } else {
        println!("Error al crear la fecha");
    }

    let utc_date = Utc::now().date().naive_utc();
    let local_date = Local::now().date().naive_local();
    println!("UTC date: {}", utc_date);
    println!("Local date: {}", local_date);
    let local_time = Local::now().time();
    println!("Local time: {}", local_time);
    let timestamp = Local::now().timestamp();
    println!("Timestamp: {}", timestamp);
}