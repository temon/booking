use std::convert::TryFrom;
use std::error::Error;
use std::time::{Duration, SystemTime};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dog {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DogRequest {
    pub owner: String,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: DogRequest) -> Result<Self, Self::Error> {
        Ok(Self{
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner id"),
            name: item.name,
            age: item.age,
            breed: item.breed,
        })
    }
}