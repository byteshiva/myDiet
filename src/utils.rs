use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, console};

use crate::data::Data;

pub async fn fetch_data(file: String) -> Result<Data, JsValue> {
    console::log_1(&format!("fetching_)data").into());
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&file, &opts)?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value= JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let data_info: Data = json.into_serde().unwrap();
    console::log_1(&format!("Resp in Data: {:?}", json).into());
    //Ok(JsValue::from_serde(&data_info).unwrap())
    Ok(data_info)
}
