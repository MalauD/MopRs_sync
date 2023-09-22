use crate::music_struct::{
    Album, AlbumMeilisearch, Artist, ArtistMeilisearch, Music, MusicMeilisearch,
};
use futures::StreamExt;
use mongodb::options::ClientOptions;
// This app sync music documents from MongoDB to MeiliSearch

mod music_struct;

async fn sync_music(db: mongodb::Database, client_search: meilisearch_sdk::Client) {
    // Get the database and collection
    let collection = db.collection::<Music>("Music");

    let total_documents = collection.count_documents(None, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut batch = Vec::new();

    let mut doc_indexed: u64 = 0;

    batch.reserve(1000);
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                batch.push(document);
                if batch.len() == 1000 {
                    // Send the batch to MeiliSearch
                    let batch_meilisearch: Vec<MusicMeilisearch> = batch
                        .iter()
                        .map(|music| MusicMeilisearch::from(music.clone()))
                        .collect();
                    client_search
                        .index("musics")
                        .add_documents(&batch_meilisearch, None)
                        .await
                        .unwrap();
                    batch.clear();
                    doc_indexed += 1000;
                    println!("Progress: {}/{}", doc_indexed, total_documents);
                }
            }
            Err(e) => panic!("Failed to get document: {}", e),
        }
    }
}

async fn sync_album(db: mongodb::Database, client_search: meilisearch_sdk::Client) {
    let collection = db.collection::<Album>("Album");

    let total_documents = collection.count_documents(None, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut batch = Vec::new();

    let mut doc_indexed: u64 = 0;

    batch.reserve(1000);

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                batch.push(document);
                if batch.len() == 1000 {
                    // Send the batch to MeiliSearch
                    let batch_meilisearch: Vec<AlbumMeilisearch> = batch
                        .iter()
                        .map(|album| AlbumMeilisearch::from(album.clone()))
                        .collect();
                    client_search
                        .index("albums")
                        .add_documents(&batch_meilisearch, None)
                        .await
                        .unwrap();
                    batch.clear();
                    doc_indexed += 1000;
                    println!("Progress: {}/{}", doc_indexed, total_documents);
                }
            }
            Err(e) => panic!("Failed to get document: {}", e),
        }
    }
}

async fn sync_artist(client_db: mongodb::Database, client_search: meilisearch_sdk::Client) {
    let collection = client_db.collection::<Artist>("Artist");

    let total_documents = collection.count_documents(None, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut batch = Vec::new();

    let mut doc_indexed: u64 = 0;

    batch.reserve(1000);

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                batch.push(document);
                if batch.len() == 1000 {
                    // Send the batch to MeiliSearch
                    let batch_meilisearch: Vec<ArtistMeilisearch> = batch
                        .iter()
                        .map(|artist| ArtistMeilisearch::from(artist.clone()))
                        .collect();
                    client_search
                        .index("artists")
                        .add_documents(&batch_meilisearch, None)
                        .await
                        .unwrap();
                    batch.clear();
                    doc_indexed += 1000;
                    println!("Progress: {}/{}", doc_indexed, total_documents);
                }
            }
            Err(e) => panic!("Failed to get document: {}", e),
        }
    }
}
#[tokio::main]
async fn main() {
    let MEILISEARCH_URL = "http://".to_owned() + std::env::var("MEILISEARCH_URL").unwrap().as_str();
    let MEILISEARCH_API_KEY = std::env::var("MEILISEARCH_API_KEY").unwrap();
    let MONGODB_URL = std::env::var("MONGODB_URL").unwrap();

    // Create a client (without sending any request so that can't fail)
    let client_search = meilisearch_sdk::Client::new(MEILISEARCH_URL, MEILISEARCH_API_KEY);

    let mut client_options = ClientOptions::parse(MONGODB_URL).await.unwrap();
    client_options.app_name = Some("MopRs_sync".to_string());
    let client_db = mongodb::Client::with_options(client_options).unwrap();
    let db = client_db.database("MopRs");

    println!("Starting music sync");
    sync_music(db.clone(), client_search.clone()).await;
    println!("Music sync finished");

    println!("Starting album sync");
    sync_album(db.clone(), client_search.clone()).await;
    println!("Album sync finished");

    println!("Starting artist sync");
    sync_artist(db.clone(), client_search.clone()).await;
    println!("Artist sync finished");
}
