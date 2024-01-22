pub fn get_modelservice_endpoint() -> String {
    let modelservice_endpoint =
        std::env::var("MODELSERVICE_ENDPOINT").unwrap_or_else(|_| panic!("MODELSERVICE_ENDPOINT must be set!"));
    modelservice_endpoint
}
