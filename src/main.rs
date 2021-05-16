use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(StructOpt)]
struct Cli {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cryptodata {
    status: Status,
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    elapsed: i32,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    id: String,
    symbol: String,
    name: String,
    slug: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    price_usd: f64,
    percent_change_usd_last_1_hour: f64,
}


impl Cryptodata {
    async fn get(target: &String) -> Result<Self,ExitFailure> {
        let url = format!("https://data.messari.io/api/v1/assets/{}/metrics/market-data", target);
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Cryptodata>()
            .await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let target =  &args.name;
    let resp = Cryptodata::get(target).await?;

    println!("The price of {} is {:.2} USD, it changed by {:.4}% in the past hour", resp.data.name, resp.data.market_data.price_usd,resp.data.market_data.percent_change_usd_last_1_hour);
    Ok(())
}
