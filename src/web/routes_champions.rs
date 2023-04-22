use crate::model::Champions::Champion;
use crate::model::Champions::ModelController;
use crate::Result;
use axum::extract::Path;
use axum::{extract::State, response::Json, routing::get, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/allchampions", get(list_champions))
        //.route("/allchampions/:id", get(list_champion))
        .with_state(mc)
}

async fn list_champions(State(mc): State<ModelController>) -> Result<Json<Vec<Champion>>> {
    let champions = mc.list_champions().await?;
    Ok(Json(champions))
}
// async fn list_champion(
//     State(mc): State<ModelController>,
//     Path(id): Path<u64>,
// ) -> Result<Json<Champion>> {
//     let champion = mc.list_champion(id).await?;
//     Ok(Json(champion))
// }
