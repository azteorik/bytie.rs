use crate::context::{Context, Error};

fn mean(numbers: &Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn sumdiffsq(numbers: &Vec<f64>) -> f64 {
    let m = mean(numbers);
    let sum: f64 = numbers.iter().map(|x| (x - m).powi(2)).sum();
    return sum;
}

fn sumdiffsq2(xs: &Vec<f64>, ys: &Vec<f64>) -> f64 {
    let m1 = mean(xs);
    let m2 = mean(ys);
    let sum: f64 = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| (x - m1) * (y - m2))
        .sum();
    return sum;
}

fn make_time_variable(n: usize) -> Vec<f64> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as f64);
    }
    return v;
}

fn linear_trend_eq(y: &Vec<f64>) -> (f64, f64) {
    let x = make_time_variable(y.len());
    let b = sumdiffsq2(&x, y) / sumdiffsq(&x);
    let a = mean(y) - b * mean(&x);
    return (a, b);
}

fn predict_next(y: &Vec<f64>) -> f64 {
    let (a, b) = linear_trend_eq(y);
    let x = y.len() as f64;
    return a + b * x;
}

fn string_to_v64(s: &str) -> Vec<f64> {
    s.split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect()
}

#[poise::command(slash_command)]
pub async fn trend(
    ctx: Context<'_>,
    #[description = "Sequence of numbers"] numbers: String,
) -> Result<(), Error> {
    let vals = string_to_v64(&numbers);
    let (a, b) = linear_trend_eq(&vals);
    let nextvalue = predict_next(&vals);

    let outstr = format!("Data: {:?}, The linear trend: y = {} + {}x, the prediction is {}", vals, a, b, nextvalue);
    ctx.say(outstr).await?;
    Ok(())
}
