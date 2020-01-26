use kube_async::client::APIClient;
use serde_json::{json, value};

// Publicly exposed functin
pub async fn run_fault_test(
    client: &APIClient,
    svc: &str,
    crd_name: &str,
    opts: &crate::chaos::Opts,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let vs_json = if opts.header_value.is_some() {
        get_fault_injection_json_header_vs(
            &svc,
            &crd_name,
            &opts.namespace,
            &opts.header_value.as_ref().unwrap(),
            &opts.header_key.as_ref().unwrap(),
        )
    } else {
        get_fault_injection_json_all_vs(&svc, &crd_name, &opts.namespace)
    };

    let _ = crate::istio::create_virtual_service(&client, &opts.namespace, vs_json).await?;
    Ok(())
}

// Fault injection doesnt use destinatin rules
pub fn get_fault_injection_json_all_vs(svc: &str, dr_name: &str, namespace: &str) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "VirtualService",
        "metadata": { "name": dr_name, "namespace": namespace},
        "spec": {
            "hosts": [svc],
            "http": [{
                "route": [{
                    "destination": {
                        "host": svc
                    }
                }],
                "fault": {
                    "delay": {
                        "fixedDelay": "7s",
                        "percentage": {
                            "value": 100
                        }
                    }
                }
            }]
        }
    });
}

fn get_fault_injection_json_header_vs(
    svc: &str,
    dr_name: &str,
    namespace: &str,
    header: &str,
    header_name: &str,
) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "DestinationRule",
        "metadata": { "name": dr_name, "namespace": namespace},
        "spec": {
            "hosts": [svc],
            "http": [{
                "route": [{
                    "destination": {
                        "host": svc
                    }
                }],
                "fault": {
                    "delay": {
                        "fixedDelay": "7s",
                        "percentage": {
                            "value": "100"
                        }
                    }
                },
                "match": [{
                    "headers": {
                        header_name: {
                            "exact": header
                        }
                    }
                }]
            }]
        }
    });
}
