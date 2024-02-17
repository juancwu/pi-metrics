// mod temperature;

use std::env;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // these two have to be defined! (for now at least)
    let url_string = env::var("LIBSQL_CLIENT_URL").unwrap();
    let db_token = env::var("LIBSQL_CLIENT_TOKEN").unwrap();
    let config = libsql_client::Config {
        url: url::Url::parse(&url_string).unwrap(),
        auth_token: db_token,
    };

    db.execute("CREATE TABLE IF NOT EXISTS metrics (id INTEGER PRIMARY KEY AUTOINCREMENT, temperature FLOAT);")
        .await.unwrap();

    // let cpu_temp = temperature::get_cpu_temp().expect("Failed to get CPU temperature");
    //
    println!("CPU Temperature: C");
}
