use std::fs;
use lofty::TaggedFileExt;
use lofty::{AudioFile, ItemKey};
use crate::models::music_model::Music;
use crate::services::music_service::add_song;
use crate::services::music_service::remove_all_songs;

//!! need to update playlists if songs have been removed !!
pub async fn load_music() -> Result<(), Box<dyn std::error::Error>>{
    let path = "./Tune-Streamer_music";

    match remove_all_songs().await {
        Ok(_) => {},
        Err(e) => return Err(e.into())
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(path) =  entry.path().to_str(){
            println!("{}", path);
            let music = get_music_properties(path)?;
            //!! check if title already exists !!
            add_song(music).await?;
        } else {
            return Err("Invalid sequence in file path".into());
        }
    }

    //update playlists to remove anysongs that have not been reimported, 
    //song ids might be change so every song is in a playlists will have to be check 

    Ok(())
}

fn get_music_properties(file_path: &str) -> Result<Music, Box<dyn std::error::Error>>{
    let tagged_file = lofty::read_from_path(file_path)?;
    if let Some(tag) = tagged_file.primary_tag() {
        Ok(Music {
            id: 0,
            song_path: file_path.to_string(),
            title: tag.get_string(&ItemKey::TrackTitle).map(|s| s.to_string()),
            artist: tag.get_string(&ItemKey::TrackArtist).map(|s| s.to_string()),
            genre: tag.get_string(&ItemKey::Genre).map(|s| s.to_string()),
            duration: Some(tagged_file.properties().duration().as_secs()),
        })
    } else {
        Err("Primary tag not found".into())
    }
}