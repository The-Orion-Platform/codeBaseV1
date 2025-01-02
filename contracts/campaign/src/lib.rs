#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, contracterror, TryFromVal, IntoVal, TryIntoVal};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidMilestonePercentages = 1,
    CampaignInactive = 2,
    MilestoneAlreadyCompleted = 3,
    MilestoneNotCompleted = 4,
    MilestoneAlreadyApproved = 5,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum DataKey {
    CampaignData = 1,
}

#[contract]
pub struct Campaign;

#[derive(Clone, Debug)]
pub struct Milestone {
    pub percentage: u32,
    pub completed: bool,
    pub approved: bool,
}

impl IntoVal<Env, soroban_sdk::Val> for Milestone {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        (self.percentage, self.completed, self.approved).into_val(env)
    }
}

impl TryFromVal<Env, soroban_sdk::Val> for Milestone {
    type Error = soroban_sdk::Error;
    fn try_from_val(env: &Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        let (percentage, completed, approved): (u32, bool, bool) = TryIntoVal::try_into_val(val, env)?;
        Ok(Self { percentage, completed, approved })
    }
}

#[derive(Clone, Debug)]
pub struct CampaignData {
    pub creator: Address,
    pub admin: Address,
    pub target_amount: i128,
    pub current_amount: i128,
    pub milestones: Vec<Milestone>,
    pub is_active: bool,
}

impl IntoVal<Env, soroban_sdk::Val> for CampaignData {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        (
            &self.creator.clone(),
            &self.admin.clone(),
            self.target_amount,
            self.current_amount,
            self.milestones.clone(),
            self.is_active,
        )
            .into_val(env)
    }
}

impl TryFromVal<Env, soroban_sdk::Val> for CampaignData {
    type Error = soroban_sdk::Error;
    fn try_from_val(env: &Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        let (creator, admin, target_amount, current_amount, milestones, is_active): (
            Address,
            Address,
            i128,
            i128,
            Vec<Milestone>,
            bool,
        ) = TryIntoVal::try_into_val(val, env)?;
        Ok(Self {
            creator,
            admin,
            target_amount,
            current_amount,
            milestones,
            is_active,
        })
    }
}

impl IntoVal<Env, soroban_sdk::Val> for DataKey {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        (*self as u32).into_val(env)
    }
}

impl TryFromVal<Env, soroban_sdk::Val> for DataKey {
    type Error = soroban_sdk::Error;
    fn try_from_val(_env: &Env, _val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        Ok(Self::CampaignData)
    }
}

#[contractimpl]
impl Campaign {
    pub fn initialize(
        env: Env,
        creator: Address,
        admin: Address,
        target_amount: i128,
        milestone_percentages: Vec<u32>,
    ) -> Result<(), Error> {
        creator.require_auth();
        admin.require_auth();

        let total: u32 = milestone_percentages.iter().sum();
        if total != 100 {
            return Err(Error::InvalidMilestonePercentages);
        }

        let mut milestones: Vec<Milestone> = Vec::new(&env);
        for percentage in milestone_percentages.iter() {
            milestones.push_back(Milestone {
                percentage: percentage,
                completed: false,
                approved: false,
            });
        }

        let campaign_data = CampaignData {
            creator,
            admin,
            target_amount,
            current_amount: 0,
            milestones,
            is_active: true,
        };

        env.storage().instance().set(&DataKey::CampaignData, &campaign_data);
        Ok(())
    }

    pub fn donate(
        env: Env,
        donor: Address,
        amount: i128,
    ) -> Result<(), Error> {
        donor.require_auth();

        let mut data: CampaignData = env.storage().instance().get(&DataKey::CampaignData).unwrap();
        if !data.is_active {
            return Err(Error::CampaignInactive);
        }

        data.current_amount += amount;
        env.storage().instance().set(&DataKey::CampaignData, &data);
        Ok(())
    }

    pub fn complete_milestone(
        env: Env,
        milestone_index: u32,
    ) -> Result<(), Error> {
        let mut data: CampaignData = env.storage().instance().get(&DataKey::CampaignData).unwrap();
        data.creator.require_auth();

        let milestone = data.milestones.get(milestone_index).unwrap();
        if milestone.completed {
            return Err(Error::MilestoneAlreadyCompleted);
        }

        data.milestones.set(
            milestone_index,
            Milestone {
                completed: true,
                ..milestone
            },
        );

        env.storage().instance().set(&DataKey::CampaignData, &data);
        Ok(())
    }

    pub fn approve_milestone(
        env: Env,
        milestone_index: u32,
    ) -> Result<(), Error> {
        let mut data: CampaignData = env.storage().instance().get(&DataKey::CampaignData).unwrap();
        data.admin.require_auth();

        let milestone = data.milestones.get(milestone_index).unwrap();
        if !milestone.completed {
            return Err(Error::MilestoneNotCompleted);
        }
        if milestone.approved {
            return Err(Error::MilestoneAlreadyApproved);
        }

        data.milestones.set(
            milestone_index,
            Milestone {
                approved: true,
                ..milestone
            },
        );

        env.storage().instance().set(&DataKey::CampaignData, &data);
        Ok(())
    }

    pub fn get_campaign_details(env: Env) -> CampaignData {
        env.storage().instance().get(&DataKey::CampaignData).unwrap()
    }
}

#[cfg(test)]
mod test;