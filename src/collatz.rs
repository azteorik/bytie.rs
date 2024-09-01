use crate::context::{Context, Error};

fn collatz_sequence(n: u64) -> Vec<u64> {
    let mut sequence = vec![n];

    if n < 1 {
        return sequence;
    }

    let mut x = n;
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
