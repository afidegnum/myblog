pub mod configs;
pub mod dbmodels;
mod error_pages;
mod templates;

pub mod global_state;

use dotenv::dotenv;

// pub mod errors;

//use crate::config::{SecConfig, KEY_LENGTH};
use perseus::{Html, PerseusApp};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .global_state_creator(global_state::get_global_state_creator())
}

