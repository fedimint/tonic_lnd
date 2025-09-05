use std::path::PathBuf;
use tonic_build::Config;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=LND_REPO_DIR");
    let dir = match std::env::var_os("LND_REPO_DIR") {
        Some(lnd_repo_path) => {
            let mut lnd_rpc_dir = PathBuf::from(lnd_repo_path);
            lnd_rpc_dir.push("lnrpc");
            lnd_rpc_dir
        }
        None => PathBuf::from("vendor"),
    };

    let lnd_rpc_proto_file = dir.join("lightning.proto");
    println!("cargo:rerun-if-changed={}", lnd_rpc_proto_file.display());

    let protos = [
        "signrpc/signer.proto",
        "walletrpc/walletkit.proto",
        "lightning.proto",
        "peersrpc/peers.proto",
        "verrpc/verrpc.proto",
        "routerrpc/router.proto",
        "invoicesrpc/invoices.proto",
        "staterpc/state.proto",
    ];

    let proto_paths: Vec<_> = protos.iter().map(|proto| dir.join(proto)).collect();

    let mut conf = Config::default();
    conf.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_protos_with_config(conf, &proto_paths, &[dir])?;
    Ok(())
}
