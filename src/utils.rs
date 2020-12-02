use std::str::FromStr;

use async_std::fs::File;
use async_std::prelude::*;

use crate::AocResult;
use tokio::time::Duration;

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

pub fn print_result(res: impl ToString, duration: Duration) {
    println!("----------------------------------------");
    println!("Risultato: {} in {:#?}", res.to_string(), duration);
    println!("----------------------------------------");
}

pub fn measure<R>(func: impl FnOnce() -> R) -> (Duration, R) {
    use std::time::{Duration, Instant};
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (duration, result)
}
