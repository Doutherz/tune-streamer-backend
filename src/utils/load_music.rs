use std::fs;
use std::path::Path;
use lofty::TaggedFileExt;
use lofty::{AudioFile, ItemKey};
use crate::models::music_model::Music;
use crate::services::music_service::{add_song, if_music_path_exists, get_all_songs, remove_song};
use crate::MUSICDATA_URL;


pub async fn load_music() -> Result<(), Box<dyn std::error::Error>>{
    let path = *MUSICDATA_URL;
    
    for entry in fs::read_dir(path)? {
        //check if song is already in db
        let entry = entry?;

        if let Some(path) =  entry.path().to_str(){
            // skip song if its already added
            if if_music_path_exists(path).await? {
                println!("{} (Already exists)", path);
                continue;
            }
            
            let music = get_music_properties(path)?;
            add_song(music).await?;
            println!("{} (Added)", path);
        } else {
            return Err("Invalid sequence in file path".into());
        }
    }

    // get all songs from db and check if paths exists if not remove song from db
    let music_db = get_all_songs().await?;
    for song in music_db {
        if !Path::new(&song.song_path).exists() {
            println!("{} (Removed)", song.song_path);
            remove_song(song.id).await?;
        }
    }

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