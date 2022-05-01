use crate::dbmodels::category;
use crate::dbmodels::category::CreateCategory;
use global_state;
use perseus::{make_rx, Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::prelude::{view, View};

//#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
// #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[derive(Debug, PartialEq, Eq, Hash)]
#[make_rx(ContentRx)]
pub struct Content {
    pub title: String,
}

// #[derive(Debug, PartialEq, Eq, Hash)]
// #[make_rx(ContentRx)]
// pub struct Content <'foo>{
//     pub title: &'foo str,
// }

// impl serde::Serialize for Content {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         todo!()
//     }
// }

// impl std::fmt::Display for Content {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(fmt, "Title {} years old.", self.title)
//     }
// }

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub content: Vec<Content>,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    // let mycontents = format!("{:?}", state.content.get());
    // let mycounts = format!("{:?}", state.content.handle());
    let mycounts = state.content.handle();

    //  let views = View::new_fragment(mycounts.iter().map(|&x| view! { li (x) }).collect());
    //        p { (mycontents) }

    view! {
        ul {
            Indexed(IndexedProps {
                iterable: mycounts,
                template: |x| view! {
                    li { ( format!("{:?}", x.title)) }
                },
            })
        }
    }

    // view! {
    //     ul {
    //         Keyed(KeyedProps {
    //             iterable: mycounts,
    //             template: |x| view! {
    //                 li { ( format!("{:?}", x)) }
    //             },
    //             key: |x| *x.title,
    //         })
    //     }
    // }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
    // db_pool: web::Data<Pool>,
) -> RenderFnResultWithCause<IndexPageState> {
    let pool = global_state::make_db_pool().await;
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::category_list(&client).await;
}
