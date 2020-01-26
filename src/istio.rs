use kube_async::{
    api::{Api, DeleteParams, Object, PostParams},
    client::APIClient,
};
use serde::{Deserialize, Serialize};

type VirtualService = Object<VsSpec, VsStatus>;
type DestinationRule = Object<DrSpec, DrStatus>;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VsSpec {
    hosts: Vec<String>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct VsStatus {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct DrSpec {
    host: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct DrStatus {}

pub async fn create_destination_rule(
    client: &APIClient,
    namespace: &str,
    dr_json: serde_json::value::Value,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let dr: Api<DestinationRule> = Api::customResource(client.clone(), "destinationrules")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let o = dr
        .create(&PostParams::default(), serde_json::to_vec(&dr_json)?)
        .await?;
    println!("Created Destination Rule {}", o.metadata.name);
    Ok(())
}

pub async fn create_virtual_service(
    client: &APIClient,
    namespace: &str,
    vr_json: serde_json::value::Value,
) -> Result<(), Box<dyn ::std::error::Error>> {
    // https://github.com/istio/istio/blob/d42db2540105cec5cfd30425bf34c9899644022c/galley/testdatasets/conversion/dataset.gen.go#L509
    let vs: Api<VirtualService> = Api::customResource(client.clone(), "virtualservices")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let o = vs
        .create(&PostParams::default(), serde_json::to_vec(&vr_json)?)
        .await?;
    println!("Created Virtual Service {}", o.metadata.name);
    Ok(())
}

pub async fn delete_destination_rule(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    println!("Deleting Destination Rule for {}", svc);
    let dr: Api<DestinationRule> = Api::customResource(client.clone(), "destinationrules")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let _ = dr.delete(&svc, &DeleteParams::default()).await?;
    Ok(())
}

pub async fn delete_virtual_service(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    println!("Deleting Virtual Service for {}", svc);
    let vs: Api<VirtualService> = Api::customResource(client.clone(), "virtualservices")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let _ = vs.delete(&svc, &DeleteParams::default()).await?;
    Ok(())
}
