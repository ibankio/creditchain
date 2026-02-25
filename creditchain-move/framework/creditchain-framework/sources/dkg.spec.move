spec creditchain_framework::dkg {

    spec module {
        use creditchain_framework::chain_status;
        invariant [suspendable] chain_status::is_operating() ==> exists<DKGState>(@creditchain_framework);
    }

    spec initialize(creditchain_framework: &signer) {
        use std::signer;
        let creditchain_framework_addr = signer::address_of(creditchain_framework);
        aborts_if creditchain_framework_addr != @creditchain_framework;
    }

    spec start(
        dealer_epoch: u64,
        randomness_config: RandomnessConfig,
        dealer_validator_set: vector<ValidatorConsensusInfo>,
        target_validator_set: vector<ValidatorConsensusInfo>,
    ) {
        aborts_if !exists<DKGState>(@creditchain_framework);
        aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@creditchain_framework);
    }

    spec finish(transcript: vector<u8>) {
        use std::option;
        requires exists<DKGState>(@creditchain_framework);
        requires option::is_some(global<DKGState>(@creditchain_framework).in_progress);
        aborts_if false;
    }

    spec fun has_incomplete_session(): bool {
        if (exists<DKGState>(@creditchain_framework)) {
            option::spec_is_some(global<DKGState>(@creditchain_framework).in_progress)
        } else {
            false
        }
    }

    spec try_clear_incomplete_session(fx: &signer) {
        use std::signer;
        let addr = signer::address_of(fx);
        aborts_if addr != @creditchain_framework;
    }

    spec incomplete_session(): Option<DKGSessionState> {
        aborts_if false;
    }
}
