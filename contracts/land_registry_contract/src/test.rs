#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Bytes};

    // Test 1: Happy Path - Admin registers a title and the owner successfully transfers it.
    #[test]
    fn test_happy_path_transfer() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let juan = Address::generate(&env);
        let cooperative = Address::generate(&env);

        let contract_id = env.register_contract(None, LandRegistryContract);
        let client = LandRegistryContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let location = Bytes::from_slice(&env, b"Baliwag, Central Luzon GPS 14.95, 120.90");
        client.register_title(&101, &juan, &location, &2500);

        // Execute MVP core feature transaction
        client.transfer_title(&101, &juan, &cooperative);

        let registered_title = client.get_title(&101);
        assert_eq!(registered_title.owner, cooperative);
    }

    // Test 2: Edge Case - An unauthorized non-admin entity attempts to register a property title.
    #[test]
    #[should_panic]
    fn test_unauthorized_registration_fails() {
        let env = Env::default();
        // Do not mock auths to enforce explicit verification checks failing
        let admin = Address::generate(&env);
        let malicious_user = Address::generate(&env);

        let contract_id = env.register_contract(None, LandRegistryContract);
        let client = LandRegistryContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let location = Bytes::from_slice(&env, b"Illegal Plot");
        // Should trigger standard auth panic since malicious_user is calling an admin function
        client.register_title(&202, &malicious_user, &location, &100);
    }

    // Test 3: State Verification - Asserts that persistent storage tracks data accurately after registration.
    #[test]
    fn test_state_verification() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let juan = Address::generate(&env);

        let contract_id = env.register_contract(None, LandRegistryContract);
        let client = LandRegistryContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let location = Bytes::from_slice(&env, b"Zone 4, Baliwag");
        client.register_title(&303, &juan, &location, &1200);

        let state = client.get_title(&303);
        assert_eq!(state.owner, juan);
        assert_eq!(state.area_sqm, 1200);
        assert_eq!(state.location, location);
    }

    // Test 4: Edge Case - Double registration under an existing Title ID triggers a panic.
    #[test]
    #[should_panic(expected = "Title ID already registered.")]
    fn test_duplicate_registration_fails() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user_a = Address::generate(&env);
        let user_b = Address::generate(&env);

        let contract_id = env.register_contract(None, LandRegistryContract);
        let client = LandRegistryContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let location = Bytes::from_slice(&env, b"Duplicate Land Plot");
        client.register_title(&404, &user_a, &location, &500);
        client.register_title(&404, &user_b, &location, &600);
    }

    // Test 5: Edge Case - An external account attempts to transfer land they do not own.
    #[test]
    #[should_panic(expected = "Caller is not the recognized title owner.")]
    fn test_non_owner_transfer_fails() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let genuine_owner = Address::generate(&env);
        let bad_actor = Address::generate(&env);
        let buyer = Address::generate(&env);

        let contract_id = env.register_contract(None, LandRegistryContract);
        let client = LandRegistryContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let location = Bytes::from_slice(&env, b"Protected Rice Field");
        client.register_title(&505, &genuine_owner, &location, &3000);

        // Bad actor attempts to pass their own address as the source authority for title 505
        client.transfer_title(&505, &bad_actor, &buyer);
    }
}