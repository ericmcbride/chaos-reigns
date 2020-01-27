use kube_async::client::APIClient;
use serde_json::{json, value};

pub async fn run_mirror_test(
    client: &APIClient,
    svc: &str,
    crd_name: &str,
    opts: &crate::chaos::Opts,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let dr_json = get_mirror_dr(&svc, &crd_name, &opts);
    let _ = crate::istio::create_destination_rule(&client, &opts.namespace, dr_json).await?;

    let vs_json = get_mirror_vs(&svc, &crd_name, &opts);
    let _ = crate::istio::create_virtual_service(&client, &opts.namespace, vs_json).await?;

    Ok(())
}

fn get_mirror_vs(svc: &str, vs_name: &str, opts: &crate::chaos::Opts) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "VirtualService",
        "metadata": { "name": vs_name, "namespace": &opts.namespace},
        "spec": { "hosts": [svc],
            "http": [{
                "route": [{
                    "destination": {
                        "host": svc,
                        "subset": &opts.first_deploy_subset_value
                    },
                    "weight": &opts.traffic_percentage
                }],
                "mirror": {
                    "host": svc,
                    "subset": &opts.second_deploy_subset_value,
                },
                "mirror_percent": &opts.traffic_percentage,
            }]
        }
    });
}

fn get_mirror_dr(svc: &str, dr_name: &str, opts: &crate::chaos::Opts) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "DestinationRule",
        "metadata": { "name": dr_name, "namespace": &opts.namespace},
        "spec": {
            "host": svc,
            "subsets": [{
                "name": &opts.first_deploy_subset_value,
                "labels": {
                    &opts.first_deploy_key_value: &opts.first_deploy_subset_value
                },
            },{
                "name": &opts.second_deploy_subset_value,
                "labels": {
                    &opts.second_deploy_key_value: &opts.second_deploy_subset_value
                }
            }]
        }
    });
}
