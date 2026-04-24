#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Map, Vec
};

#[contracttype]
#[derive(Clone)]
pub struct Project {
    pub name: Symbol,
    pub funds_allocated: i128,
}

#[contract]
pub struct OceanCleanupPool;

#[contractimpl]
impl OceanCleanupPool {

    // Donate to pool
    pub fn donate(env: Env, amount: i128) {
        if amount <= 0 {
            panic!("amount must be > 0");
        }

        let key = Symbol::short("TOTAL");

        let total: i128 = env.storage().instance()
            .get(&key)
            .unwrap_or(0);

        env.storage().instance().set(&key, &(total + amount));
    }

    // Add project
    pub fn add_project(env: Env, name: Symbol) {
        let proj_key = Symbol::short("PROJ");
        let count_key = Symbol::short("COUNT");

        let mut projects: Map<u32, Project> =
            env.storage().instance()
            .get(&proj_key)
            .unwrap_or(Map::new(&env));

        let count: u32 =
            env.storage().instance()
            .get(&count_key)
            .unwrap_or(0);

        let project = Project {
            name,
            funds_allocated: 0,
        };

        projects.set(count, project);

        env.storage().instance().set(&proj_key, &projects);
        env.storage().instance().set(&count_key, &(count + 1));
    }

    // Allocate funds
    pub fn allocate(env: Env, project_id: u32, amount: i128) {
        let total_key = Symbol::short("TOTAL");
        let proj_key = Symbol::short("PROJ");

        let mut total: i128 =
            env.storage().instance()
            .get(&total_key)
            .unwrap_or(0);

        if amount > total {
            panic!("not enough funds");
        }

        let mut projects: Map<u32, Project> =
            env.storage().instance()
            .get(&proj_key)
            .unwrap_or(Map::new(&env));

        let mut project = projects.get(project_id).unwrap();

        project.funds_allocated += amount;
        total -= amount;

        projects.set(project_id, project);

        env.storage().instance().set(&proj_key, &projects);
        env.storage().instance().set(&total_key, &total);
    }

    // Get total funds
    pub fn get_total(env: Env) -> i128 {
        env.storage().instance()
            .get(&Symbol::short("TOTAL"))
            .unwrap_or(0)
    }
}