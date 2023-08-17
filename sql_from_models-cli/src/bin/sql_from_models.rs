use console::style;
use dotenv::dotenv;
use sql_from_models_cli::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // no special handling here
    if let Err(error) = sql_from_models_cli::run(Opt::from_args()).await {
        println!("{}: {}", style("error").bold().red(), error);
        std::process::exit(1);
    }
}
