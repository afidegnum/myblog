use perseus::{make_rx, Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::prelude::{view, View};

#[derive(Debug, PartialEq, Eq, Hash)]
#[make_rx(MenuRx)]
pub struct Menu {
    pub name: String,
    pub bb: String,
    pub cc: String,
    pub dd: String,
    pub sss: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[make_rx(ContentRx)]
pub struct Content {
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[make_rx(FooterRx)]
pub struct Footer {
    pub contact: String,
}

#[perseus::make_rx(DemoPageStateRx)]

pub struct DemoPageState {
    pub menu: Vec<Menu>,
    pub content: Vec<Content>,
    pub footer: Vec<Footer>,
}

#[perseus::template_rx]
pub fn Demo_page(state: DemoPageStateRx) -> View<G> {
    let Demomenu = state.menu.handle();
    let Democontent = state.content.handle();
    let Demofooter = state.footer.handle();

    view! {
        body  {
            ul (data-table="menu", class="menu") {div (data-field="name", class="itm") {home}

                                                  div (data-field="bb", class="itm") {home}

                                                  div (data-field="cc", class="itm") {home}

                                                  div (data-field="dd", class="itm") {home}

                                                  div (data-field="sss", class="itm") {home}

                                                  div (class="itm") {About us}

                                                  div (class="itm") {Contact us}

            }
            div (id="idone", class="classone ") {

                ul (data-table="content", class="listing") {li (data-field="title", class="item1") {First  liste}

                                                            li (class="item2") {Second}

                                                            li (class="item3") {Third}

                }
                div (data-field="intro", id="innerone") {h1  {This Title Update}
                }

                div (id="innertwo") {h2  {Subheads}
                }

            }

            div (id="second", class="below") {}

            div (data-table="footer", class="inner") {
                h1 (data-field="contact") {welcome}

                h1  {another}

                h2  {third}

            }


        }

    }
    //  let mycontents = format!("{:?}", state.content.get());

    //    view! {
    //        ul {
    //            Indexed(IndexedProps {
    //                iterable: mycounts,
    //                template: |x| view! {
    //                    li { ( format!("{:?}", x.title)) }
    //                },
    //            })
    //        }
    //    }

    // template: cloned!(x => move |x| view! {
    //  li { (format!("{:?}", x.title)) }
    // })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("Demo")
        .build_state_fn(get_build_state)
        .template(Demo_page)
        .head(head)
}

#[perseus::head]
pub fn head(_props: DemoPageState) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<DemoPageState> {
    //    let c1 = Content {
    //        title: String::from("Alpha"),
    //   };

    //    let c2 = Content {
    //        title: String::from("Beta"),
    //    };

    //    let c3 = Content {
    //        title: String::from("Gamma"),
    //    };

    //    Ok(IndexPageState {
    //        content: vec![c1, c2, c3],
    //    })
}
