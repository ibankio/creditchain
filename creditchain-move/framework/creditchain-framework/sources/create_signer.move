/// Provides a common place for exporting `create_signer` across the CreditChain Framework.
///
/// To use create_signer, add the module below, such that:
/// `friend creditchain_framework::friend_wants_create_signer`
/// where `friend_wants_create_signer` is the module that needs `create_signer`.
///
/// Note, that this is only available within the CreditChain Framework.
///
/// This exists to make auditing straight forward and to limit the need to depend
/// on account to have access to this.
module creditchain_framework::create_signer {
    friend creditchain_framework::account;
    friend creditchain_framework::creditchain_account;
    friend creditchain_framework::coin;
    friend creditchain_framework::fungible_asset;
    friend creditchain_framework::genesis;
    friend creditchain_framework::account_abstraction;
    friend creditchain_framework::multisig_account;
    friend creditchain_framework::object;
    friend creditchain_framework::permissioned_signer;
    friend creditchain_framework::transaction_validation;

    public(friend) native fun create_signer(addr: address): signer;
}
