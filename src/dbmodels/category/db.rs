use crate::dbmodels::category::{Category, CreateCategory};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn category_add(
    client: &Client,
    selfobj: CreateCategory,
) -> Result<CreateCategory, io::Error> {
    let statement = client
        .prepare(
            "INSERT INTO public.category
   (id, name, slug, description)
    VALUES ($0, $1, $2, $3) RETURNING id, name, slug, description",
        )
        .await
        .unwrap();

    client
        .query(
            &statement,
            &[&selfobj.name, &selfobj.slug, &selfobj.description],
        )
        .await
        .expect("Error creating category")
        .iter()
        .map(|row| CreateCategory::from_row_ref(row).unwrap())
        .collect::<Vec<CreateCategory>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating category tables",
        ))
}

// TODO populate fields

pub async fn category_list(client: &Client) -> Result<Vec<Category>, io::Error> {
    let statement = client
        .prepare("select * from public.category order by id desc")
        .await
        .unwrap();

    let category_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Category::from_row_ref(row).unwrap())
        .collect::<Vec<Category>>();

    Ok(category_list)
}

pub async fn category_id(client: &Client, id_category: i32) -> Result<Category, io::Error> {
    let statement = client
        .prepare("select * from public.category where id = $1")
        .await
        .unwrap();

    let maybe_category = client
        .query_opt(&statement, &[&id_category])
        .await
        .expect("Error adding category ")
        .map(|row| Category::from_row_ref(&row).unwrap());

    match maybe_category {
        Some(category) => Ok(category),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

//TODO take into account ID position

pub async fn category_update(
    client: &Client,
    id: i32,
    mdl: CreateCategory,
) -> Result<(), io::Error> {
    let statement = client.prepare("update public.category set (id, name, slug, description) = ($0, $1, $2, $3) where id = $3").await.unwrap();

    let result = client
        .execute(&statement, &[&mdl.name, &mdl.slug, &mdl.description, &id])
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn category_delete(client: &Client, category_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.category WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&category_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD
