async fn create_post(post: web::Json<CreatePost>, pool: web::Data<Pool>, user: web::ReqData<User>) -> HttpResponse {
    use schema::posts::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let new_post = NewPost {
        id: Uuid::new_v4(),
        user_id: user.id,
        title: &post.title,
        body: &post.body,
        published: false,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post");

    HttpResponse::Ok().json("Post created")
}

async fn get_posts(pool: web::Data<Pool>) -> HttpResponse {
    use schema::posts::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let results = posts
        .filter(published.eq(true))
        .load::<Post>(&conn)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}

async fn update_post(post_id: web::Path<Uuid>, post: web::Json<CreatePost>, pool: web::Data<Pool>, user: web::ReqData<User>) -> HttpResponse {
    use schema::posts::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let target = posts.filter(id.eq(post_id.into_inner())).filter(user_id.eq(user.id));

    diesel::update(target)
        .set((title.eq(&post.title), body.eq(&post.body), updated_at.eq(Utc::now().naive_utc())))
        .execute(&conn)
        .expect("Error updating post");

    HttpResponse::Ok().json("Post updated")
}

async fn delete_post(post_id: web::Path<Uuid>, pool: web::Data<Pool>, user: web::ReqData<User>) -> HttpResponse {
    use schema::posts::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let target = posts.filter(id.eq(post_id.into_inner())).filter(user_id.eq(user.id));

    diesel::delete(target)
        .execute(&conn)
        .expect("Error deleting post");

    HttpResponse::Ok().json("Post deleted")
}
