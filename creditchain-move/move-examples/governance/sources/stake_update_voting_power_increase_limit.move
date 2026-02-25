script {
    use creditchain_framework::creditchain_governance;
    use creditchain_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = creditchain_governance::resolve(proposal_id, @creditchain_framework);
        // Update voting power increase limit to 10%.
        staking_config::update_voting_power_increase_limit(&framework_signer, 10);
    }
}
