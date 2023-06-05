use boxcars::{ParseError, Replay};
use std::error;
use std::fs;
use std::path::Path;

fn parse_rl(data: &[u8]) -> Result<Replay, ParseError> {
    boxcars::ParserBuilder::new(data)
        .must_parse_network_data()
        .parse()
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let replay_dir = "./raw_replays";
    let json_dir = "./parsed_replays";

    // Créer le dossier ./json s'il n'existe pas
    fs::create_dir_all(json_dir)?;

    // Lire les fichiers .replay du dossier ./replay
    let replay_files = fs::read_dir(replay_dir)?;
    for replay_file in replay_files {
        let replay_file = replay_file?;
        let replay_path = replay_file.path();
        if let Some(extension) = replay_path.extension() {
            if extension == "replay" {
                let replay_filename = replay_path.file_name().unwrap().to_string_lossy();
                let json_filename = format!("{}.json", replay_filename);

                let json_path = Path::new(json_dir).join(&json_filename);

                // Vérifier si le fichier JSON existe déjà
                if json_path.exists() {
                    println!("Le fichier {} existe déjà, il sera ignoré.", json_filename);
                    continue;
                }

                let replay_buffer = fs::read(&replay_path)?;
                let replay = parse_rl(&replay_buffer)?;

                let mut json_file = fs::File::create(json_path)?;
                serde_json::to_writer(&mut json_file, &replay)?;
            }
        }
    }

    Ok(())
}

fn main() { 
    let _ = run();
    println!("Parsing fini !");
}