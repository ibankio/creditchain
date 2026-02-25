module aa::test_functions {
    use creditchain_framework::creditchain_account;

    /// test function for multi-agent aa.
    public entry fun transfer_to_the_last(a: &signer, b: &signer, c: &signer, d: address) {
        creditchain_account::transfer(a, d, 1);
        creditchain_account::transfer(b, d, 1);
        creditchain_account::transfer(c, d, 1);
    }
}
