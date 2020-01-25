use serde_json::{json, value};

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
