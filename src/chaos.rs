use kube_async::client::APIClient;

pub async fn chaos_reigns(
    client: &APIClient,
    svc: &str,
    namespace: &str,
) -> Result<(), Box<dyn ::std::error::Error>> {
    let vs_exists = crate::istio::check_virtual_service(&client, &svc, &namespace).await?;
    if !vs_exists {
        println!("Creating Virtual Service for {}", svc);
        crate::istio::create_virtual_service(&client, &svc, &namespace).await?;
    }

    let dr_exists = crate::istio::check_destination_rule(&client, &svc, &namespace).await?;
    if !dr_exists {
        println!("Creating Destination Rule for {}", svc);
        crate::istio::create_destination_rule(&client, &svc, &namespace).await?;
    }

    let chaos = get_random_chaos();
    Ok(())
}

fn get_random_chaos() {}
