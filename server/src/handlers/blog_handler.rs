use std::sync::Arc;

use crate::{ error::Error, models::{blog::{CategorySchema, CreateBlogPost, CreateCategory}, querys::{BlogPostPaginationQuery, TotalPostQuery}}, prelude::{ResponseStatusType, SAMPLE_MD}, util };
use axum::{ extract::{Query, State, Path}, response::IntoResponse, Json };
use chrono::Local;
use serde_json::{json, Value};

use crate::{db::Database, models::blog::BlogPostSchema};

// TODO: to be moved to general handlers
pub async fn health_check() -> impl IntoResponse {
    let hc_response = json!({
        "status": "success",
        "data": "server status -> up & running"
    });

    Json(hc_response)
}

pub async fn get_total_categories_count(State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut db_query = db.client.query("SELECT count() FROM categories GROUP ALL")
        .await?;
    
    let result:Option<u32> = db_query.take("count")?;
    let total_count = match result {
        Some(value) => value,
        None => 0
    };
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result)))
}

pub async fn get_total_posts_count(Query(query): Query<TotalPostQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let category = query.category;
    let (query, category) = match category {
        Some(cat) => {
            let cat = cat.split(",").map(|s| s.to_string()).collect();
            (format!("SELECT count() FROM blog_posts WHERE category CONTAINSANY $cat GROUP ALL"), cat)
        },
        None => (format!("SELECT count() FROM blog_posts GROUP ALL"), Vec::new())
    };
    
    let mut db_query = db.client.query(query)
        .bind(("category", category))
        .await?;
    
    let result:Option<u32> = db_query.take("count")?;
    let total_count = match result {
        Some(value) => value,
        None => 0
    };
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result)))
}

pub async fn get_latest_posts(State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut db_query = db.client.query("SELECT * OMIT content FROM blog_posts ORDER BY date DESC LIMIT 3")
        .await?;
    
    let results: Vec<BlogPostSchema> = db_query.take(0)?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()),results)))
}

pub async fn get_categories(State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut db_query = db.client.query("SELECT * FROM categories")
        .await?;
    
    let results: Vec<CategorySchema> = db_query.take(0)?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()),results)))
}

pub async fn get_category_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, Error>{
    let record: Option<CategorySchema> = db.client.select(("categories", id)).await?;
    
    match record {
        Some(r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "category not found")))
    }
}

pub async fn delete_category_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, Error>{
    let record: Option<CategorySchema> = db.client.delete(("categories", id)).await?;
    
    match record {
        Some(r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "category not found")))
    }
}

pub async fn create_category(State(db): State<Arc<Database>>, Json(payload): Json<CreateCategory>) -> Result<Json<Value>, Error> {
    let bp_uuid = util::gen_uuid();
    let category_schema = CategorySchema {
        uuid: bp_uuid.clone(),
        name: payload.name
    };
    
    let record: Option<CategorySchema> = db.client.create(("categories", bp_uuid)).content(category_schema).await?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), record)))
}

pub async fn get_blog_posts(Query(query): Query<BlogPostPaginationQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut page = query.page.unwrap_or_else(|| 1);
    page = if query.page.unwrap_or_else(|| 1) == 0 { 1 } else { page }; // prevents assigning page 0

    let category = query.category;
    let page_size = query.page_size.unwrap_or_else(|| 5);
    let start = (page - 1) * page_size;
    
    let (query, category) = match category {
        Some(cat) => {
            let cat = cat.split(",").map(|s| s.to_string()).collect();
            (format!("SELECT * OMIT content FROM blog_posts WHERE category CONTAINSANY $cat LIMIT $page_size START $start"), cat)
        },
        None => (format!("SELECT * OMIT content FROM blog_posts LIMIT $page_size START $start"), Vec::new())
    };
    
    let mut db_query = db.client.query(query)
        .bind(("page_size", page_size))
        .bind(("start", start))
        .bind(("cat", category))
        .await?;

    let mut results: Vec<BlogPostSchema> = db_query.take(0)?;
    
    results.iter_mut().map(|r| async {
        let categories: Result<Vec<CategorySchema>, Error> = get_categories_from_ids(&db, &r.category.iter().map(|c| c.clone()).collect()).await;
        let new_cat = match categories {
            Ok(c) => c.iter().map(|c| c.name.clone()).collect(),
            Err(_) => vec![]
        };
        
        r.category = new_cat;
    });

    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), results)))
}

pub async fn get_categories_from_ids(db: &Arc<Database>, uuids: &Vec<String>) -> Result<Vec<CategorySchema>, Error>{
    let result: Vec<CategorySchema> = db.client.query(format!("SELECT * FROM categories WHERE uuid IN [{}]", uuids.join(", ")))
        .await?.take(0)?;
    
    Ok(result)
}

pub async fn get_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, Error> {
    let record: Option<BlogPostSchema> = db.client.select(("blog_posts", &id)).await?;

    match record {
        Some(mut r) => {
            let categories: Vec<CategorySchema> = get_categories_from_ids(&db, &r.category.iter().map(|c| c.clone()).collect()).await?;
            r.category = categories.iter().map(|c| c.name.clone()).collect();
            
            Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r)))
        },
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "blog post not found")))
    }
}

// TODO: create a separate page to handle blog post creation
pub async fn create_blog_post(State(db): State<Arc<Database>>, Json(payload): Json<CreateBlogPost>) -> Json<Value> {
    let current_dt = Local::now().format("%d-%m-%y").to_string();
    let bp_uuid = util::gen_uuid();
    
    let mut post = BlogPostSchema{
        uuid: bp_uuid.clone(),
        author: payload.author,
        title: payload.title,
        date: payload.date,
        description: payload.description,
        category: payload.category,
        content: Some(payload.content)
    };
    post.convert_content_to_html();
    
    let record: Result<Option<BlogPostSchema>, surrealdb::Error> = db.client.create(("blog_posts", &bp_uuid)).content(post).await;

    match record {
        Ok(data) => Json(util::gen_response(ResponseStatusType::Success("200".to_string()),data)),
        Err(err) => {
            eprintln!("[ERROR] {err}"); //TODO: switch this to a logger eventually

            Json(util::gen_response(
                ResponseStatusType::Error("400".to_string()),
                "failed to create blog post",
            ))
        }
    }
}

pub async fn update_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>, Json(payload): Json<CreateBlogPost>) -> Result<Json<Value>, Error> {
    let result: CreateBlogPost = db.client.update(("blog_posts", &id))
        .content(payload).await?
        .unwrap_or_default();
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result)))
}

pub async fn delete_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, Error> {
    let record: Option<BlogPostSchema> = db.client.delete(("blog_posts", &id)).await?;

    match record {
        Some(r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "blog post not found")))
    }
}
