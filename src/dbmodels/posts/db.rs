use crate::posts::{CreatePosts, Posts};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn posts_add(client: &Client, selfobj: CreatePosts) -> Result<CreatePosts, io::Error> {
    let statement = client.prepare("INSERT INTO public.posts
   (id, title, slug, intro, content, created_date, category_id, user_id)
    VALUES ($0, $1, $2, $3, $4, $5, $6, $7) RETURNING id, title, slug, intro, content, created_date, category_id, user_id").await.unwrap();

    client
        .query(
            &statement,
            &[
                &selfobj.id,
                &selfobj.title,
                &selfobj.slug,
                &selfobj.intro,
                &selfobj.content,
                &selfobj.created_date,
                &selfobj.category_id,
                &selfobj.user_id,
            ],
        )
        .await
        .expect("Error creating posts")
        .iter()
        .map(|row| CreatePosts::from_row_ref(row).unwrap())
        .collect::<Vec<CreatePosts>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating posts tables",
        ))
}

// TODO populate fields

pub async fn posts_list(client: &Client) -> Result<Vec<Posts>, io::Error> {
    let statement = client
        .prepare("select * from public.posts order by id desc")
        .await
        .unwrap();

    let posts_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Posts::from_row_ref(row).unwrap())
        .collect::<Vec<Posts>>();

    Ok(posts_list)
}

pub async fn posts_id(client: &Client, id_posts: i32) -> Result<Posts, io::Error> {
    let statement = client
        .prepare("select * from public.posts where id = $1")
        .await
        .unwrap();

    let maybe_posts = client
        .query_opt(&statement, &[&id_posts])
        .await
        .expect("Error adding posts ")
        .map(|row| Posts::from_row_ref(&row).unwrap());

    match maybe_posts {
        Some(posts) => Ok(posts),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

//TODO take into account ID position

pub async fn posts_update(client: &Client, id: i32, mdl: CreatePosts) -> Result<(), io::Error> {
    let statement = client.prepare("update public.posts set (id, title, slug, intro, content, created_date, category_id, user_id) = ($0, $1, $2, $3, $4, $5, $6, $7) where id = $3").await.unwrap();

    let result = client
        .execute(
            &statement,
            &[
                &mdl.id,
                &mdl.title,
                &mdl.slug,
                &mdl.intro,
                &mdl.content,
                &mdl.created_date,
                &mdl.category_id,
                &mdl.user_id,
                &id,
            ],
        )
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn posts_delete(client: &Client, posts_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.posts WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&posts_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD
