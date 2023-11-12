use std::net::IpAddr;
use tracing::info;

pub async fn get_address() -> (IpAddr, u16) {
    // could be improve by getting the ip and port from the host system
    let host_ip_address =
        std::env::var("WEBSERVER_IP").unwrap_or_else(|_| panic!("WEBSERVER_IP must be set!"));
    let host_port =
        std::env::var("WEBSERVER_PORT").unwrap_or_else(|_| panic!("WEBSERVER_PORT must be set!"));

    let ip_address = host_ip_address
        .parse::<IpAddr>()
        .expect("IP Adress invalid");
    let port = host_port.parse::<u16>().expect("PORT is invalid.");

    let url_message = format!(
        "executed: initializing server url = {}:{}",
        &ip_address, &port
    );
    info!(url_message);

    (ip_address, port)
}
