use std::fs::File;

use clap::Parser;

mod algorithm;
mod serialization;
use serialization::{LobbyData, parse_graph};
use algorithm::travel;


#[derive(Parser)]
struct Cli {
    file: String,

    #[arg(long, short, value_name = "COUNT")]
    rank: Option<i32>
}

fn main() {
    let cli = Cli::parse();
    let data_file = cli.file;
    let file = match File::open(&data_file) {
        Err(msg) => panic!("cannot open file {}: {:?}", &data_file, msg),
        Ok(file) => file,
    };
    let lobby: LobbyData = match serde_json::from_reader(file){
        Err(msg) => panic!("cannot parse json. {}", msg),
        Ok(lobby) => lobby,
    };
    let graph = parse_graph(&lobby.adj_table);
    let source: i32 = (&lobby.source).parse().unwrap();
    let destination: i32 = (&lobby.destination).parse().unwrap();
    let ranking_count: usize = (&cli.rank).unwrap_or(1).try_into().unwrap();
    println!("travel through the lobby: {}", &lobby.lobby_path);
    println!("source: {}", &lobby.source);
    println!("destination: {}", &lobby.destination);
    let (counter, rc, winner) = travel(&graph, source, destination, ranking_count);
    println!("find {} paths", counter);
    println!("fastest {} paths:", rc);
    let mut i = 1;
    for (d, path) in winner {
        println!("{}. t:{}", i, d);
        for v in path {
            print!("{}->", v);
        }
        i += 1;
    }
}
