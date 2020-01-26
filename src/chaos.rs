use kube_async::client::APIClient;
use structopt::StructOpt;
use std::io::{Error, ErrorKind};

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

    /// Kubernetes Namespace for tests to be ran on
    #[structopt(short, long, env = "CHAOS_NAMESPACE", hide_env_values = true)]
    pub namespace: String,

    /// What service should be attacked in the kubernetes namespace
    #[structopt(
        short,
        long,
        env = "CHAOS_SERVICE",
        required_unless("random"),
        hide_env_values = true
    )]
    pub service: Option<String>,

    /// Delete applied rules
    #[structopt(short, long)]
    pub delete: bool,

    /// What kind of Istio Fault tolerance is going to be ran
    #[structopt(
        short,
        long,
        env = "CHAOS_TYPE",
        required_unless("random"),
        required_unless("delete"),
        hide_env_values = true
    )]
    pub chaos_type: Option<String>,

    /// User needs to route via header this is the value
    #[structopt(
        long,
        env = "CHAOS_HEADER_VALUE",
        requires("header-key"),
        hide_env_values = true
    )]
    pub header_value: Option<String>,

    /// Header key to be passed to request in istio
    #[structopt(
        long,
        env = "CHAOS_HEADER_KEY",
        requires("header-value"),
        hide_env_values = true
    )]
    pub header_key: Option<String>,

    /// Simulated Delay for requests coming to envoy ex: 7s for Fault
    #[structopt(
        long,
        env = "CHAOS_DELAY",
        default_value = "7s",
        hide_env_values = true
    )]
    pub delay: String,

    /// Percentage of traffic ex 100 for Fault
    #[structopt(
        long,
        env = "CHAOS_TRAFFIC_PERCENTAGE",
        default_value = "100",
        hide_env_values = true
    )]
    pub traffic_percentage: String,

    /// Max TCP Connections for Circuit
    #[structopt(
        long,
        env = "CHAOS_MAX_CONNECTIONS",
        default_value = "1",
        hide_env_values = true
    )]
    pub max_connections: usize,

    /// Max Pending HTTP1 Requests for Circuit
    #[structopt(
        long,
        env = "CHAOS_MAX_PENDING_REQUESTS",
        default_value = "1",
        hide_env_values = true
    )]
    pub max_pending_reqs: usize,

    /// Setting to 1 disables keep-alive for Circuit
    #[structopt(
        long,
        env = "CHAOS_MAX_REQUESTS_PER_CONN",
        default_value = "1",
        hide_env_values = true
    )]
    pub max_requests_per_conn: usize,

    /// Number of 5xx before host is ejected for Circuit
    #[structopt(
        long,
        env = "CHAOS_CONSECUTIVE_ERRORS",
        default_value = "20",
        hide_env_values = true
    )]
    pub consecutive_errors: usize,

    /// Time interval between ejection sweep analysis for Circuit
    #[structopt(
        long,
        env = "CHAOS_INTERVAL",
        default_value = "1s",
        hide_env_values = true
    )]
    pub interval: String,

    /// Min amount of time host is ejected for Circuit
    #[structopt(
        long,
        env = "CHAOS_BASE_EJECT_TIME",
        default_value = "3m",
        hide_env_values = true
    )]
    pub base_eject_time: String,

    /// Max % of load balancer pool containers that can be ejected for Circuit
    #[structopt(
        long,
        env = "CHAOS_MAX_EJECT_PERCENT",
        default_value = "100",
        hide_env_values = true
    )]
    pub max_eject_percent: usize,
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
    let crd_name = format!("{}-chaos", &svc);

    // Always delete the virtual service first to make sure 503s dont happen
    if opts.delete {
        // Ignore errors here, to make it easier to delete stuff
        let _ = crate::istio::delete_virtual_service(&client, &crd_name, &opts.namespace).await;
        let _ = crate::istio::delete_destination_rule(&client, &crd_name, &opts.namespace).await;
        return Ok(());
    }

    println!("About to reign chaos on {}", svc);
    let _ = run_tests(&client, &svc, &crd_name, &opts).await?;
    Ok(())
}

async fn run_tests(
    client: &APIClient,
    svc: &str,
    crd_name: &str,
    opts: &Opts,
) -> Result<(), Box<dyn ::std::error::Error>> {
    match &opts.chaos_type.as_ref().unwrap()[..] {
        "fault" => crate::fault::run_fault_test(&client, &svc, &crd_name, &opts).await,
        "circuit" => crate::circuit::run_circuit_test(&client, &svc, &crd_name, &opts).await,
        _ => Err(Box::new(Error::new(ErrorKind::Other, "invalid chaos type"))),
    }
}

fn get_random_chaos() {}
