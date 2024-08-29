use crate::context::{Context, Error};

/// Responds with the USD/TRY parity
#[poise::command(slash_command)]
pub async fn usdtry(ctx: Context<'_>) -> Result<(), Error> {
    let parity = get_usd_try().await;
    let buy = parity.get(0).unwrap();
    let sell = parity.get(1).unwrap();
    ctx.say(format!("{buy} - {sell}")).await?;
    Ok(())
}

pub async fn get_usd_try() -> Vec<String> {
    let mut parity = Vec::new();
    let url = "https://www.turkiye.gov.tr/doviz-kurlari";
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    let document = scraper::Html::parse_document(&body);
    let kurboxes_selector = scraper::Selector::parse("tr").unwrap();
    let kur_boxes = document.select(&kurboxes_selector);
    for kur_box in kur_boxes {
        let content = kur_box.text().collect::<Vec<_>>();
        if content.len() >= 5 {
            if content[1] == "1 ABD DOLARI" {
                //println!("{:?}", content[3]);
                //println!("{:?}", content[5]);
                parity.push(String::from(content[3]));
                parity.push(String::from(content[5]));
            }
        }
    }
    return parity;
}
