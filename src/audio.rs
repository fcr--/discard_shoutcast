use std::time::{Duration, Instant};
use serde_json;

pub struct Metadata<'a> {
    name: &'a str,
    artist: &'a str,
}

pub trait Source {
    fn uri<'a>(&'a mut self) -> Result<&'a str, String>;
    fn metadata<'a>(&'a mut self) -> Result<Metadata<'a>, String>;
}

struct YoutubeSource {
    id: String,
    title: String,
    uri: String,
    fetch_time: Option<Instant>,
}

pub fn open_youtube_stream(id: &str) -> Box<Source> {
    Box::new(YoutubeSource { id: id.to_string(), title: "".to_string(), uri: "".to_string(), fetch_time: None })
}

impl YoutubeSource {
    fn fetch(&mut self) -> Result<(), String> {
        use std::process::{Command, Stdio};
        let output = try!(Command::new("youtube-dl")
            .args(&[
                "-f", "webm[abr>0]/bestaudio/best",
                "--no-playlist", "--print-json",
                "--skip-download",
                &self.id])
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("running youtube-dl: {}", e)));
        if !output.status.success() {
            return Err(format!("youtube-dl: {:?}", output));
        }

        let json: serde_json::Value = try!(serde_json::from_reader(&output.stdout[..])
            .map_err(|e| format!("parsing json: {}", e)));
        let map = match json.as_object() {
            Some(map) => map,
            None => return Err("youtube-dl output could not be read".to_string())
        };
        self.title = map.get("title").and_then(serde_json::Value::as_str).unwrap_or("").to_string();
        self.uri = match map.get("url").and_then(serde_json::Value::as_str) {
            Some(url) => url.to_string(),
            None => return Err("youtube-dl output's \"url\" could not be read".to_string())
        };
        self.fetch_time = Some(Instant::now());
        Ok(())
    }
}

impl Source for YoutubeSource {
    fn uri<'a>(&'a mut self) -> Result<&'a str, String> {
        let res = match self.fetch_time.map(|i| i.elapsed() < Duration::new(30, 0)) {
            Some(true) => self.fetch(),
            _ => Ok(())
        };
        let uri : &str = &self.uri;
        res.map(|_| uri)
    }

    fn metadata<'a>(&'a mut self) -> Result<Metadata<'a>, String> {
        let res = match self.fetch_time {
            None => self.fetch(),
            _ => Ok(())
        };
        let title = &self.title;
        res.map(|_| Metadata { name: title, artist: "" })
    }
}
