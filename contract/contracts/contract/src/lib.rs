
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum Status {
    Submitted,
    Resolved,
    Rewarded,
}

#[contracttype]
#[derive(Clone)]
pub struct Report {
    pub reporter: Address,
    pub reward: i128,
    pub status: Status,
}

#[contracttype]
pub enum ReportKey {
    Report(u32),
    Count,
    Admin,
}

#[contract]
pub struct WhistleblowerReward;

#[contractimpl]
impl WhistleblowerReward {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&ReportKey::Admin, &admin);
        env.storage().instance().set(&ReportKey::Count, &0u32);
    }

    pub fn submit_report(env: Env, reporter: Address, reward: i128) -> u32 {
        reporter.require_auth();

        let mut count: u32 = env.storage().instance().get(&ReportKey::Count).unwrap_or(0);
        count += 1;

        let report = Report {
            reporter,
            reward,
            status: Status::Submitted,
        };

        env.storage().instance().set(&ReportKey::Report(count), &report);
        env.storage().instance().set(&ReportKey::Count, &count);

        count
    }

    pub fn resolve_report(env: Env, admin: Address, id: u32) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&ReportKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("not authorized");
        }

        let mut report: Report = env.storage().instance().get(&ReportKey::Report(id)).unwrap();
        report.status = Status::Resolved;

        env.storage().instance().set(&ReportKey::Report(id), &report);
    }

    pub fn release_reward(env: Env, admin: Address, id: u32) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&ReportKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("not authorized");
        }

        let mut report: Report = env.storage().instance().get(&ReportKey::Report(id)).unwrap();

        match report.status {
            Status::Resolved => {
                report.status = Status::Rewarded;
                env.storage().instance().set(&ReportKey::Report(id), &report);
            }
            _ => panic!("report not resolved"),
        }
    }

    pub fn get_report(env: Env, id: u32) -> Report {
        env.storage().instance().get(&ReportKey::Report(id)).unwrap()
    }
}

