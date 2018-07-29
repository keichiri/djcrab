use std::error::Error;

use vlc;


#[derive(Debug)]
pub(super) enum PlayerError {
    MediaError(String),
}


pub(super) struct Player {
    instance: Box<vlc::Instance>,
    media_player: Box<vlc::MediaPlayer>,
    media: Option<vlc::Media>,
}


impl Player {
    pub(super) fn new() -> Option<Self> {
        let instance = match vlc::Instance::new() {
            None => return None,
            Some(instance) => Box::new(instance),
        };
        let media_player = match vlc::MediaPlayer::new(&instance) {
            None => return None,
            Some(media_player) => Box::new(media_player),
        };

        Some(Self {
            instance: instance,
            media_player: media_player,
            media: None,
        })
    }

    pub(super) fn play(&mut self, path: &str) -> Result<(), PlayerError> {
        println!("Playing!");
        match vlc::Media::new_path(&self.instance, path) {
            None => return Err(_media_error()),
            Some(new_media) => self.media = Some(new_media)
        };

        self.media_player.set_media(&self.media.as_ref().unwrap());

        match self.media_player.play() {
            Err(_) => Err(_media_error()),
            Ok(()) => Ok(()),
        }
    }

    pub(super) fn pause(&self) {
        self.media_player.set_pause(true);
    }

    pub(super) fn unpause(&self) {
        self.media_player.set_pause(false);
    }
}


fn _media_error() -> PlayerError {
    let err_msg = vlc::errmsg().unwrap_or(String::new());
    PlayerError::MediaError(err_msg)
}