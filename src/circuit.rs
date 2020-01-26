use kube_async::client::APIClient;
use serde_json::{json, value};

pub async fn run_circuit_test(
    client: &APIClient,
    svc: &str,
    crd_name: &str,
    opts: &crate::chaos::Opts,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let dr_json = get_circuit_breaker_json_dr(&svc, &crd_name, &opts);

    let _ = crate::istio::create_destination_rule(&client, &opts.namespace, dr_json).await?;
    Ok(())
}
// Circuit breaker only utilizes a destination rule
pub fn get_circuit_breaker_json_dr(
    svc: &str,
    dr_name: &str,
    opt: &crate::chaos::Opts,
) -> value::Value {
    return json!({
        "apiVersion": "networking.istio.io/v1alpha3",
        "kind": "DestinationRule",
        "metadata": { "name": dr_name, "namespace": &opt.namespace},
        "spec": {
            "host": svc,
            "trafficPolicy": {
                "connectionPool": {
                    "tcp": {
                        "maxConnections": &opt.max_connections
                    },
                    "http": {
                        "http1MaxPendingRequests": &opt.max_pending_reqs,
                        "maxRequestsPerConnection": &opt.max_requests_per_conn,
                    },
                },
                "outlierDetection": {
                    "consecutiveErrors": &opt.consecutive_errors,
                    "interval": &opt.interval,
                    "baseEjectionTime": &opt.base_eject_time,
                    "maxEjectionPercent": &opt.max_eject_percent,
                }
            },
        }
    });
}
