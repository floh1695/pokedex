mod config;
use config::Config;

mod pokemon_expand;
use pokemon_expand::ToDocument;

use futures::stream::TryStreamExt;
use mongodb::{
    Client,
    Collection,
    Database,
    bson::{
        Document,
        doc,
    }, 
    options::{
        ClientOptions,
        FindOptions,
    }
};
use pokerust::{
    FromId,
    Pokemon,
};
use std::{
    env::{
        args,
        current_dir,
    },
    error::Error,
    time::SystemTime,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = args().collect();
    let program = arguments
        .get(1)
        .ok_or("Please input a program: 'fetch' or 'index'")?;

    let app_root = current_dir()?;
    let config_path = app_root.join("config/config.ron");
    let config = Config::from_file_path(config_path)?;

    let database = get_database(config.mongo_url, config.mongo_database).await?;
    let pokemon_collection = database.collection("pokemon");

    let before = SystemTime::now();
    match &program[..] {
        "fetch" => {
            add_all_pokemon_to_collection(pokemon_collection.clone()).await?;
        },
        "index" => {
            let filter = doc! { "id": 25 };
            let find_options = FindOptions::builder()
                .sort(doc! { "name": 1 })
                .build();

            let cursor = pokemon_collection.clone().find(filter, find_options).await?;
            let results: Vec<Document> = cursor.try_collect().await?;

            for result in results.iter() {
                println!("{}", *result);
            }
        },
        _ => {
            return Err(format!("'{}' is not a valid program name", program).into());
        },
    }
    let elapsed = before.elapsed()?;
    let elapsed_seconds = elapsed.as_micros() as f64 / 1000000.0;
    println!("Seconds to run program {}: {}", program, elapsed_seconds);

    Ok(())
}

async fn add_all_pokemon_to_collection(collection: Collection) -> Result<(), Box<dyn Error>> {
    collection.drop(None).await?;

    let all_ids = get_pokemon_ids();
    let all_jobs = all_ids.iter()
        .map(|&id| Pokemon::from_id(id).map_err(|e| format!("Error processing {}\n{}", id, e)));
    let all_pokemon = all_jobs.into_iter()
        .collect::<Result<Vec<Pokemon>, String>>()?;
    let all_documents = all_pokemon.iter()
        .map(|pokemon| pokemon.clone().to_document());

    collection.insert_many(all_documents, None).await?;

    Ok(())
}

fn get_pokemon_ids() -> Vec<i16> {
    let first = 1i16;
    let last = 898i16;
    let mut ids = vec![];

    for id in first..=last {
        ids.push(id);
    }

    ids
}

async fn get_database(mongo_url: String, database_name: String) -> Result<Database, Box<dyn Error>> {
    let client = get_client(mongo_url).await?;
    let database = client.database(&database_name);
    
    Ok(database)
}

async fn get_client(mongo_url: String) -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse(&mongo_url).await?;
    let client = Client::with_options(client_options)?;

    Ok(client)
}
