use kube_async::{client::APIClient, config};

mod chaos;
mod circuit;
mod fault;
mod istio;
mod kube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let config = config::load_kube_config().await.unwrap();
    let client = APIClient::new(config);
    let opts = chaos::get_chaos_opts();

    if opts.random {
        println!("random only partially implemented, feature coming soon");
        return Ok(());
    }

    //gotta figure this out
    let inner_opts = opts.clone();
    let svc = if opts.random {
        kube::get_random_svc(&client, &inner_opts.namespace).await?
    } else {
        inner_opts.service.unwrap()
    };

    chaos::chaos_reigns(&client, &svc, &opts).await?;
    Ok(())
}
