use crate::context::{Context, Error};

fn collatz_sequence(n: u64) -> Vec<u64> {
    let mut sequence: Vec<u64> = Vec::new();

    if n < 1 {
        return sequence;
    }

    let mut x = n;

    sequence.push(x);

    while x > 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        sequence.push(x);
    }
    return sequence;
}

/// Returns the Collatz sequence for a given pozitive integer number
#[poise::command(slash_command)]
pub async fn collatz(
    ctx: Context<'_>,
    #[description = "Positive integer"] n: u64,
) -> Result<(), Error> {
    let sequence = collatz_sequence(n);
    let outstr = format!("Collatz sequence for {}: {:?}", n, sequence);
    ctx.say(outstr).await?;
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_sequence() {
        assert_eq!(collatz_sequence(0), Vec::<u64>::new());
        assert_eq!(collatz_sequence(1), vec![1]);
        assert_eq!(collatz_sequence(2), vec![2, 1]);
        assert_eq!(collatz_sequence(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(collatz_sequence(4), vec![4, 2, 1]);
    }
}