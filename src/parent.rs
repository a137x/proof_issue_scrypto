use crate::child::child::*;
use scrypto::prelude::*;

#[blueprint]
mod parent {
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
        },
        methods {
            create_child => PUBLIC;
        }
    }

    #[derive(ScryptoSbor, Sbor)]
    struct Parent {
        /// Internal authority for minting package_badges and other resources
        minter: FungibleVault,

        /// Vault for storing package badge of this component.
        /// Package badge could be minted only by this Parent component.
        package_badge_vault: FungibleVault,

        /// Compoent address of child
        child: Option<ComponentAddress>,
    }

    impl Parent {
        pub fn new() -> ComponentAddress {
            // Internal authority
            let minter = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .mint_roles(mint_roles!(
                    minter => rule!(deny_all);
                    minter_updater => rule!(deny_all);
                ))
                .mint_initial_supply(1);

            let package_badge_manager = ResourceBuilder::new_fungible(OwnerRole::Fixed(rule!(
                require(minter.resource_address())
            )))
            .divisibility(DIVISIBILITY_NONE)
            .mint_roles(mint_roles! {
                minter => rule!(require(minter.resource_address()));
                minter_updater => rule!(require(minter.resource_address()));
            })
            .create_with_no_initial_supply();

            // Minting 1 package_badge for component
            LocalAuthZone::push(minter.create_proof_of_all());
            let package_badge_bucket: FungibleBucket =
                FungibleBucket(package_badge_manager.mint(1));
            LocalAuthZone::drop_regular_proofs();

            debug!("debug on new parent 0");

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Parent::blueprint_id());
            debug!("debug on new parent 1");

            let component = Self {
                minter: FungibleVault::with_bucket(minter),
                package_badge_vault: FungibleVault::with_bucket(package_badge_bucket),
                child: None,
            }
            .instantiate();
            debug!("debug on new parent 2");

            // Creating fee/royalty vault
            component.create_child();
            debug!("debug on new parent 3");

            debug!("debug on new parent 4");

            // Globolizing component
            let _globalized_component = component
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address_reservation)
                .globalize();

            component_address
        }

        /// Internal method for creating child of this parent
        /// Called only once on parent instantiation
        pub fn create_child(&mut self) {
            // Precaution, so this function never called again after parent init
            if let Some(_) = self.child {
                panic!("child already created!")
            }
            // Creating Child component
            let package_badge_bucket = self.mint_package_badge(Decimal::ONE);
            let fee_vault_component = Blueprint::<Child>::new(package_badge_bucket);
            self.child = Some(fee_vault_component);
        }

        /// Internal method for minting package_badges
        /// Returns bucket with badge.
        fn mint_package_badge(&self, amount: Decimal) -> FungibleBucket {
            debug!("debug mint package badge 0");
            debug!("resource of self.minter = {:?}", self.minter.resource_address());
            debug!("self.minter vault has {:?} amount", self.minter.amount());
            let package_badge_bucket = self
                .minter
                .as_fungible()
                .authorize_with_amount(Decimal::ONE, || {
                    self.package_badge_vault.resource_manager().mint(amount)
                });
            // panic on the previous line.
            debug!("we never reach this; debug mint package badge 1");
            FungibleBucket(package_badge_bucket)
        }
    }
}
