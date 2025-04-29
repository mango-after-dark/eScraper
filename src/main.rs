mod structs;
use structs::{ ScrapedPost, Metadata };

use chrono::{ Utc };

use rs621::client::Client;

use reqwest;
use tokio;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use std::fs;
use std::path::Path;
use std::io::Write;

use anyhow::Result;

use tracing::{ info, warn, error };
use tracing_subscriber;

use clap::Parser;

use futures::prelude::*;

/// A program to scrape e621
#[derive(Parser, Debug)]
struct Args {
    /// The amount of chunks to scrape
    #[clap(long, default_value_t = 1)]
    chunk_count: u64,
    /// The amount of posts the program tries to scrape per chunk. Note that the actual count may be lower if posts fail to load
    #[clap(long, default_value_t = 10)]
    chunk_size: u64,
}

async fn download_file(http: &reqwest::Client, url: String, file_path: String) -> Result<()> {
    info!("Downloading {url} as {file_path}");

    let mut file = File::create_new(Path::new(&file_path)).await?;

    let mut response = http.get(&url).send().await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let client = Client::new("https://e621.net", "eScaper/1.0 (by MrNoahMango on Discord)").expect("Error creating client");
    let reqwest = reqwest::Client::new();

    let mut current_id = client.post_search(&[""][..])
        .take(1)
        .next()
        .await
        .expect("Uhh... I'm gonna' be honest, I have no clue what happened. Please open an issue.")
        .expect("Failed to get latest post, check your internet connection")
        .id;

    if !fs::exists("chunks").expect("Error checking chunks folder") {
        fs::create_dir("chunks").expect("Error creating chunks folder");
    }

    for _ in 0..args.chunk_count {
        let start_time = Utc::now();
        
        let start_id = current_id;
        let end_id = current_id - args.chunk_size;
        
        let folder_name = format!(r"chunks\chunk_{}_{start_id}-{end_id}", start_time.format("%Y;%m;%d-%H;%M;%S"));
        
        
        match fs::exists(&folder_name) {
            Err(why) => {
                warn!("Error while checking chunk folder {}, skipping chunk.\n{:?}", &folder_name, why);
                continue
            }
            Ok(exists) => {
                if !exists {
                    match fs::create_dir(&folder_name) {
                        Ok(_) => {}
                        Err(why) => {
                            warn!("Failed to create chunk folder, skipping chunk. {}\n{:?}", &folder_name, why);
                            continue
                        }
                    }
                }
            }
        }

        info!("Fetching posts {start_id}-{end_id}");
        let posts: Vec<ScrapedPost> = client
            .post_search(&[format!("id:<={}", current_id.to_string())][..])
            .take(args.chunk_size as usize)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(Result::ok)
            .map(|post| post.into())
            .collect::<Vec<_>>();

        for post in &posts {
            info!("Processing post {}", post.id);

            let Some(ref url) = post.file.url else {
                warn!("Post {} had no file to download, including it in the metadata anyway", post.id);
                continue;
            };

            let file_name = format!(r"{}\{}.{}", &folder_name, post.id, post.file.extension);

            match download_file(&reqwest, url.to_string(), file_name).await {
                Ok(_) => {}
                Err(why) => {error!("Error downloading post {}: {:?}", post.id, why);}
            }
        }

        current_id -= args.chunk_size;
        
        let finish_time = Utc::now();

        let meta = Metadata {
            start_time,
            start_id,
            finish_time,
            end_id,
            posts
        };

        let meta_json = match serde_json::to_string_pretty(&meta) {
            Ok(json) => json,
            Err(why) => {error!("Error serializing metadata: {:?}", why); continue}
        };

        let mut meta_file = match fs::File::create(format!(r"{}\meta.json", folder_name)) {
            Ok(file) => file,
            Err(why) => {error!("Error creating metadata file: {:?}", why); continue}
        };
        
        match meta_file.write_all(meta_json.as_bytes()) {
            Ok(_) => {}
            Err(why) => {error!("Error writing metadata file: {:?}", why); continue}
        }
    }
}
