use scrypto::prelude::*;

#[blueprint]
mod child {
    struct Child {
        package_badge_vault: FungibleVault,
    }

    impl Child {
        /// Instantiating New Child component
        pub fn new(package_badge: FungibleBucket) -> ComponentAddress {
            let component = Self {
                package_badge_vault: FungibleVault::with_bucket(package_badge),
            }
            .instantiate();

            // Globolizing component
            let globalized_component = component.prepare_to_globalize(OwnerRole::None).globalize();

            globalized_component.address()
        }
    }
}
