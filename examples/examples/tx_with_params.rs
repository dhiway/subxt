use sp_keyring::AccountKeyring;
use subxt::config::cord::{Era, PlainTip, CordExtrinsicParamsBuilder as Params};
use subxt::{tx::PairSigner, OnlineClient, config::CordConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/cord_new.scale")]
pub mod cord {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<CordConfig>::new().await?;

    // Build a balance transfer extrinsic.
    let dest = AccountKeyring::Bob.to_account_id().into();
    let tx = cord::tx().balances().transfer(dest, 10_000);

    // Configure the transaction parameters; for Polkadot the tip and era:
    let tx_params = Params::new()
        .tip(PlainTip::new(1_000))
        .era(Era::Immortal, api.genesis_hash());

    // submit the transaction:
    let from = PairSigner::new(AccountKeyring::Alice.pair());
    let hash = api.tx().sign_and_submit(&tx, &from, tx_params).await?;
    println!("Balance transfer extrinsic submitted with hash : {hash}");

    Ok(())
}
