module 0xcafe::test {
    use creditchain_framework::coin::{Self, Coin};
    use creditchain_framework::creditchain_coin::CreditChainCoin;

    struct State has key {
        important_value: u64,
        coins: Coin<CreditChainCoin>,
    }

    fun init_module(s: &signer) {
        move_to(s, State {
            important_value: get_value(),
            coins: coin::zero<CreditChainCoin>(),
        })
    }

    fun get_value(): u64 {
        2
    }
}
