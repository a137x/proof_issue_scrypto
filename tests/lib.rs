use radix_engine::vm::NativeVmExtension;

use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
/// Test instantiation of parent
fn test_instatiation_parent() {
    let mut test_runner = TestRunnerBuilder::new().without_trace().build();
    setup_fixture(&mut test_runner);
}

fn setup_fixture<E: NativeVmExtension, D: TestDatabase>(test_runner: &mut TestRunner<E, D>) {
    // Create one user
    let (public_key_1, _private_key, account_1) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());
    // let package_address = test_runner.publish_package(
    //     include_bytes!("../target/wasm32-unknown-unknown/release/proof_issue_scrypto.wasm").to_vec(),
    //     manifest_decode(include_bytes!(
    //         "../target/wasm32-unknown-unknown/release/proof_issue_scrypto.rpd"
    //     ))
    //     .unwrap(),
    //     btreemap!(),
    //     OwnerRole::None,
    // );

    instantiate_parent(test_runner, public_key_1, account_1, package_address);

    fn instantiate_parent<E: NativeVmExtension, D: TestDatabase>(
        test_runner: &mut TestRunner<E, D>,
        public_key_1: Secp256k1PublicKey,
        account_address: ComponentAddress,
        package_address: PackageAddress,
    ) {
        println!("\nTest Parent component `new` function.\n");
        let manifest = ManifestBuilder::new()
            .call_function(package_address, "Parent", "new", manifest_args!())
            .call_method(
                account_address,
                "deposit_batch",
                manifest_args!(ManifestExpression::EntireWorktop),
            )
            .build();
        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key_1)],
        );
        println!("{:?}\n", receipt);
        let _commit_result = receipt.expect_commit_success();
    }
}
