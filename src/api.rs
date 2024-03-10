use actix_web::{
    delete,
    get,
    patch,
    post,
    web::{Json, Path},
    // HttpResponse,
};
use chrono::Local;

use crate::db::*;
use crate::model::*;
use crate::prelude::*;

#[post(
    "/food/{name}/{brand}/{cost}/{weight}/{volume}"
)]
pub async fn create(
    food: Path<(String, String, usize, usize, usize)>,
) -> Result<Json<Food>> {
    let (name, brand, cost, weight, volume) =
        food.into_inner();
    let food = Food {
        id: None,
        name,
        brand,
        cost,
        weight,
        volume,
        created_at: Local::now(),
    };
    let food = add_food(food).await?;

    Ok(Json(food))
}

#[get("/food/{id}")]
pub async fn get(
    id: Path<String>,
) -> Result<Json<Food>> {
    let food = get_food(id.into_inner()).await?;

    Ok(Json(food))
}

#[patch("/food/{id}")]
pub async fn update(
    id: Path<String>,
) -> Result<Json<AffectedRows>> {
    let updated =
        toggle_food(id.into_inner()).await?;

    Ok(Json(updated))
}

#[delete("/food/{id}")]
pub async fn delete(
    id: Path<String>,
) -> Result<Json<AffectedRows>> {
    let deleted =
        delete_food(id.into_inner()).await?;

    Ok(Json(deleted))
}

#[get("/foods")]
pub async fn list() -> Result<Json<Vec<Food>>> {
    let foods = get_all_foods().await?;

    Ok(Json(foods))
}
