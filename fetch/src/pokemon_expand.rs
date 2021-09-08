use mongodb::bson::{
    Document,
    doc,
};
use pokerust::Pokemon;

pub trait ToDocument {
    fn to_document(self) -> Document;
}

impl ToDocument for Pokemon {
    fn to_document(self) -> Document {
        doc! {
            "id": i32::from(self.id),
            "name": self.name,
        }
    }
}
