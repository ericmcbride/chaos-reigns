use kube_async::{
    api::{Api, ListParams},
    client::APIClient,
};

use rand::{thread_rng, Rng};

pub async fn get_random_svc(
    client: &APIClient,
    namespace: &str,
) -> Result<String, Box<dyn ::std::error::Error>> {
    let mut rng = rand::thread_rng();
    let svc = Api::v1Service(client.clone()).within(&namespace);
    let svc_list = svc.list(&ListParams::default()).await?;
    let random = rng.gen_range(0, svc_list.items.len() - 1);

    Ok(svc_list.items[random].metadata.name.to_string())
}
