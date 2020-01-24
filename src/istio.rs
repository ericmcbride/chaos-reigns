use kube_async::{
    api::{Api, DeleteParams, ListParams, Object, PostParams},
    client::APIClient,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

type VirtualService = Object<VsSpec, VsStatus>;
type DestinationRule = Object<DrSpec, DrStatus>;

#[derive(Clone, Debug, Default)]
struct RequestRouting {}
impl RequestRouting {}

#[derive(Clone, Debug, Default)]
struct FaultInjection {}
impl FaultInjection {}

#[derive(Clone, Debug, Default)]
struct TrafficShifting {}
impl TrafficShifting {}

#[derive(Clone, Debug, Default)]
struct RequestTimeouts {}
impl RequestTimeouts {}

#[derive(Clone, Debug, Default)]
struct CircuitBreaking {}
impl CircuitBreaking {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VsSpec {
    hosts: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
// Embed the Istio types in here
struct VsHttp {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VsRoute {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VsFault {}

#[derive(Clone, Default, Deserialize, Serialize)]
struct VsStatus {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct DrSpec {
    host: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct DrStatus {}

pub async fn check_destination_rule(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<bool, Box<dyn ::std::error::Error>> {
    let dr: Api<DestinationRule> = Api::customResource(client.clone(), "destinationrules")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let mut found = false;
    for d in dr.list(&ListParams::default()).await? {
        if svc == d.spec.host {
            found = true;
            break;
        }
    }
    Ok(found)
}

pub async fn check_virtual_service(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<bool, Box<dyn ::std::error::Error>> {
    let vs: Api<VirtualService> = Api::customResource(client.clone(), "virtualservices")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let mut found = false;
    for v in vs.list(&ListParams::default()).await? {
        if v.spec.hosts.iter().any(|h| h == svc) {
            found = true;
            break;
        }
    }
    Ok(found)
}

pub async fn create_destination_rule(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let dr_json = json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "DestinationRule",
        "metadata": { "name": svc, "namespace": namespace},
        "spec": { "host": svc},
    });

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
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    // Helper json found in istio repo:
    // https://github.com/istio/istio/blob/d42db2540105cec5cfd30425bf34c9899644022c/galley/testdatasets/conversion/dataset.gen.go#L509
    let vr_json = json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "VirtualService",
        "metadata": { "name": svc, "namespace": namespace},
        "spec": {"hosts": [svc], "http": [{ "route": [{ "destination": { "host": svc}},]}]}
    });

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
    let vs: Api<VirtualService> = Api::customResource(client.clone(), "virtualservices")
        .version("v1alpha3")
        .group("networking.istio.io")
        .within(namespace);

    let _ = vs.delete(&svc, &DeleteParams::default()).await?;
    Ok(())
}

pub async fn patch_destination_rule(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    Ok(())
}

pub async fn patch_virtual_service(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    Ok(())
}
