use crate::web;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Champion {
    pub name: String,
    data: Value,
}

#[derive(Clone)]
pub struct ModelController {
    champions_store: Arc<Mutex<Vec<Option<Champion>>>>,
}

impl ModelController {
    // pub fn print_champions(&self) {
    //     let store = self.champions_store.lock().unwrap();
    //     for champion in store.iter() {
    //         if let Some(c) = champion {
    //             println!("Champion: {:?}", c);
    //         }
    //     }
    // }
    pub async fn new() -> Result<Self> {
        let champions_store = Self::populate_champions_store().await?;
        Ok(Self {
            champions_store: Arc::new(Mutex::new(champions_store)),
        })
    }

    async fn populate_champions_store() -> Result<Vec<Option<Champion>>> {
        let api_url = format!(
            "{}{}{}",
            web::BASE_URL,
            web::VERSION,
            web::ALL_CHAMPIONS_PATH
        );
        let res = reqwest::get(api_url).await?;
        let json = res.json::<Value>().await?;
        //this one runs
        // let json_str = serde_json::to_string(&json)?;
        // let n = 50; // Replace with the number of characters you want to print
        // println!("JSON data: {}", &json_str.get(..n).unwrap_or(&json_str));

        let data = json.get("data").ok_or(Error::InvalidApiResponse)?;
        let champions: Vec<Champion> = data
            .as_object()
            .ok_or(Error::InvalidApiResponse)?
            .values()
            .map(|value| Champion {
                name: value["name"].as_str().unwrap().to_owned(),
                data: value.clone(),
            })
            .collect();
        Ok(champions.into_iter().map(Some).collect())
    }

    pub async fn list_champions(&self) -> Result<Vec<Champion>> {
        let store = self.champions_store.lock().unwrap();
        let champions = store.iter().filter_map(|c| c.clone()).collect();
        Ok(champions)
    }

    pub async fn list_champion(&self, name: &str) -> Result<Champion> {
        let store = self.champions_store.lock().unwrap();
        let champion = store
            .iter()
            .find(|c| {
                if let Some(champion) = c {
                    champion.name == name
                } else {
                    false
                }
            })
            .ok_or(Error::NoChampionWithNameFound(name.to_string()))?;
        champion
            .clone()
            .ok_or(Error::NoChampionWithNameFound(name.to_string()))
    }
}
