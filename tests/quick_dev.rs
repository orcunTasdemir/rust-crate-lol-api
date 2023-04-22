#![allow(unused)]

use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::any::type_name;
use std::any::Any;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[tokio::test]
async fn tests_1() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let res = hc.do_get("/api/allchampions").await?.print().await?;

    Ok(())
}
