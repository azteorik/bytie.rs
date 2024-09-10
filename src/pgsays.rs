use crate::context::{Context, Error};
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;
use rand::prelude::*;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use thiserror::Error;

const BASE_URL: &str = "https://paulgraham.com";
const SCRIPT_REGEX: &str = r"<script[^>]*>(\n?(.*?)\n?)+<\/script>";
const SENTENCE_REGEX: &str = r"(?s)(?:[^.!?])+[.!?]+\s*";
static SCRIPT_REGEX_COMPILED: Lazy<Regex> = Lazy::new(|| Regex::new(SCRIPT_REGEX).unwrap());
static SENTENCE_REGEX_COMPILED: Lazy<Regex> = Lazy::new(|| Regex::new(SENTENCE_REGEX).unwrap());

#[derive(Debug, Clone)]
struct PGEssay {
    link: String,
    title: String,
    content: Option<String>,
}

#[derive(Error, Debug)]
enum EssayError {
    #[error("Failed to fetch data: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("Failed to parse HTML: {0}")]
    ParseError(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("No essays found")]
    NoEssaysFound,
    #[error("No sentences found in the essay")]
    NoSentencesFound,
    #[error("Failed to compile regex: {0}")]
    RegexError(#[from] regex::Error),
}

async fn find_random_essay() -> Result<PGEssay, EssayError> {
    let essays_url = format!("{}/articles.html", BASE_URL);
    let essays_response = reqwest::get(&essays_url).await?.text().await?;
    let html_document = Html::parse_document(&essays_response);
    let selector = Selector::parse("td a").map_err(EssayError::ParseError)?;
    let essay_links: Vec<_> = html_document.select(&selector)
        .filter_map(|element| {
            let link = element.attr("href")?;
            let title = element.text().next()?.trim().to_string();
            Some((link.to_string(), title))
        })
        .collect();

    essay_links.choose(&mut thread_rng())
        .map(|(link, title)| PGEssay { link: format!("{}/{}", BASE_URL, link.clone()), title: title.clone(), content: None })
        .ok_or(EssayError::NoEssaysFound)
}

async fn get_essay_content(essay: &PGEssay) -> Result<PGEssay, EssayError> {
    let content = reqwest::get(&essay.link).await?.text().await?;
    Ok(PGEssay {
        link: essay.link.clone(),
        title: essay.title.clone(),
        content: Some(content),
    })
}

fn get_random_sentence(essay: &PGEssay) -> Result<String, EssayError> {
    let content = essay.content.as_ref().ok_or(EssayError::NoSentencesFound)?;
    let content_without_scripts = SCRIPT_REGEX_COMPILED.replace_all(content, "");
    let content_fragment = Html::parse_document(&content_without_scripts);
    let body_selector = Selector::parse("body").map_err(EssayError::ParseError)?;
    let body_text: String = content_fragment
        .select(&body_selector)
        .flat_map(|element| element.text())
        .collect();

    let sentences: Vec<&str> = SENTENCE_REGEX_COMPILED.find_iter(&body_text)
        .map(|m| m.as_str())
        .collect();

    sentences.choose(&mut thread_rng())
        .map(|&sentence| sentence.replace('\n', " "))
        .ok_or(EssayError::NoSentencesFound)
}

#[poise::command(slash_command)]
pub async fn pgsays(
    ctx: Context<'_>,
    #[description = "pg says what"] _about: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let random_essay = find_random_essay().await.map_err(|e| Error::from(e.to_string()))?;
    let random_essay_content = get_essay_content(&random_essay).await.map_err(|e| Error::from(e.to_string()))?;
    let random_sentence = get_random_sentence(&random_essay_content).map_err(|e| Error::from(e.to_string()))?;

    let reply = poise::CreateReply::default().content(String::new()).embed(
        serenity::CreateEmbed::new()
            .title(random_essay.title)
            .url(random_essay.link)
            .description(&random_sentence)
            .author(serenity::CreateEmbedAuthor::new("Paul Graham").url("https://paulgraham.com"))
    );

    ctx.send(reply).await?;

    Ok(())
}
