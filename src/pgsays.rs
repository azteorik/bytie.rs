use crate::context::{Context, Error};
use rand::prelude::*;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

const BASE_URL: &str = "https://paulgraham.com";
const SCRIPT_REGEX: &str = r"<script[^>]*>(\n?(.*?)\n?)+<\/script>";
const SENTENCE_REGEX: &str = r"(?s)(?:[^.!?])+[.!?]+\s*"; // taken from https://stackoverflow.com/a/65853729/288652

async fn find_random_essay() -> Result<String, Error> {
    let essays_url = BASE_URL.to_string() + "/articles.html";
    let essays_response = reqwest::get(essays_url).await?.text().await?;
    let html_fragment = Html::parse_document(essays_response.as_str());
    let selector = Selector::parse("td a").unwrap();
    let links = html_fragment.select(&selector);
    let hrefs: Vec<&str> = links.filter_map(|element| element.attr("href")).collect();
    let random_index = thread_rng().gen_range(0..hrefs.len());
    Ok(hrefs[random_index].to_string())
}

async fn get_essay_content(essay_link: String) -> Result<String, Error> {
    let essay_url = BASE_URL.to_string() + "/" + &essay_link;
    Ok(reqwest::get(essay_url).await?.text().await?)
}

fn get_random_sentence(essay_content: String) -> String {
    let content_fragment = Html::parse_document(essay_content.as_str());
    let body_selector = Selector::parse("body").unwrap();
    let body_element = content_fragment.select(&body_selector).next().unwrap();
    let inner_html: String = body_element.inner_html();//.collect::<Vec<_>>().join("\n\n");
    let replacer = Regex::new(SCRIPT_REGEX).unwrap();
    let replaced_body = replacer.replace_all(&inner_html, "");
    let body_fragment = Html::parse_fragment(&replaced_body);
    let body_text = body_fragment.root_element()
        .descendants()
        .filter_map(|node| {
            if let Some(text) = node.value().as_text() {
                Some(text.text.trim())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let regex = Regex::new(SENTENCE_REGEX).unwrap();
    let captures: Vec<&str> = regex
        .find_iter(&body_text)
        .map(|m| m.as_str())
        .collect();
    let random_index = rand::random::<usize>() % captures.len();
    captures[random_index].replace("\n", " ").to_string()
}

#[poise::command(slash_command)]
pub async fn pgsays(
    ctx: Context<'_>,
    #[description = "pg says what"] _about: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let random_essay = find_random_essay().await?;
    let random_essay_content = get_essay_content(random_essay).await?;
    let random_sentence = get_random_sentence(random_essay_content);
    ctx.say(random_sentence).await?;
    Ok(())
}
