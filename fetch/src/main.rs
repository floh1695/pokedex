use mongodb::{
    Client,
    options::ClientOptions
};
use pokerust::{
    Berry,
    FromName,
    Pokemon
};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let berry = Berry::from_name("cheri").unwrap();
    println!("{}: {}", berry.name, berry.max_harvest);

    let pokemon = Pokemon::from_name("pikachu").unwrap();
    println!("{}: {}", pokemon.name, pokemon.base_experience);

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("Pokedex".to_string());

    let client = Client::with_options(client_options)?;

    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
}
