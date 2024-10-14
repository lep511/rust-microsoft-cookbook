use chrono::prelude::*;
use polars::prelude::*;
use std::fs::File;

fn example() -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("iris.csv".into()))?
            .finish()
}

fn main() {

    let mut df: DataFrame = df!(
        "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
        "birthdate" => [
            NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
            NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
            NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
            NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
        ],
        "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
        "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
    )
    .unwrap();
    println!("{}", df);
    let mut new_df: DataFrame = example().unwrap();
    println!("{}", new_df);

    let result = df
        .clone()
        .lazy()
        .select([
            col("name"),
            col("birthdate").dt().year().alias("birth_year"),
            (col("weight") / col("height").pow(2)).alias("bmi"),
        ])
        .collect();
    println!("{:?}", result);

    println!("{:?}", df.schema());


}