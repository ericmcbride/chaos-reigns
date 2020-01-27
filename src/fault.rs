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
        get_fault_injection_json_header_vs(&svc, &crd_name, &opts)
    } else {
        get_fault_injection_json_all_vs(&svc, &crd_name, &opts)
    };

    let _ = crate::istio::create_virtual_service(&client, &opts.namespace, vs_json).await?;
    Ok(())
}

// Fault injection doesnt use destinatin rules
pub fn get_fault_injection_json_all_vs(
    svc: &str,
    dr_name: &str,
    opts: &crate::chaos::Opts,
) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "VirtualService",
        "metadata": { "name": dr_name, "namespace": &opts.namespace},
        "spec": { "hosts": [svc],
            "http": [{
                "route": [{
                    "destination": {
                        "host": svc
                    }
                }],
                "fault": {
                    "delay": {
                        "fixedDelay": &opts.delay,
                        "percentage": {
                            "value": &opts.traffic_percentage,
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
    opts: &crate::chaos::Opts,
) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "VirtualService",
        "metadata": { "name": dr_name, "namespace": &opts.namespace},
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
                        "fixedDelay": &opts.delay,
                        "percentage": {
                            "value": &opts.traffic_percentage
                        }
                    }
                },
                "match": [{
                    "headers": {
                        opts.header_key.as_ref().unwrap(): {
                            "exact": opts.header_value.as_ref().unwrap()
                        }
                    }
                }]
            }]
        }
    });
}
