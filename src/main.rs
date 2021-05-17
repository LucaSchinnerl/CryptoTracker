use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "bitcoin")] 
    crypto_currency: String,

    #[structopt(short = "m", long = "mute")]
    mute: bool,
    
    #[structopt(short = "v", long = "volumne")]
    volume: bool,

    #[structopt(short = "b", long = "price_btc")]
    price_in_btc: bool,

    #[structopt(short = "e", long = "price_eth")]
    price_in_eth: bool,

    #[structopt(short = "c", long = "change_24h")]
    change_24h: bool,
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
    price_btc: f64,
    price_eth: f64,
    volume_last_24_hours: f64,
    percent_change_usd_last_1_hour: f64,
    percent_change_usd_last_24_hours: f64,

}


impl Cryptodata {
    async fn get(target: &String) -> Result<Self,ExitFailure> {
        let url = format!("https://data.messari.io/api/v1/assets/{}/metrics/market-data", target);
        let url = Url::parse(&*url)?;
        
        

        let resp = reqwest::get(url)
            .await?
            .json::<Cryptodata>()
            .await;
        
        let resp = match resp {
            Ok(cryptodata) => {cryptodata}
            Err(_error) => {panic!("No data available for {}. For a list of available currencies use -l", target);}
        };
        
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let target =  &args.crypto_currency;
    let resp = Cryptodata::get(target).await?;

    println!("Information about {}:", resp.data.name);
    if !args.mute {
        println!("The price is {:.2} USD, it changed by {:.4}% in the last hour", resp.data.market_data.price_usd,resp.data.market_data.percent_change_usd_last_1_hour);
    }
    if args.volume {
        println!("The volumne of the last 24h was {:.2} USD", resp.data.market_data.volume_last_24_hours);
    }
    if args.price_in_btc {
        println!("The price is {:.2} BTC", resp.data.market_data.price_btc);
    }
    if args.price_in_eth {
        println!("The price is {:.2} ETH", resp.data.market_data.price_eth); 
    }
    if args.change_24h {
        println!("The price changed by {:.4}% in the last 24h", resp.data.market_data.percent_change_usd_last_24_hours);
    }
    Ok(())
}
