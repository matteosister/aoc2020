use std::str::FromStr;

use async_std::fs::File;
use async_std::prelude::*;

use crate::AocResult;
use std::time::{Duration, Instant};

pub async fn read_file<T: FromStr>(file: &str) -> AocResult<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut file = File::open(format!("input/2020/{}.txt", file)).await?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;
    let data = std::str::from_utf8(&contents)?;
    Ok(data.parse().expect("impossible to parse data"))
}

pub async fn read_file_contents(file: &str) -> String {
    let mut file = File::open(format!("input/2020/{}.txt", file))
        .await
        .unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.unwrap();
    let data = std::str::from_utf8(&contents).unwrap();
    data.to_string()
}

pub fn print_result(res: impl ToString, duration: Duration) {
    println!("----------------------------------------");
    println!("Risultato: {} in {:#?}", res.to_string(), duration);
    println!("----------------------------------------");
}

pub fn measure<R>(func: impl FnOnce() -> R) -> (Duration, R) {
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (duration, result)
}
