#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=chico").await?.print().await?;
    hc.do_get("/hello/chico").await?.print().await?;
    //println!("{:#?}", r);

    hc.do_get("/src/main.rs").await?.print().await?;

    let re = hc
        .do_post("/api/login", json!({"username":"chico","password":"123"}))
        .await?;

    re.print().await?;

    let re = hc
        .do_post("/api/login", json!({"username":"chico","password":"123d"}))
        .await?;

    re.print().await?;
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;
    Ok(())
}
