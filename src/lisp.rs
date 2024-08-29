use crate::context::{Context, Error};
use std::io::prelude::*;
use tulisp::TulispContext;

#[poise::command(slash_command)]
pub async fn lisp(
    ctx: Context<'_>,
    #[description = "Lisp code"] code: String,
) -> Result<(), Error> {
    let temp_file_name = "temp.lisp";
    let mut file = std::fs::File::create(temp_file_name)?;
    file.write_all(code.as_bytes())?;

    let result = tokio::task::spawn_blocking(move || {
        let mut lisp_ctx = TulispContext::new();
        match lisp_ctx.eval_file(&temp_file_name) {
            Ok(value) => value.to_string(),
            Err(_) => "LispError!".to_string(),
        }
    })
    .await?;

    std::fs::remove_file(temp_file_name)?;

    ctx.say(result).await?;
    Ok(())
}
