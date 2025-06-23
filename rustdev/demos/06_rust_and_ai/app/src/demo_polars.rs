use std::fs::File;
use polars::prelude::*;
use polars::lazy::dsl::col;

pub fn do_demo() {

    let df = create_dataframe_from_csv();
    let _  = process_dataframe(df);
}

fn create_dataframe_from_csv() -> DataFrame{
    
    println!("\ncreate_dataframe_from_csv");
    println!("--------------------------------------");

    let file = File::open("WorldCupWinners.csv").expect("Can't open CSV file");

    let df = CsvReader::new(file)
        .has_header(true)  
        .finish()
        .expect("Can't parse CSV file");

    println!("{:?}", df);

    return df
}

fn process_dataframe(df: DataFrame) -> Result<(), Box<dyn std::error::Error>> {

    println!("\nprocess_dataframe");
    println!("--------------------------------------");

    let df_multiple_winners = df
        .clone()
        .lazy()
        .filter(col("Wins").gt(1))
        .collect()?;

    println!("DataFrame of multiple winners: {:?}", df_multiple_winners);

    let df_with_multiple_winner_column = df
        .clone()
        .lazy()
        .with_column(
            (col("Wins").gt(1))
            .alias("IsMulipleWinner")
        )
        .collect()?;

    println!("DataFrame with IsMultipleWinner column added: {:?}", df_with_multiple_winner_column);

    Ok(())
}