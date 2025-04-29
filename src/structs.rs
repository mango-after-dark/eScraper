use chrono::{DateTime, Utc};
use rs621::post::{Post, PostFile, PostFileExtension, PostRating, PostRelationships, PostScore, PostTags};
use serde::{ Serialize };
use std::fmt;

#[derive(Serialize, Debug)]
pub struct Metadata { 
    pub start_time: DateTime<Utc>,
    pub finish_time: DateTime<Utc>,
    pub start_id: u64,  // inclusive
    pub end_id: u64,   // exclusive 
    pub posts: Vec<ScrapedPost>,
}

#[derive(Serialize, Debug)]
pub struct ScrapedPost {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub file: ScrapedPostFile,
    pub score: ScrapedPostScore,
    pub tags: ScrapedPostTags,
    pub locked_tags: Vec<String>,
    pub rating: ScrapedPostRating,
    pub fav_count: u64,
    pub sources: Vec<String>,
    pub pools: Vec<u64>,
    pub relationships: ScrapedPostRelationships,
    pub approver_id: Option<u64>,
    pub uploader_id: u64,
    pub description: String,
    pub comment_count: u64
}

impl From<Post> for ScrapedPost {
    fn from(original: Post) -> Self {
        let file: ScrapedPostFile = original.file.into();
        let score: ScrapedPostScore = original.score.into();
        let tags: ScrapedPostTags = original.tags.into();
        let rating: ScrapedPostRating = original.rating.into();
        let relationships: ScrapedPostRelationships = original.relationships.into();

        Self {
            id: original.id,
            created_at: original.created_at,
            updated_at: original.updated_at,
            file,
            score,
            tags,
            locked_tags: original.locked_tags,
            rating,
            fav_count: original.fav_count,
            sources: original.sources,
            pools: original.pools,
            relationships,
            approver_id: original.approver_id,
            uploader_id: original.uploader_id,
            description: original.description,
            comment_count: original.comment_count,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ScrapedPostFile {
    pub width: u64,
    pub height: u64,
    pub extension: ScrapedPostFileExtension,
    pub size: u64,
    pub md5: String,
    pub url: Option<String>
}

impl From<PostFile> for ScrapedPostFile {
    fn from(original: PostFile) -> Self {
        let ext: ScrapedPostFileExtension = original.ext.into();

        Self {
            width: original.width,
            height: original.height,
            extension: ext,
            size: original.size,
            md5: original.md5,
            url: original.url,
        }
    }
}

#[derive(Serialize, Debug)]
pub enum ScrapedPostFileExtension {
    #[serde(rename = "jpg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "swf")]
    Swf,
    #[serde(rename = "webm")]
    Webm,
}

impl From<PostFileExtension> for ScrapedPostFileExtension {
    fn from(original: PostFileExtension) -> Self {
        match original {
            PostFileExtension::Jpeg => ScrapedPostFileExtension::Jpeg,
            PostFileExtension::Png => ScrapedPostFileExtension::Png,
            PostFileExtension::Gif => ScrapedPostFileExtension::Gif,
            PostFileExtension::Swf => ScrapedPostFileExtension::Swf,
            PostFileExtension::WebM => ScrapedPostFileExtension::Webm,
        }
    }
}

impl fmt::Display for ScrapedPostFileExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let extension = match self {
            ScrapedPostFileExtension::Jpeg => "jpg",
            ScrapedPostFileExtension::Png => "png",
            ScrapedPostFileExtension::Gif => "gif",
            ScrapedPostFileExtension::Swf => "swf",
            ScrapedPostFileExtension::Webm => "webm",
        };
        write!(f, "{}", extension)
    }
}

#[derive(Serialize, Debug)]
pub struct ScrapedPostScore {
    pub up: i64,
    pub down: i64,
    pub total: i64,
}

impl From<PostScore> for ScrapedPostScore {
    fn from(original: PostScore) -> Self {
        Self {
            up: original.up,
            down: original.down,
            total: original.total,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ScrapedPostTags {
    pub general: Vec<String>,
    pub species: Vec<String>,
    pub character: Vec<String>,
    pub artist: Vec<String>,
    pub invalid: Vec<String>,
    pub lore: Vec<String>,
    pub meta: Vec<String>,
}

impl From<PostTags> for ScrapedPostTags {
    fn from(original: PostTags) -> Self {
        Self {
            general: original.general,
            species: original.species,
            character: original.character,
            artist: original.artist,
            invalid: original.invalid,
            lore: original.lore,
            meta: original.meta,
        }
    }
}

#[derive(Serialize, Debug)]
pub enum ScrapedPostRating {
    #[serde(rename = "s")]
    Safe,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "e")]
    Explicit,
}

impl From<PostRating> for ScrapedPostRating {
    fn from(original: PostRating) -> Self {
        match original {
            PostRating::Safe => ScrapedPostRating::Safe,
            PostRating::Questionable => ScrapedPostRating::Questionable,
            PostRating::Explicit => ScrapedPostRating::Explicit,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ScrapedPostRelationships {
    pub parent_id: Option<u64>,
    pub has_children: bool,
    pub children: Vec<u64>,
}

impl From<PostRelationships> for ScrapedPostRelationships {
    fn from(original: PostRelationships) -> Self {
        Self {
            parent_id: original.parent_id,
            has_children: original.has_children,
            children: original.children,
        }
    }
}
