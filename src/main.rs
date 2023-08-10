use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct BlockInfo {
    author: String,
    baseFeePerGas: String,
    difficulty: String,
    extraData: String,
    gasLimit: String,
    gasUsed: String,
    hash: String,
    logsBloom: String,
    miner: String,
    nonce: String,
    number: String,
    parentHash: String,
    receiptsRoot: String,
    sha3Uncles: String,
    size: String,
    stateRoot: String,
    timestamp: String,
    totalDifficulty: String,
    transactions: Vec<String>, // You can change this to a more specific type if needed
    transactionsRoot: String,
    uncles: Vec<String>, // You can change this to a more specific type if needed
}

#[derive(Debug, Deserialize)]
struct ResponseRPC {
    id: Value,
    jsonrpc: Value,
    result: Value,
}

async fn post_it() -> Result<(), Error> {
    let url = "https://httpbin.org/post";
    let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    let client = reqwest::Client::new();
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

async fn get_block_json() -> Result<(), Error> {
    println!("--- get_block_json ------------------------------------------");
    let url = "https://evm.astar.network/";
    let json_data = r#"{"jsonrpc":"2.0", "id":"1", "method":"eth_getBlockByNumber", "params":["0x10126", true]}"#;
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    // Parse the response as JSON
    let response_json: Value = response.json().await?;

    // Print the parsed JSON response
    println!("Response JSON: {:?}", response_json["result"]);
    println!("Response JSON: {:?}", response_json["result"]["hash"]);

    Ok(())
}

async fn get_block_json_type() -> Result<(), Error> {
    println!("--- get_block_json_type ------------------------------------------");
    let url = "https://evm.astar.network/";
    let json_data = r#"{"jsonrpc":"2.0", "id":"1", "method":"eth_getBlockByNumber", "params":["0x10126", true]}"#;
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    // Parse the response as JSON
    let response_json: Value = response.json().await?;
    println!("Response JSON: {:?}", response_json);
    let result = response_json["result"].clone();

    // Print the parsed JSON response
    let block_info: BlockInfo = serde_json::from_value(result).unwrap();

    println!("{:?}", block_info);

    Ok(())
}

async fn get_block_json_type_incall() -> Result<(), Error> {
    println!("--- get_block_json_type_incall ------------------------------------------");
    let url = "https://evm.astar.network/";
    let json_data = r#"{"jsonrpc":"2.0", "id":"1", "method":"eth_getBlockByNumber", "params":["0x10126", true]}"#;
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .body(json_data.to_owned())
        .send()
        .await?
        .json::<ResponseRPC>()
        .await?;

    // Parse the response as JSON
    let result = response.result.clone();

    // Print the parsed JSON response
    println!("{:?}", result);
    println!("Hash: {:?}", result["hash"]);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    post_it().await?;
    get_methods().await?;
    get_block().await?;
    get_block_json().await?;
    get_block_json_type().await?;
    get_block_json_type_incall().await?;
    Ok(())
}
