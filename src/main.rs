use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{Write,BufReader};
use strsim::levenshtein;
use std::process::Command;
use std::path::Path;

#[derive(Deserialize)]
struct GamesResponse {
    response: Response,
}

#[derive(Deserialize)]
struct Response {
    games: Vec<Game>,
}

#[derive(Deserialize, Serialize)]
struct Game {
    appid: u32,
    name: String,
}

//save games to a JSON file
fn save_games_to_json(games: &Vec<Game>, file_name: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(games)?;
    let mut file = File::create(file_name)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

fn refresh_game_list(){
    let api_key = "YOUR_STEAM_API_KEY"; // Replace with your Steam API key
    let steam_id = "YOUR_STEAM_ID"; // Replace with your Steam ID
    let url = format!(
        "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&include_appinfo=true&format=json",
        api_key, steam_id
    );

    let client = Client::new();
    let response = client.get(&url)
        .send()
        .expect("Failed to send request");

    let body = response.text().expect("Failed to read response body");
    println!("Response Body: {}", body); // Print the raw response

    let res: GamesResponse = serde_json::from_str(&body)
        .expect("Failed to parse response");

    for game in res.response.games.iter() {
        println!("Game ID: {}, Name: {}", game.appid, game.name);
    }

    // Save games to JSON
    if let Err(e) = save_games_to_json(&res.response.games, "games.json") {
        eprintln!("Failed to save games to JSON: {}", e);
    } else {
        println!("Games saved to games.json");
    }
}

fn load_json(file_path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    // Open the file and create a buffered reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Deserialize JSON into a vector of Game structs
    let games: Vec<Game> = serde_json::from_reader(reader)?;
    
    Ok(games)
}

fn launch_steam_game(app_id: u32) {
    // Construct the command to launch the game using the Steam protocol
    let steam_url = format!("steam://run/{}", app_id);
    
    // Attempt to open the Steam URL
    if let Err(e) = Command::new("cmd")
        .args(&["/C", "start", "", &steam_url]) // Use 'start' to open the URL
        .spawn()
    {
        eprintln!("Failed to launch the game: {}", e);
    }
}

fn find_closest_match<'a>(input: &'a str, games: &'a [Game]) -> Option<(u32, String)> {
    let mut closest_match = None;
    let mut smallest_distance = usize::MAX;

    for game in games {
        let distance = levenshtein(input, &game.name);
        if distance < smallest_distance {
            smallest_distance = distance;
            closest_match = Some((game.appid, game.name.clone())); // Store both ID and name
        }
    }

    closest_match
}


fn main() {
    println!("
        ███████╗████████╗███████╗ █████╗ ███╗   ███╗ ██████╗  █████╗ ███╗   ███╗███████╗
        ██╔════╝╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██╔════╝ ██╔══██╗████╗ ████║██╔════╝
        ███████╗   ██║   █████╗  ███████║██╔████╔██║██║  ███╗███████║██╔████╔██║█████╗  
        ╚════██║   ██║   ██╔══╝  ██╔══██║██║╚██╔╝██║██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  
        ███████║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗
        ╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝
                                                                                        
        ███╗   ███╗ █████╗ ███╗   ██╗ █████╗  ██████╗ ███████╗██████╗                   
        ████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝ ██╔════╝██╔══██╗                  
        ██╔████╔██║███████║██╔██╗ ██║███████║██║  ███╗█████╗  ██████╔╝                  
        ██║╚██╔╝██║██╔══██║██║╚██╗██║██╔══██║██║   ██║██╔══╝  ██╔══██╗                  
        ██║ ╚═╝ ██║██║  ██║██║ ╚████║██║  ██║╚██████╔╝███████╗██║  ██║                  
        ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝                  
                                                                                        
        ██╗███████╗ ██████╗ ███╗   ███╗██╗                                             
        ██╔╝██╔════╝██╔════╝ ████╗ ████║╚██╗                                            
        ██║ ███████╗██║  ███╗██╔████╔██║ ██║                                            
        ██║ ╚════██║██║   ██║██║╚██╔╝██║ ██║                                            
        ╚██╗███████║╚██████╔╝██║ ╚═╝ ██║██╔╝                                            
        ╚═╝╚══════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝                                             
    ");

    if !Path::new("games.json").exists(){
        println!("Acquiring Games List...");
        refresh_game_list();
        println!("Games List Acquired!");
    }

    let games = load_json("games.json").expect("Failed to load games");

    loop {
        let mut user_inp = String::new();
        print!("SGM> ");      
        std::io::stdout().flush().expect("Failed to flush output"); // Ensure the prompt is displayed immediately  
        std::io::stdin().read_line(&mut user_inp).expect("Error occurred while taking input!");
        
        let user_inp = user_inp.trim();

        if user_inp.eq_ignore_ascii_case("help") {
            println!("\nhelp\t\tShows the menu\nshow games\t\tShows the games you own.");
        } else if user_inp.eq_ignore_ascii_case("show games") {
            for game in &games {
                println!("{}", game.name);
            }
        } else if user_inp.starts_with("launch") {
            let msg_arr: Vec<&str> = user_inp.split_whitespace().collect(); 

            if msg_arr.len() > 1 {
                if let Some((id, name)) = find_closest_match(msg_arr[1], &games) {
                    println!("Launching game: {} (ID: {})", name, id);
                    launch_steam_game(id);

                } else {
                    println!("No match found");
                }
            } else {
                println!("Please specify a game to launch.");
            }
        } else if user_inp.eq_ignore_ascii_case("exit") {
            break;
        } else if user_inp.eq_ignore_ascii_case("refresh") {
            refresh_game_list();
        }
    }
}
