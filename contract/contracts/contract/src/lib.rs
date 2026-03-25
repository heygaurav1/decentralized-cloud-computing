#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComputeProvider {
    pub provider: Address,
    pub price_per_unit: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub id: u32,
    pub client: Address,
    pub provider: Address,
    pub description: String,
    pub compute_units: i128,
    pub completed: bool,
}

#[contracttype]
pub enum DataKey {
    Provider(Address),
    Job(u32),
    JobCount,
}

#[contract]
pub struct DecentralizedCloud;

#[contractimpl]
impl DecentralizedCloud {

    // Register as a compute provider
    pub fn register_provider(env: Env, provider: Address, price_per_unit: i128) {
        provider.require_auth();

        let provider_data = ComputeProvider {
            provider: provider.clone(),
            price_per_unit,
            active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Provider(provider), &provider_data);
    }

    // Submit a compute job
    pub fn submit_job(
        env: Env,
        client: Address,
        provider: Address,
        description: String,
        compute_units: i128,
    ) -> u32 {
        client.require_auth();

        let mut job_count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0);

        job_count += 1;

        let job = Job {
            id: job_count,
            client: client.clone(),
            provider: provider.clone(),
            description,
            compute_units,
            completed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Job(job_count), &job);

        env.storage()
            .instance()
            .set(&DataKey::JobCount, &job_count);

        job_count
    }

    // Mark job as completed by provider
    pub fn complete_job(env: Env, provider: Address, job_id: u32) {
        provider.require_auth();

        let mut job: Job = env
            .storage()
            .persistent()
            .get(&DataKey::Job(job_id))
            .unwrap_or_else(|| panic!("job not found"));

        if job.provider != provider {
            panic!("not assigned provider");
        }

        if job.completed {
            panic!("already completed");
        }

        job.completed = true;

        env.storage()
            .persistent()
            .set(&DataKey::Job(job_id), &job);
    }

    // Fetch job details
    pub fn get_job(env: Env, job_id: u32) -> Job {
        env.storage()
            .persistent()
            .get(&DataKey::Job(job_id))
            .unwrap_or_else(|| panic!("job not found"))
    }

    // Fetch provider details
    pub fn get_provider(env: Env, provider: Address) -> ComputeProvider {
        env.storage()
            .persistent()
            .get(&DataKey::Provider(provider))
            .unwrap_or_else(|| panic!("provider not found"))
    }

    // Total jobs created
    pub fn job_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::JobCount)
            .unwrap_or(0)
    }
}