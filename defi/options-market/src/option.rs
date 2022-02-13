use scrypto::prelude::*;

blueprint! {
    struct Option {
        collateral_asset: String,
        payment_asset: String,
        strike_price: i32,
        expiration_date: i32,
        kind: String, //CALL or PUT
    }

    impl Option {
        pub fn new
        (
            collateral_asset: String,
            payment_asset: String,
            strike_price: i32,
            expiration_date: i32,
            kind: String,
        ) -> Component {
            let my_bucket: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "Option")
                .metadata("symbol", "OPT")
                .initial_supply_fungible(1);

            Self {
                collateral_asset: collateral_asset,
                payment_asset: payment_asset,
                strike_price: strike_price,
                expiration_date: expiration_date,
                kind: kind,
            }
            .instantiate()
        }
    }
}
