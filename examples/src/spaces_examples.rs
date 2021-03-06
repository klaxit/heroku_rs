extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::space;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    // create_space(api_client);
    // update_space(api_client);
    // get_space(api_client);
    get_spaces(api_client);
    // delete_space(api_client);

    // get_space_access(api_client);
    // get_space_access_members(api_client);
    // update_space_access(api_client);

    // get_space_nat(api_client);

    // create_space_transfer (api_client);

    // get_inbound_ruleset_list(api_client);
    // get_inbound_ruleset_current(api_client);
    // get_inbound_ruleset_details(api_client);
    // create_inbound_ruleset(api_client);

    // get_outbound_ruleset_list(api_client);
    // get_outbound_ruleset_current(api_client);
    // get_outbound_ruleset_details(api_client);
    // create_inbound_ruleset(api_client);

    // create_vpn(api_client);
    // delete_vpn(api_client);
    // list_vpn(api_client);
    // details_vpn(api_client);
}

// get details about a specific vpn
fn details_vpn<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let vpn_id = "123";
    let space = &space::VPNDetails::new(space_id, vpn_id);
    let response = api_client.request(space);

    print_response(response);
}

// list vpn
fn list_vpn<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::VPNList::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// delete vpn
fn delete_vpn<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let vpn_id = "123";
    let space = &space::VPNDelete::new(space_id, vpn_id);
    let response = api_client.request(space);

    print_response(response);
}

// create vpn
fn create_vpn<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::VPNCreate::new(space_id, "office", "35.161.69.30", vec!["172.16.0.0/16"]);
    let response = api_client.request(space);

    print_response(response);
}

// create outbound ruleset
fn create_outbound_ruleset<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let target = "1.1.1.1/1";
    let protocol = "tcp";
    let to_port = 80;
    let from_port = 80;
    let space = &space::OutboundRulesetCreate::new(space_id)
        .rule(target, protocol, from_port, to_port)
        .build();
    let response = api_client.request(space);

    print_response(response);
}

// get outbound ruleset details
fn get_outbound_ruleset_details<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let outbound_ruleset_id = "123";
    let space = &space::OutboundRulesetDetails::new(space_id, outbound_ruleset_id);
    let response = api_client.request(space);

    print_response(response);
}

// get outbound ruleset current
fn get_outbound_ruleset_current<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::OutboundRulesetCurrent::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// get outbound ruleset list
fn get_outbound_ruleset_list<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::OutboundRulesetList::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// create inbound ruleset
fn create_inbound_ruleset<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::InboundRulesetCreate::new(space_id)
        .rule("allow", "1.1.1.1/1")
        .build();
    let response = api_client.request(space);

    print_response(response);
}

// get inbound ruleset details
fn get_inbound_ruleset_details<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let ruleset_id = "123";
    let space = &space::InboundRulesetDetails::new(space_id, ruleset_id);
    let response = api_client.request(space);

    print_response(response);
}

// get inbound ruleset current
fn get_inbound_ruleset_current<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::InboundRulesetCurrent::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// get inbound ruleset list
fn get_inbound_ruleset_list<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::InboundRulesetList::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// get Space Network Address Translation Info
fn get_space_nat<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::SpaceNATDetails::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// create space transfer
fn create_space_transfer<T: HerokuApiClient>(api_client: &T) {
    let new_owner_id = "123";
    let space_id = "123";
    let space = &space::SpaceTransferCreate::new(space_id, new_owner_id);
    let response = api_client.request(space);

    print_response(response);
}

// get spaces access members
fn get_space_access_members<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::SpaceAccessList::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// u0pdate spaces access
fn update_space_access<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let account_id = "123";
    let permission_name = "examples";
    let space = &space::SpaceAccessUpdate::new(space_id, account_id, permission_name);
    let response = api_client.request(space);

    print_response(response);
}

// get spaces access
fn get_space_access<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let account_id = "123";
    let space = &space::SpaceAccessDetails::new(space_id, account_id);
    let response = api_client.request(space);

    print_response(response);
}
// get spaces list
fn delete_space<T: HerokuApiClient>(api_client: &T) {
    let space_id = "123";
    let space = &space::SpaceDelete::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

// get spaces list
fn get_spaces<T: HerokuApiClient>(api_client: &T) {
    let space = &space::SpaceList::new();
    let response = api_client.request(space);

    print_response(response);
}

//get space
fn get_space<T: HerokuApiClient>(api_client: &T) {
    let space_id = "myspacename";
    let space = &space::SpaceDetails::new(space_id);
    let response = api_client.request(space);

    print_response(response);
}

//update space
fn update_space<T: HerokuApiClient>(api_client: &T) {
    let space_id = "myspacename";
    let space = &space::SpaceUpdate::new(space_id)
        .name("mynewspacename")
        .build();
    let response = api_client.request(space);

    print_response(response);
}

// create space
fn create_space<T: HerokuApiClient>(api_client: &T) {
    let space = &space::SpaceCreate::new("myspacename", "myteamname")
        .cidr("123")
        .data_cidr("10.2.0.0/16")
        .region("6f2b2ec9-b087-4976-8ec9-5d2f62276aeb")
        .shield(true)
        .build();
    let response = api_client.request(space);

    print_response(response);
}
