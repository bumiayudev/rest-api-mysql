use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use crate::{
    model::{Post, PostResponse}, schema::{CreatePostSchema, FilterOptions}, AppState
};


fn filter_db_record(post: &Post) -> PostResponse {
    PostResponse {
        id: post.id.to_owned(),
        title: post.title.to_owned(),
        content: post.content.to_owned(),
    }
}

#[get("/api/posts")]
pub async fn get_post(opt: web::Query<FilterOptions>,config: web::Data<AppState>)-> impl Responder{
    let limit = opt.limit.unwrap_or(10);
    let offset = (opt.page.unwrap_or(1) - 1) * limit;
    let posts = sqlx::query_as!(Post, r#"SELECT id, title, content FROM posts ORDER BY id DESC LIMIT ? OFFSET ?"#, limit as i32, offset as i32)
        .fetch_all(&config.db)
        .await
        .unwrap();

    let post_resp = posts
                            .into_iter()
                            .map(|post| filter_db_record(&post))
                            .collect::<Vec<PostResponse>>();
    let response = serde_json::json!({
        "status" : "success",
        "posts" : post_resp
    });

    HttpResponse::Ok().json(response)
}

#[post("/api/posts")]
pub async fn create_post(body: web::Json<CreatePostSchema>, config: web::Data<AppState>) -> impl Responder{
    sqlx::query!(r#"INSERT INTO posts(title, content) VALUES(?, ?)"#, body.title, body.content)
        .execute(&config.db)
        .await;

    let query_result = sqlx::query_as!(Post ,r#"SELECT id, title, content FROM posts WHERE title =?"#, body.title)
        .fetch_one(&config.db)
        .await;

    match query_result {
        Ok(value) => {
            let post_response = serde_json::json!({"status": "success", "data": serde_json::json!({"post": filter_db_record(&value)})});

            return HttpResponse::Ok().json(post_response);
        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", err)
            }));
        }
    }
}

#[get("/api/posts/{id}")]
pub async fn get_single_post(path: web::Path<i32>, config: web::Data<AppState>) -> impl Responder{
    let post_id = path.into_inner().to_string();
    let query_result = sqlx::query_as!(Post, r#"SELECT id, title, content FROM posts WHERE id=?"#, post_id)
                    .fetch_one(&config.db)
                    .await;

    match query_result {
        Ok(post) => {
            let post_response = serde_json::json!({"status": "success", "data": serde_json::json!({"post": filter_db_record(&post)})});

            return HttpResponse::Ok().json(post_response)
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}));
        }
    }
}

#[patch("/api/posts/{id}")]
pub async fn edit_post(path: web::Path<i32>, body: web::Json<CreatePostSchema>, config: web::Data<AppState>)-> impl Responder{
    let post_id = path.into_inner().to_string();

    let update_result = sqlx::query(r#"UPDATE posts SET title = ?, content = ? WHERE id= ?"#)
    .bind(body.title.to_owned())
    .bind(body.content.to_owned())
    .bind(post_id.to_owned())
    .execute(&config.db)
    .await;
    
    match  update_result{
        Ok(value) => {
            if value.rows_affected() == 0 {
                let message = format!("Post with ID: {} not found", post_id);
                return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}));
            }
        },
        Err(err) => {
            let message = format!("Internal server error: {}", err);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}));
        }
    }

    let _result = sqlx::query_as!(Post, r#"SELECT id, title, content FROM posts WHERE id=?"#, post_id.to_owned())
            .fetch_one(&config.db)
            .await;
    match _result {
        Ok(row) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": filter_db_record(&row)
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)})),
    }
}

#[delete("/api/posts/{id}")]
pub async fn delete_post(path: web::Path<i32>, config: web::Data<AppState>) -> impl Responder{
    let post_id = path.into_inner().to_string();
    let query_result = sqlx::query!(r#"DELETE FROM posts WHERE id = ?"#, post_id )
                .execute(&config.db).await;

    match  query_result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let message = format!("Post with ID: {} not found", post_id);
                return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}));
            } else {
                return HttpResponse::Ok().json(json!({"status": "success", "message": "Delete record successfully"}));
            }
        },
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Internal server error {:?}", err)}));
        }
    }
}