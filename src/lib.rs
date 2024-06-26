#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

pub mod bar;
pub mod doughnut;
pub mod exports;
pub mod functions;
pub mod pie;
pub mod scatter;
pub mod traits;
pub mod types;

#[doc(hidden)]
mod utils;

use exports::get_chart;
use gloo_utils::format::JsValueSerdeExt;
use serde::{de::DeserializeOwned, Serialize};
pub use traits::*;
pub use types::*;
pub use utils::*;

pub trait ChartExt: DeserializeOwned + Serialize {
    fn get_id(self) -> String;

    fn into_chart(self) -> Chart {
        Chart {
            obj: <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self)
                .expect("Unable to serialize chart."),
            id: self.get_id(),
            mutate: false,
            plugins: String::new(),
            defaults: String::new(),
        }
    }

    fn get_chart_from_id(id: &str) -> Option<Self> {
        let chart = get_chart(id);
        serde_wasm_bindgen::from_value(chart)
            .map_err(|e| {
                gloo_console::error!("{}", e.to_string());
                e
            })
            .ok()
    }
}
