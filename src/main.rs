use structopt::StructOpt;

mod location;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Callsign to track
    #[structopt(short, long, default_value = "LB7HJ-10")]
    callsign: String,

    /// MongoDB connection String
    #[structopt(short, long, default_value = "mongodb://localhost:27017")]
    mongodb: String,

    /// Collect data to mongodb
    #[structopt(short, long)]
    collect_data: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    let mut client = mongodb::Client::with_uri_str(&opts.mongodb).await?;
    let db = client.database("aprs");
    let collection = db.collection("location");

    if opts.collect_data {
        println!("Collecting data for {}...", opts.callsign);
        let client_builder = reqwest::Client::builder();
        static APP_USER_AGENT: &str = concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
            " ",
            env!("CARGO_PKG_HOMEPAGE"),
        );
        let client = client_builder.user_agent(APP_USER_AGENT).build().unwrap();
        let res = client
            .get(format!(
                "https://api.aprs.fi/api/get?name={}&what=loc&apikey={}&format=json",
                opts.callsign,
                env!("APIKEY")
            ))
            .send()
            .await?
            .json::<location::Location>()
            .await?;
        println!("Got data: {}", res);
        println!("Inserting data into mongodb...");
        collection.insert_one(res, None).await?;
    }

    Ok(())
}
