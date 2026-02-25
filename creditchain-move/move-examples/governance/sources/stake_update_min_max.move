script {
    use creditchain_framework::creditchain_governance;
    use creditchain_framework::coin;
    use creditchain_framework::creditchain_coin::CreditChainCoin;
    use creditchain_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = creditchain_governance::resolve(proposal_id, @creditchain_framework);
        let one_creditchain_coin_with_decimals = 10 ** (coin::decimals<CreditChainCoin>() as u64);
        // Change min to 1000 and max to 1M CreditChain coins.
        let new_min_stake = 1000 * one_creditchain_coin_with_decimals;
        let new_max_stake = 1000000 * one_creditchain_coin_with_decimals;
        staking_config::update_required_stake(&framework_signer, new_min_stake, new_max_stake);
    }
}
