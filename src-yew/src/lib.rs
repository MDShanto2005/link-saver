pub use std::collections::HashMap;

pub use link_types::{Link, LinkSavingError};
pub use serde_json::from_str as string_to_struct;
pub use serde_json::to_string as struct_to_string;
pub use uuid::Uuid;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen_futures::spawn_local;
pub use web_sys::HtmlInputElement;
pub use weblog::{console_error, console_log};
pub use yew::prelude::*;

pub use components::*;

pub mod components;

#[wasm_bindgen(module = "/assets/scripts/main.js")]
extern "C" {
    /// Get data from user's filesystem.
    ///
    /// After calling `.await.unwrap().as_string()` it will return `Option<String>`. And it can be `None`. It will be `None` if the file is not exits or can't read the file
    ///
    /// If it returns `Some` then you can parse it as a [`Vec<Link`] *if the file contains valid data*.
    #[wasm_bindgen(js_name = getData, catch)]
    pub async fn get_data() -> Result<JsValue, JsValue>; // Vec<Link>, null

    /// Add data in the file system.
    ///
    /// After calling `.await.unwrap().as_string().unwrap()` will return a `String` which can be parse as [`Link`] or [`LinkSavingError`].
    ///
    /// After calling `.as_string()` it will not return `None`.
    ///
    /// The final `String` can be [`Link`] if this successfully adds the data. This link is a new link after validating the old link passed by this function.
    ///
    /// The final `String` can be [`LinkSavingError`] if the Rust backend sends [`Err(LinkSavingError)`];
    ///
    /// You can also use [`store_data`] function to add the data. But if you need to validate and to add `automatic` infos like `title`, status codes, you should use this function because it calls the Rust backend which will fetch informations. *Behind the scene it uses [`store_data`] to store the links.*
    ///
    /// # Arguments
    ///
    /// `full_data` - it is a JSON string which contains [`Vec<Link`]. This is the list of links the user currently have
    ///
    /// `data` - it is a JSON string which contains [`Link`]. This is the new link the user wants to create
    ///
    #[wasm_bindgen(js_name = addData, catch)]
    pub async fn add_data(full_data: String, data: String) -> Result<JsValue, JsValue>; // Vec<Link>, Link

    /// Store data in user's filesystem
    ///
    /// After calling `.await.unwrap().as_string()` it will return `Option<String>`.
    ///
    /// If this function succcessfully stores the data, then it will return `None`.
    ///
    /// If any errror occurs it will return the error inside the `String`.
    #[wasm_bindgen(js_name = storeData, catch)]
    pub async fn store_data(full_data: String) -> Result<JsValue, JsValue>;
}
