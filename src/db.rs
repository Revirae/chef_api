use crate::model::*;
use crate::prelude::*;
use crate::DB;

#[allow(dead_code)]
const FOOD: &str = "food";

pub async fn add_food(data: Food) -> Result<Food> {
    match DB.get() {
        Some(db) => {
            let mut created =
                db.create(FOOD).content(data).await?;
            Ok(created.pop().unwrap())
        }
        None => {
            Err(Error::FailedToLockDb("add".into()))
        }
    }
}

pub async fn get_food(id: String) -> Result<Food> {
    // let th = id.split_once(":").unwrap();
    // let id = "food:".to_owned() + &id;
    match DB.get() {
        Some(db) => {
            let rec = db.select(("food", id.clone())).await?;
            if rec.is_none() {
                Err(Error::Generic(id + "not found"))
            } else {
                Ok(rec.unwrap())
            }
        }
        None => {
            Err(Error::FailedToLockDb("get".into()))
        }
    }
}

pub async fn delete_food(
    id: String,
) -> Result<AffectedRows> {
    let th = id.split_once(":").unwrap();

    match DB.get() {
        Some(db) => {
            let _rec: Option<Record> =
                db.delete(th).await?;
            Ok(AffectedRows { rows_affected: 1 })
        }
        None => Err(Error::FailedToLockDb(
            "delete".into(),
        )),
    }
}

pub async fn toggle_food(
    id: String,
) -> Result<AffectedRows> {
    let (tb, id) = id.split_once(":").unwrap();
    let sql =
        "UPDATE type::thing($tb, $id) SET completed = function() { return !this.completed; };";

    let mut response = DB
        .get()
        .expect("3")
        .query(sql)
        .bind(("tb", tb))
        .bind(("id", id))
        .await?;

    let _food_updated = response
        .take::<Vec<Food>>(0)?
        .into_iter()
        .next()
        .ok_or(Error::Generic(
            "Failed to update record".into(),
        ))?;

    Ok(AffectedRows { rows_affected: 1 })
}

pub async fn get_all_foods() -> Result<Vec<Food>> {
    // let foods: Vec<Food> = DB.select(FOOD).await?;

    // Ok(foods)
    let sql = "SELECT * FROM type::table($table) ORDER BY created_at DESC;";

    match DB.get() {
        Some(db) => {
            let mut response = db
                .query(sql)
                .bind(("table", FOOD))
                .await?;
            let foods: Vec<Food> =
                response.take(0)?;
            Ok(foods)
        }
        None => Err(Error::FailedToLockDb(
            "get_all".into(),
        )),
    }
}
