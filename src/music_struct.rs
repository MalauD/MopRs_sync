use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

pub type DeezerId = i64;

#[derive(Deserialize, Serialize, Clone)]
pub struct Music {
    #[serde(rename = "_id")]
    pub id: DeezerId,
    pub title: String,
    pub artist_name: String,
    pub published_date: DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub views: i64,
    pub likes: i64,
    pub rank: i64,
    pub last_view: DateTime,
}

#[derive(Serialize, Deserialize)]
pub struct MusicMeilisearch {
    pub id: DeezerId,
    pub title: String,
    pub artist_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub rank: i64,
}

impl From<Music> for MusicMeilisearch {
    fn from(music: Music) -> Self {
        MusicMeilisearch {
            id: music.id,
            title: music.title,
            artist_name: music.artist_name,
            image_url: music.image_url,
            rank: music.rank,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Album {
    #[serde(rename = "_id")]
    pub id: DeezerId,
    pub name: String,
    pub cover: String,
    is_complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musics: Option<Vec<DeezerId>>,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumMeilisearch {
    pub id: DeezerId,
    pub name: String,
    pub cover: String,
}

impl From<Album> for AlbumMeilisearch {
    fn from(album: Album) -> Self {
        AlbumMeilisearch {
            id: album.id,
            name: album.name,
            cover: album.cover,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Artist {
    #[serde(rename = "_id")]
    pub id: DeezerId,
    pub name: String,
    pub picture: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<DeezerId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_tracks: Option<Vec<DeezerId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_artists: Option<Vec<DeezerId>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub last_update: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistMeilisearch {
    pub id: DeezerId,
    pub name: String,
    pub picture: String,
}

impl From<Artist> for ArtistMeilisearch {
    fn from(artist: Artist) -> Self {
        ArtistMeilisearch {
            id: artist.id,
            name: artist.name,
            picture: artist.picture,
        }
    }
}
