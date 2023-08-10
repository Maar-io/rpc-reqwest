use reqwest::{Client, Error};

async fn post_it() -> Result<(), Error> {
    let url = "https://httpbin.org/post";
    let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    let client = reqwest::Client::new();
    println!(
        "{:?}",
        client
            .post(url)
            .header("Content-Type", "application/json;charset=utf-8")
            .body(json_data.to_owned())
    );
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}

async fn get_methods() -> Result<(), Error> {
    println!("--- get_methods ------------------------------------------");
    let url = "https://evm.astar.network/";
    let json_data = r#"{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}"#;
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}

/// Get block by number
/// curl command:
/*
    curl https://evm.astar.network/ -H "Content-Type:application/json;charset=utf-8" -d \
  '{"jsonrpc":"2.0", "id":1, "method":"eth_getBlockByNumber", "params": ["0x10126", true]}'
*/
  async fn get_block() -> Result<(), Error> {
    println!("--- get_block ------------------------------------------");
    let url = "https://evm.astar.network/";
    let json_data = r#"{"jsonrpc":"2.0", "id":"1", "method":"eth_getBlockByNumber", "params":["0x10126", true]}"#;
    // println!("json_data: {}", json_data);
    let client = reqwest::Client::new();

    // println!(
    //     "{:?}",
    //     client
    //         .post(url)
    //         .header("Content-Type", "application/json;charset=utf-8")
    //         .body(json_data.to_owned())
    // );
    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // post_it().await?;
    get_methods().await?;
    get_block().await?;
    Ok(())
}


