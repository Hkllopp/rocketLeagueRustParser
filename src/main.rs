use boxcars::{ParseError, Replay};
use std::error;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use config::Config;

fn parse_rl(data: &[u8]) -> Result<Replay, ParseError> {
    boxcars::ParserBuilder::new(data)
        .must_parse_network_data()
        .parse()
}

fn run(config_str : HashMap<String, String>) -> Result<(), Box<dyn error::Error>> {
    let raw_replays_folder = config_str.get("raw_replays_folder").unwrap();
    let parsed_replays_folder = config_str.get("parsed_replays_folder").unwrap();
    let replays_extension = config_str.get("replays_extension").unwrap();

    // Create parsed_replays_folder folder if it doesn't exist
    fs::create_dir_all(parsed_replays_folder)?;

    // Read the replays_extension files from the raw_replays_folder folder
    let replay_files = fs::read_dir(raw_replays_folder)?;
    for replay_file in replay_files {
        let replay_file = replay_file?;
        let replay_path = replay_file.path();
        if let Some(extension) = replay_path.extension() {
            if extension.to_str() == Some(replays_extension) {
                let replay_filename = replay_path.file_name().unwrap().to_string_lossy();
                let json_filename = format!("{}.json", replay_filename);

                let json_path = Path::new(parsed_replays_folder).join(&json_filename);

                // Check if the json parsed file already exists
                if json_path.exists() {
                    println!("File {} already parsed, will be ignored", json_filename);
                    continue;
                }

                let replay_buffer = fs::read(&replay_path)?;

                // Attempt to parse the replay
                match parse_rl(&replay_buffer) {
                    Ok(replay) => {
                        let mut json_file = fs::File::create(json_path)?;
                        serde_json::to_writer(&mut json_file, &replay)?;
                    }
                    Err(error) => {
                        println!("Parsing failed for file {}: {}", replay_filename, error);
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}


fn load_config() -> HashMap<String, String>{
    let settings = Config::builder()
    .add_source(config::File::with_name("config"))
    .build()
    .unwrap();

    settings.try_deserialize::<HashMap<String, String>>().unwrap()
}

fn main(){
    let config_str =load_config();

    let _ = run(config_str);
    println!("Parsing done !");
}