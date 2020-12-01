use std::str::FromStr;

use async_std::fs::File;
use async_std::prelude::*;

use crate::AocResult;

pub async fn read_file<T: FromStr>(file: &str) -> AocResult<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut file = File::open(format!("input/{}", file)).await?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;
    let data = std::str::from_utf8(&contents)?;
    Ok(data.parse().expect("impossible to parse data"))
}

pub fn print_result(res: impl ToString) {
    println!("----------------------------------------");
    println!("Risultato: {}", res.to_string());
    println!("----------------------------------------");
}
