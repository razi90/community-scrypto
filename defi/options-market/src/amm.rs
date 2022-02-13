use scrypto::prelude::*;

blueprint! {
    struct AutomaticMarketMaker {
        collateral_vault: Vault,
    }

    impl AutomaticMarketMaker {
        pub fn new() -> Component {
            let my_bucket: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .initial_supply_fungible(1000);

            Self {
                sample_vault: Vault::with_bucket(my_bucket)
            }
            .instantiate()
        }

        pub fn add_collateral(&mut self, collateral: Bucket) -> Bucket {
            // move collateral to vault
            // mint LP token for provider
            // return LP token to provider
        }

        pub fn remove_collateral(&mut self, lp_token: Bucket) -> Bucket {
            // calculate collateral amount based on lp token amount
            // burn lp token
            // take collateral from vault and return to lp provider
        }

        pub fn buy_option(&mut self, payment: Bucket) -> Bucket {
            // TODO
        }

        pub fn exercise_option(&mut self, option: Bucket) -> {
            // check if options is eligable for payout
                // condtions: option is not expired and is 'in the money'
            // if option eligable
                // calculate difference between strike price and pay option holder with asset from vault
                // burn option
            // else
                // throw error
        }
    }
}