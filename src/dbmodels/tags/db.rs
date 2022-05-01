use crate::tags::{CreateTags, Tags};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn tags_add(client: &Client, selfobj: CreateTags) -> Result<CreateTags, io::Error> {
    let statement = client
        .prepare(
            "INSERT INTO public.tags
   (id, slug, description, icon)
    VALUES ($0, $1, $2, $3) RETURNING id, slug, description, icon",
        )
        .await
        .unwrap();

    client
        .query(
            &statement,
            &[
                &selfobj.id,
                &selfobj.slug,
                &selfobj.description,
                &selfobj.icon,
            ],
        )
        .await
        .expect("Error creating tags")
        .iter()
        .map(|row| CreateTags::from_row_ref(row).unwrap())
        .collect::<Vec<CreateTags>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating tags tables",
        ))
}

// TODO populate fields

pub async fn tags_list(client: &Client) -> Result<Vec<Tags>, io::Error> {
    let statement = client
        .prepare("select * from public.tags order by id desc")
        .await
        .unwrap();

    let tags_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Tags::from_row_ref(row).unwrap())
        .collect::<Vec<Tags>>();

    Ok(tags_list)
}

pub async fn tags_id(client: &Client, id_tags: i32) -> Result<Tags, io::Error> {
    let statement = client
        .prepare("select * from public.tags where id = $1")
        .await
        .unwrap();

    let maybe_tags = client
        .query_opt(&statement, &[&id_tags])
        .await
        .expect("Error adding tags ")
        .map(|row| Tags::from_row_ref(&row).unwrap());

    match maybe_tags {
        Some(tags) => Ok(tags),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

//TODO take into account ID position

pub async fn tags_update(client: &Client, id: i32, mdl: CreateTags) -> Result<(), io::Error> {
    let statement = client
        .prepare(
            "update public.tags set (id, slug, description, icon) = ($0, $1, $2, $3) where id = $3",
        )
        .await
        .unwrap();

    let result = client
        .execute(
            &statement,
            &[&mdl.id, &mdl.slug, &mdl.description, &mdl.icon, &id],
        )
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok((