use canapi::Provider;
use rocket::{State, http::uri::Origin};
use rocket_contrib::Json;
use serde_json;
use serde_qs;
use workerpool::{Pool, thunk::*};

use plume_api::posts::PostEndpoint;
use plume_models::{
    Connection,
    db_conn::DbConn,
    posts::Post,
};
use api::authorization::*;

type Worker = Pool<ThunkWorker<()>>;

#[get("/posts/<id>")]
fn get(id: i32, conn: DbConn, worker: State<Worker>, auth: Option<Authorization<Read, Post>>) -> Json<serde_json::Value> {
    let post = <Post as Provider<(&Connection, &Worker, Option<i32>)>>::get(&(&*conn, &*worker, auth.map(|a| a.0.user_id)), id).ok();
    Json(json!(post))
}

#[get("/posts")]
fn list(conn: DbConn, uri: &Origin, worker: State<Worker>, auth: Option<Authorization<Read, Post>>) -> Json<serde_json::Value> {
    let query: PostEndpoint = serde_qs::from_str(uri.query().unwrap_or("")).expect("api::list: invalid query error");
    let post = <Post as Provider<(&Connection, &Worker, Option<i32>)>>::list(&(&*conn, &*worker, auth.map(|a| a.0.user_id)), query);
    Json(json!(post))
}

#[post("/posts", data = "<payload>")]
fn create(conn: DbConn, payload: Json<PostEndpoint>, worker: State<Worker>, auth: Authorization<Write, Post>) -> Json<serde_json::Value> {
    let new_post = <Post as Provider<(&Connection, &Worker, Option<i32>)>>::create(&(&*conn, &*worker, Some(auth.0.user_id)), (*payload).clone());
    Json(new_post.map(|p| json!(p)).unwrap_or(json!({
        "error": "Invalid data, couldn't create new post"
    })))
}
