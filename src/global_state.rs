use deadpool_postgres::{Manager, Pool, Runtime};

use dotenv::dotenv;
use perseus::{state::GlobalStateCreator, RenderFnResult};
use serde::{Deserialize, Serialize};

use crate::configs;

// use deadpool_postgres::Pool;
// use openssl::ssl::{SslConnector, SslMethod};
// use postgres_openssl::*;

async fn make_db_pool() {
    let config = configs::Config::from_env().unwrap();
    config
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();

    //     let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    //     let connector = MakeTlsConnector::new(builder.build());
    //     config.pg.create_pool(connector).unwrap()
}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[perseus::make_rx(AppStateRx)]
pub struct AppState {
    pub pools: BasicPool,
}
#[derive(Deserialize, Clone)]
pub struct BasicPool {
    pub basics: Pool,
}

// #[perseus::autoserde(global_build_state)]
// pub async fn get_build_state() -> RenderFnResult<AppState> {
//     dotenv().ok();

//     let config = configs::Config::from_env().unwrap();

//     let pool = config
//         .pg
//         .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
//         .unwrap();

//     Ok(AppState {
//         dbpool: PoolsData {
//             pools: pool.clone(),
//         },
//     })
// }

#[perseus::autoserde(global_build_state)]
pub async fn get_build_state() -> RenderFnResult<AppState> {
    let pool = make_db_pool().await;
    Ok(AppState {
        pools: pool.clone(),
    })
}
