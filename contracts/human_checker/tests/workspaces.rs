use anyhow::Ok;
use near_sdk::json_types::Base64VecU8;
use near_units::parse_near;
use sbt::TokenMetadata;
use serde_json::json;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const REGISTER_HUMAN_TOKEN: &'static str = "register_human_token";

async fn init(
    worker: &Worker<Sandbox>,
) -> anyhow::Result<(
    Contract,
    Contract,
    Account,
    Account,
    Account,
    Account,
    Account,
)> {
    // import the contract from mainnet
    let registry = worker
        .dev_deploy(include_bytes!("../../res/registry.wasm"))
        .await?;
    let human_checker = worker
        .dev_deploy(include_bytes!("../../res/human_checker.wasm"))
        .await?;

    let authority_acc = worker.dev_create_account().await?;
    let iah_issuer = worker.dev_create_account().await?;
    let og_issuer = worker.dev_create_account().await?;
    let alice_acc = worker.dev_create_account().await?;
    let bob_acc = worker.dev_create_account().await?;
    let john_acc = worker.dev_create_account().await?;
    let elon_acc = worker.dev_create_account().await?;

    // init the contracts
    let res1 = registry
        .call("new")
        .args_json(json!({"authority": authority_acc.id(), "iah_issuer": iah_issuer.id(), "iah_classes": [1]}))
        .max_gas()
        .transact();

    let res2 = human_checker
        .call("new")
        .args_json(json!({"registry": registry.id()}))
        .max_gas()
        .transact();

    assert!(res1.await?.is_success() && res2.await?.is_success());

    // populate registry with mocked data
    let token_metadata = vec![TokenMetadata {
        class: 1,
        issued_at: Some(0),
        expires_at: None,
        reference: None,
        reference_hash: None,
    }];

    let iah_token_spec = vec![
        (alice_acc.id(), token_metadata.clone()),
        (bob_acc.id(), token_metadata.clone()),
        (john_acc.id(), token_metadata.clone()),
        (elon_acc.id(), token_metadata.clone()),
    ];

    let res = iah_issuer
        .call(registry.id(), "sbt_mint")
        .args_json(json!({ "token_spec": iah_token_spec }))
        .deposit(parse_near!("1 N"))
        .max_gas()
        .transact()
        .await?;

    return Ok((
        registry,
        human_checker.clone(),
        alice_acc,
        bob_acc,
        john_acc,
        elon_acc,
        iah_issuer,
    ));
}

#[tokio::test]
async fn is_human_call() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let (registry, human_checker, alice, bob, john, elon, iah_issuer) = init(&worker).await?;

    let tokens = vec![(iah_issuer.id(), vec![1])];
    let args = serde_json::to_vec(&json!({"user": alice.id(), "tokens": tokens})).unwrap();
    let args_base64: Base64VecU8 = args.into();

    // call the is_human_call method
    let res: bool = registry
        .call("is_human_call")
        .args_json(json!({"account": alice.id(), "ctr": human_checker.id(), "function": REGISTER_HUMAN_TOKEN, "args": args_base64}))
        .max_gas()
        .transact()
        .await?
        .json()?;

    Ok(())
}