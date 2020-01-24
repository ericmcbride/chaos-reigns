use kube_async::{client::APIClient, config};

use structopt::StructOpt;

mod chaos;
mod istio;
mod kube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let config = config::load_kube_config().await.unwrap();
    let client = APIClient::new(config);
    let opts = get_chaos_opts();

    // add conditionals later on
    // if svc is passed in on command line, need to make sure service exists
    let svc = kube::get_random_svc(&client, &opts.namespace).await?;
    println!("Service is {:?}", svc);
    chaos::chaos_reigns(&client, &svc, &opts.namespace).await?;

    Ok(())
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = "Chaos Reigns",
    about = "Reign Chaos onto your Istio Service Mesh",
    author = "Eric McBride <ericmcbridedeveloper@gmail.com> github.com/ericmcbride"
)]
pub struct Opts {
    /// Randomize the pod to get attacked
    #[structopt(short, long)]
    random: bool,

    // Make an enum or tyhpe for chaos_type
    /// What kind of test is going to be ran
    //#[structopt(short, long)]
    //chaos_type: String,

    /// What service should be attacked
    //#[structopt(short, long)]
    //service: String,

    /// Namespace of tests
    #[structopt(short, long)]
    namespace: String,
}

fn get_chaos_opts() -> Opts {
    let opt = Opts::from_args();
    opt
}
