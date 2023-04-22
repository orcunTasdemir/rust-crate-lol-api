use crate::web;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Champion {
    pub id: u64,
    data: Value,
}

#[derive(Clone)]
pub struct ModelController {
    champions_store: Arc<Mutex<Vec<Option<Champion>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        let champions = Self::populate_champions_store().await?;
        Ok(Self {
            champions_store: Arc::new(Mutex::new(
                champions.iter().map(|c| Some(c.clone())).collect(),
            )),
        })
    }

    async fn populate_champions_store() -> Result<Vec<Champion>> {
        let api_url = format!(
            "{}{}{}",
            web::BASE_URL,
            web::VERSION,
            web::ALL_CHAMPIONS_PATH
        );
        let res = reqwest::get(api_url).await?;
        let json = res.json::<Value>().await?;
        let champions: HashMap<String, Champion> = from_value(json)?;

        // Print a single champion
        if let Some(champion) = champions.get("Aatrox") {
            println!("Champion: {:?}", champion);
        }
        let champions_vec: Vec<Champion> = champions.into_iter().map(|(_, v)| v).collect();
        Ok(champions_vec)
    }

    pub async fn list_champions(&self) -> Result<Vec<Champion>> {
        let store = self.champions_store.lock().unwrap();
        let champions = store.iter().filter_map(|c| c.clone()).collect();
        Ok(champions)
    }

    // pub async fn list_champion(&self, id: u64) -> Result<Champion> {
    //     let store = self.champions_store.lock().unwrap();
    //     let champion = store
    //         .get(id as usize)
    //         .ok_or(Error::NoChampionWithIdFound(id))?;
    //     champion.clone().ok_or(Error::NoChampionWithIdFound(id))
    // }
}
