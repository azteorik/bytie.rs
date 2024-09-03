use num_complex::Complex;

/// Calculate the Fast Fourier Transform of a list of real numbers
#[poise::command(slash_command)]
pub async fn fft(
    ctx: crate::context::Context<'_>,
    #[description = "Real numbers (integers or floats)"] numbers: String,
) -> Result<(), crate::context::Error> {
    let numbers = parse_real_numbers(&numbers);
    match numbers {
        Ok(valid_numbers) => {
            let output = fft_calculator(valid_numbers);
            let output = output
                .iter()
                .map(|c| format!("{:.2}", c))
                .collect::<Vec<_>>()
                .join("\n");
            ctx.say(output).await?;
        }
        Err(_) => {
            ctx.say("meh").await?;
        }
    }
    Ok(())
}

fn parse_real_numbers(input: &str) -> Result<Vec<Complex<f64>>, String> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<f64>()
                .map(|re| Complex::new(re, 0.0))
                .map_err(|_| "Failed to parse number".to_string())
        })
        .collect()
}

pub fn fft_calculator(input: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = input.len();
    if n == 1 {
        return input;
    }
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for i in 0..n {
        if i % 2 == 0 {
            even.push(input[i]);
        } else {
            odd.push(input[i]);
        }
    }
    let even = fft_calculator(even);
    let odd = fft_calculator(odd);
    let mut output = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n / 2 {
        let t = even[k];
        let u = odd[k] * Complex::new(0.0, -2.0 * std::f64::consts::PI * k as f64 / n as f64).exp();
        output[k] = t + u;
        output[k + n / 2] = t - u;
    }
    output
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(14.0, 0.0),
            Complex::new(9.0, 0.0),
        ];
        let output = fft_calculator(input);
        let expected = vec![
            Complex::new(26.0, 0.0),
            Complex::new(-13.0, 7.0),
            Complex::new(4.0, 0.0),
            Complex::new(-13.0, -7.0),
        ];
        for (a, b) in output.iter().zip(expected.iter()) {
            assert!((a - b).norm() < 1e-6);
        }
    }

    #[test]
    fn test_parser() {
        let input = "1, -2, 3.2, 4";
        let output = parse_real_numbers(input);
        let expected = vec![
            Complex::new(1.0, 0.0),
            Complex::new(-2.0, 0.0),
            Complex::new(3.2, 0.0),
            Complex::new(4.0, 0.0),
        ];
        assert_eq!(output, Ok(expected));
    }
}
