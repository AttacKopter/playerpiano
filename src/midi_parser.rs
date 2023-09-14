use std::fs;
use std::i64;

struct Song {
    format: i64,
    num_tracks: i64,
    time: i64,
    time_type: i64,
    tracks: Vec<Track>
}

struct Track {
    name: String,
    instrument: String,
    channel: i64,
    tempo: i64,
    events: Vec<Event>
}

struct Event {
    time: i64,
    action: String,
    channel: i64,
    key: i64,
    velocity: i64
}

fn old_main() {

    let song = parse_midi_file("Fur Elise.mid");

    println!("Format: {}", song.format);
    println!("Number Of Tracks: {}", song.format);
    println!("Time Type: {}", song.format);
    println!("Time: {}", song.format);

    for track in song.tracks {
        println!();
        println!();
        println!();
        println!();
        println!("Track Name: {}", track.name);
        println!("Track Instrument: {}", track.instrument);
        println!("Track Channel: {}", track.channel);
        println!("Track Tempo: {}", track.tempo);
        println!();
        for event in track.events {
            println!("At: {}, {}, Channel: {}, Key: {}, Velocity: {}", event.time, event.action, event.channel, event.key, event.velocity)
        }
    }
    
}

fn parse_midi_file(file_path: &str) -> Song {
    let mut song = Song { format: 0, num_tracks: 0, time: 0, time_type: 0, tracks: Vec::new() };

    let hex = get_hex(file_path);
    let chunks = get_chunks(hex);

    for chunk in chunks {
        if chunk[0] == "68" && chunk[1] == "64" { // Header Chunk
            song = parse_header(chunk, song)
        } else if chunk[0] == "72" && chunk[1] == "6B" { // Track Chunk
            song.tracks.push(parse_track(chunk))
        }
    }

    song
}

fn get_hex(file_path: &str) -> Vec<String> {
    let midi = fs::read(file_path).unwrap();

    let hex: Vec<String> = midi
    .iter()
    .map(
        |f| format!("{f:X}")
    ).map(
        |f| if f.len() == 2 {
            f
        } else {
            format!("0{}",f)
        }
    )
    .collect();
    hex
}



fn get_chunks(hex: Vec<String>) -> Vec<Vec<String>> {
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut start_index = 0;
    
    for (index, element) in hex.iter().enumerate() {
        if element == "4D" && hex.get(index+1).unwrap() == &"54".to_string() {
            chunks.push(hex[start_index..index].to_vec());
            start_index = index+2;
        }
    }
    chunks[1..].to_vec()
}



fn parse_header(chunk: Vec<String>, mut song: Song) -> Song {
    let Ok(format) = i64::from_str_radix(&format!("{}{}", chunk[6], chunk[7]), 16) else {panic!()};
    let Ok(tracks) = i64::from_str_radix(&format!("{}{}", chunk[8], chunk[9]), 16) else {panic!()};
    let Ok(mut time) = i64::from_str_radix(&format!("{}{}", chunk[10], chunk[11]), 16) else {panic!()};
    let mut time_type = 0;
    if time > 16383 {
        time_type = 1;
        time -= 16384;
    }

    song.format = format;
    song.num_tracks = tracks;
    song.time = time;
    song.time_type = time_type;
    
    song
}



fn parse_track(oversized_chunk: Vec<String>) -> Track {
    let chunk = oversized_chunk[6..].to_vec();
    
    
    let mut track = Track {name: "".to_string(), instrument: "".to_string(), channel: 0, tempo: 0, events: Vec::new()};

    let mut start_index = 0;

    let mut skip = 0;
    let mut time_finished = false;
    
    for (index, element) in chunk.iter().enumerate() {
        
        if skip != 0 {
            skip -= 1;
            if skip != 0 {
                continue;
            }
            time_finished = false;
            track = parse_event(chunk[start_index..=index].to_vec(), track);
            start_index = index + 1;
            continue;
        }

        let Ok(decimal) = i64::from_str_radix(element, 16) else {panic!()};
        if !time_finished {
            time_finished = decimal < 128;
            continue;
        }

        //print!("test");

        if element.starts_with("8") {
            skip = 2;
        } else if element.starts_with("9") {
            skip = 2;
        } else if element.starts_with("A") {
            skip = 2;
        } else if element.starts_with("B") {
            skip = 2;
        } else if element.starts_with("C") {
            skip = 1;
        } else if element.starts_with("D") {
            skip = 1;
        } else if element.starts_with("E") {
            skip = 2;
        } else if element == "F2" {
            skip = 2;
        } else if element == "F3" {
            skip = 1;
        } else if element == "F6" {
            time_finished = false;
            track = parse_event(chunk[start_index..=index].to_vec(), track);
            start_index = index + 1;
            continue;
        } else if element == "FF" {
            let next = chunk.get(index+1).unwrap();
            if next == "00" {
                skip = 2
            } else if next.starts_with("0") || next == "7F"{
                let Ok(length) = i64::from_str_radix(chunk.get(index + 2).unwrap(), 16) else {panic!()};
                skip = 2 + length
            } else if next == "51" {
                skip = 5
            } else if next == "54" {
                skip = 7
            } else if next == "58" {
                skip = 6
            } else if next == "59" {
                skip = 4
            } else if next == "21" {
                skip = 3
            } else if next == "20" {
                skip = 3
            } else if next == "2F" {
                break;
            } else {
                println!("{}", next)
            }
        } else {
            println!("{:?}", chunk[start_index..index].to_vec());
            println!("{}", element);
            println!();
        }


    }

    track
}

