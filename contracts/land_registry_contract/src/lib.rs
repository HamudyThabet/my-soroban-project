#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Bytes};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TitleDetails {
    pub owner: Address,
    pub location: Bytes,
    pub area_sqm: u32,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Title(u32),
}

#[contract]
pub struct LandRegistryContract;

#[contractimpl]
impl LandRegistryContract {
    /// Initializes the contract by setting a designated municipal administrator.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract is already initialized.");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Registers a brand new digital land title. Only the authorized administrator can execute this.
    pub fn register_title(env: Env, title_id: u32, owner: Address, location: Bytes, area_sqm: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Contract uninitialized");
        admin.require_auth();

        let key = DataKey::Title(title_id);
        if env.storage().persistent().has(&key) {
            panic!("Title ID already registered.");
        }

        let details = TitleDetails {
            owner,
            location,
            area_sqm,
        };
        env.storage().persistent().set(&key, &details);
    }

    /// Safely updates ownership mapping from one entity to another. Requires authentication from the current owner.
    pub fn transfer_title(env: Env, title_id: u32, from: Address, to: Address) {
        from.require_auth();

        let key = DataKey::Title(title_id);
        if !env.storage().persistent().has(&key) {
            panic!("Specified land title does not exist.");
        }

        let mut details: TitleDetails = env.storage().persistent().get(&key).unwrap();
        if details.owner != from {
            panic!("Caller is not the recognized title owner.");
        }

        details.owner = to;
        env.storage().persistent().set(&key, &details);
    }

    /// Read-only method to fetch the absolute state details of a registered land title.
    pub fn get_title(env: Env, title_id: u32) -> TitleDetails {
        let key = DataKey::Title(title_id);
        if !env.storage().persistent().has(&key) {
            panic!("Specified land title does not exist.");
        }
        env.storage().persistent().get(&key).unwrap()
    }
}