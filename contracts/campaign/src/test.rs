#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use soroban_sdk::{vec, Address, Env, IntoVal};

#[test]
fn test_campaign_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Campaign);
    let client = CampaignClient::new(&env, &contract_id);

    // Setup accounts
    let creator = Address::generate(&env);
    let admin = Address::generate(&env);
    let donor = Address::generate(&env);

    // Initialize campaign with 3 milestones (30%, 40%, 30%)
    let milestone_percentages = vec![&env, 30, 40, 30];
    client.initialize(&creator, &admin, &100, &milestone_percentages);

    // Donor contributes
    client.donate(&donor, &100);

    // Complete and approve milestones
    client.complete_milestone(&0);
    client.approve_milestone(&0);

    // Verify campaign state
    let campaign_data = client.get_campaign_details();
    assert_eq!(campaign_data.current_amount, 100);
    assert!(campaign_data.milestones.get(0).unwrap().approved);
}