fn parse_event(event_hex: Vec<String>, mut track: Track) -> Track {

    let mut event = Event {time: 0, action: "".to_string(), channel: 0, key: 0, velocity: 0};
    let mut time: i64 = 0;
    let mut time_finished = false;

    for (index, element) in event_hex.iter().enumerate() {

        let Ok(decimal) = i64::from_str_radix(element, 16) else {panic!()};
        
        if !time_finished {
            let Ok(hex_val) = i64::from_str_radix(element, 16) else {panic!()};
            

            time_finished = decimal < 128;
            if time_finished {
                time = time * 128 + hex_val;
                event.time = time;
            } else {
                time = time * 128 + hex_val - 128;
            }
            continue;
        }

        
            

        if element.starts_with("8") {
            let Ok(channel) = i64::from_str_radix(&element.chars().last().unwrap().to_string(), 16) else {panic!()};
            let Ok(key) = i64::from_str_radix(event_hex.get(index+1).unwrap(), 16) else {panic!()};
            let Ok(velocity) = i64::from_str_radix(event_hex.get(index+2).unwrap(), 16) else {panic!()};
          
            event.channel = channel;
            event.key = key;
            event.velocity = velocity;
            event.action = "Note Off".to_string();
            track.events.push(event);
            break;
        } else if element.starts_with("9") {
            let Ok(channel) = i64::from_str_radix(&element.chars().last().unwrap().to_string(), 16) else {panic!()};
            let Ok(key) = i64::from_str_radix(event_hex.get(index+1).unwrap(), 16) else {panic!()};
            let Ok(velocity) = i64::from_str_radix(event_hex.get(index+2).unwrap(), 16) else {panic!()};

            event.channel = channel;
            event.key = key;
            event.velocity = velocity;
            event.action = "Note On".to_string();
            track.events.push(event);
            break;
        } else if element.starts_with("A") {
            let Ok(channel) = i64::from_str_radix(&element.chars().last().unwrap().to_string(), 16) else {panic!()};
            let Ok(key) = i64::from_str_radix(event_hex.get(index+1).unwrap(), 16) else {panic!()};
            let Ok(velocity) = i64::from_str_radix(event_hex.get(index+2).unwrap(), 16) else {panic!()};

            event.channel = channel;
            event.key = key;
            event.velocity = velocity;
            event.action = "Aftertouch".to_string();
            track.events.push(event);
            break;
        } else if element.starts_with("B") {
            let Ok(channel) = i64::from_str_radix(&element.chars().last().unwrap().to_string(), 16) else {panic!()};
            let Ok(key) = i64::from_str_radix(event_hex.get(index+1).unwrap(), 16) else {panic!()};
            let Ok(velocity) = i64::from_str_radix(event_hex.get(index+2).unwrap(), 16) else {panic!()};

            event.channel = channel;
            event.key = key;
            event.velocity = velocity;
            event.action = "Control Change".to_string();
            track.events.push(event);
            break;
        } else if element == "FF" {
            let next = event_hex.get(index+1).unwrap();

            if next == "03" {
                track.name = event_hex[index+2..].join("");
                break;
            } else if next == "04" {
                track.instrument = event_hex[index+2..].join("");
                break;
            } else if next == "20" {
                let Ok(channel) = i64::from_str_radix(event_hex.get(index+3).unwrap(), 16) else {panic!()};
                track.channel = channel;
                break;
            } else if next == "51" {
                let Ok(tempo) = i64::from_str_radix(event_hex.get(index+3).unwrap(), 16) else {panic!()};
                track.tempo = tempo;
                break;
            }
        }
        break;
    }


    track
}