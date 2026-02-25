/// Define the GovernanceProposal that will be used as part of on-chain governance by CreditChainGovernance.
///
/// This is separate from the CreditChainGovernance module to avoid circular dependency between CreditChainGovernance and Stake.
module creditchain_framework::governance_proposal {
    friend creditchain_framework::creditchain_governance;

    struct GovernanceProposal has store, drop {}

    /// Create and return a GovernanceProposal resource. Can only be called by CreditChainGovernance
    public(friend) fun create_proposal(): GovernanceProposal {
        GovernanceProposal {}
    }

    /// Useful for CreditChainGovernance to create an empty proposal as proof.
    public(friend) fun create_empty_proposal(): GovernanceProposal {
        create_proposal()
    }

    #[test_only]
    public fun create_test_proposal(): GovernanceProposal {
        create_empty_proposal()
    }
}
