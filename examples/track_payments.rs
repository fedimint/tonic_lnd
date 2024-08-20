// This example connects to LND and uses router rpc to track a payment.
//
// The program accepts four arguments: address, cert file, macaroon file, payment hash
// The address must start with `https://`!
//
// Example run: `cargo run --features=routerrpc --example track_payments <address> <tls.cert> <file.macaroon> <payment_hash>`

#[tokio::main]
#[cfg(feature = "routerrpc")]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address: String = args
        .next()
        .expect("missing arguments: address, macaroon file, payment hash")
        .into_string()
        .expect("address is not UTF-8");
    let cert_file: String = args
        .next()
        .expect("missing arguments: cert file, macaroon file, payment hash")
        .into_string()
        .expect("cert_file is not UTF-8");
    let macaroon_file: String = args
        .next()
        .expect("missing argument: macaroon file, payment hash")
        .into_string()
        .expect("macaroon_file is not UTF-8");

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = fedimint_tonic_lnd::connect(address, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    match client
        .router()
        .track_payments(fedimint_tonic_lnd::routerrpc::TrackPaymentsRequest {
            no_inflight_updates: true,
        })
        .await
    {
        Ok(response) => {
            let mut response = response.into_inner();
            loop {
                println!("Waiting for payment...");
                match response.message().await {
                    Ok(Some(payment)) => {
                        println!("Got payment: {:#?}", payment);
                    }
                    Ok(None) => {
                        println!("Payments stream ended");
                    }
                    Err(e) => {
                        eprintln!("Error reading payment stream: {e:?}");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error subscribing to payments stream: {e:?}");
        }
    }
}
