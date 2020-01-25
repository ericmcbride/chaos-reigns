use kube_async::client::APIClient;
use std::{thread, time};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = "Chaos Reigns",
    about = "Reign Chaos onto your Istio Service Mesh",
    author = "Eric McBride <ericmcbridedeveloper@gmail.com> github.com/ericmcbride"
)]
pub struct Opts {
    /// Randomize the svc and chaos to get attacked
    #[structopt(short, long)]
    pub random: bool,

    /// What kind of test is going to be ran
    #[structopt(short, long, required_unless("random"))]
    chaos_type: Option<String>,

    /// User needs to route via header this is the value
    #[structopt(long, requires("header-key"))]
    header_value: Option<String>,

    /// Header key to be passed to request in istio
    #[structopt(long, requires("header-value"))]
    header_key: Option<String>,

    /// Delay for fault injection ex: 7s
    #[structopt(short, long, required_if("chaos-type", "fault"))]
    delay: Option<String>,

    /// Percentage of traffic to be effected by Fault Injection ex 100
    #[structopt(short, long, required_if("chaos-type", "fault"))]
    traffic_percentage: Option<String>,

    /// What service should be attacked
    #[structopt(short, long, required_unless("random"))]
    pub service: Option<String>,

    /// Namespace of tests
    #[structopt(short, long)]
    pub namespace: String,
}

pub fn get_chaos_opts() -> Opts {
    let opt = Opts::from_args();
    opt
}

pub async fn chaos_reigns(
    client: &APIClient,
    svc: &str,
    opts: &Opts,
) -> Result<(), Box<dyn ::std::error::Error>> {
    println!("About to reign chaos on {}", svc);
    // make this end up getting random or based off opts
    let crd_name = format!("{}-chaos", &svc);

    // Do destination rules first if needed
    let vs_json = crate::fault::get_fault_injection_json_all_vs(&svc, &crd_name, &opts.namespace);
    let _ = crate::istio::create_virtual_service(&client, &opts.namespace, vs_json).await?;

    println!("We Waiting");
    let ten_millis = time::Duration::from_secs(60);
    thread::sleep(ten_millis);
    println!("Deleting VS");

    let _ = crate::istio::delete_virtual_service(&client, &crd_name, &opts.namespace).await?;
    Ok(())
}

fn get_random_chaos() {}
