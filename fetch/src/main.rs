use futures::stream::TryStreamExt;
use mongodb::{
    Client,
    bson::{
        Document, 
        doc,
    },
    options::{
        ClientOptions,
        FindOptions,
    },
};
use pokerust::{
    Berry,
    FromName,
    Pokemon,
};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let berry = Berry::from_name("cheri").unwrap();
    let pokemon = Pokemon::from_name("pikachu").unwrap();

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("Pokedex".to_string());

    let client = Client::with_options(client_options)?;

    let pokedex_database = client.database("pokedex");
    let pokemon_collection = pokedex_database.collection("pokemon");

    let pokemon_doc = doc! {
        "id": i32::from(pokemon.id),
        "name": pokemon.name,
    };

    pokemon_collection.insert_one(pokemon_doc, None).await?;

    let filter = doc! { "id": 25 };
    let find_options = FindOptions::builder()
        .sort(doc! { "name": 1 })
        .build();

    let cursor = pokemon_collection.find(filter, find_options).await?;
    let results: Vec<Document> = cursor.try_collect().await.unwrap();

    for result in results.iter() {
        println!("{}", *result);
    }

    Ok(())
}
