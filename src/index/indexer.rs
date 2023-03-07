// use {
//   super::updater::BlockData,
//   anyhow::{anyhow, Result},
//   bitcoin::{Transaction, Txid},
//   bitcoincore_rpc::Auth,
//   hyper::{client::HttpConnector, Body, Client, Method, Request, Uri},
//   serde::Deserialize,
//   serde_json::{json, Value},
// };

// // broadcast block to our rpc server
// pub async fn rpc_mesage(body: &BlockData) -> Result<()> {
//   println!("request>>>>>>>>>{:?}", body);
//   let client = Client::new();
//   let url = "http://127.0.0.1:7001?test=name";
//   let testbody = "".to_string();
//   let req = Request::builder()
//     .method(Method::GET)
//     .uri(url)
//     .header(hyper::header::CONTENT_TYPE, "application/json")
//     .body(Body::from(testbody))?;
//   // .body(Body::empty())?;

//   let response = client.request(req).await?;
//   println!("response>>>>>>>>>{:?}", response);
//   Ok(())
// }
