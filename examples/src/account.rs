#![allow(dead_code)] // Just warns about un-used methods until they're used.
use heroku_rs::client::{Executor, Heroku};
use heroku_rs::errors::Error;
use heroku_rs::{HeaderMap, StatusCode};
use serde_json::Value;
// Uncomment methods to run them.
pub fn run(client: Heroku) {
    get_account(&client);
    // get_account_features(&client);
    // get_specific_account_feature(&client);
    // get_user_account(&client);
    // get_account_transfers(&client);
    // get_account_credits(&client);
    // get_account_invoices(&client);
    // get_account_invoice_address(&client);
    // get_account_keys(&client);
    // get_account_rate_limits(&client);
    // get_user_account_addons(&client);
    // get_user_account_apps(&client);
    // get_user_account_sms_number(&client);
    // get_user_account_apps(&client);
}

// == GET account ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#account-info
// Requires the Heroku client
// GET /account
fn get_account(client: &Heroku) {
    let me = client.get().account().execute::<Value>();
    log_response(me);
}

// == GET account feature list. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#account-feature-list
// Requires the Heroku client
// GET /account/features
fn get_account_features(client: &Heroku) {
    let me = client.get().account().account_features().execute::<Value>();
    log_response(me);
}

// == GET specific account feature. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#account-feature-info
// You can get a feature by name e.g. team-internal-routing : .get().account().account_features().feature_id("ID_HERE")
// GET /account/features
fn get_specific_account_feature(client: &Heroku) {
    let me = client
        .get()
        .account()
        .account_features()
        .feature_name("team-internal-routing")
        .execute::<Value>();
    log_response(me);
}

// == GET specific account. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#account-info-by-user
// You can get a user by id e.g.: .get().accounts().account_id("ID_HERE")
// GET /users/{account_email_or_id_or_self}
fn get_user_account(client: &Heroku) {
    let me = client
        .get()
        .accounts()
        .account_email("EMAIL_HERE")
        .execute::<Value>();
    log_response(me);
}

// == GET account transfers. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-list
// You can also get a transfer by id or name:
//   e.g.: .get().account().account_app_tranfers().transfer_id("ID_HERE").execute::<Value>();
//   e.g.: .get().account().account_app_tranfers().transfer_name("NAME_HERE").execute::<Value>();
// GET /account/app-transfers
fn get_account_transfers(client: &Heroku) {
    let me = client
        .get()
        .account()
        .account_app_tranfers()
        .execute::<Value>();
    log_response(me);
}

// == GET account credits. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#credit-list
// You can also get a credit by id: .get().account().account_credits().credit_id("ID_HERE").execute::<Value>();
// GET /account/credits
fn get_account_credits(client: &Heroku) {
    let me = client.get().account().account_credits().execute::<Value>();
    log_response(me);
}

// == GET account invoices. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#invoice-list
// You can also get a invoice by id: .get().account().account_invoices().invoice_id("ID_HERE").execute::<Value>();
// GET /account/invoices
fn get_account_invoices(client: &Heroku) {
    let me = client.get().account().account_invoices().execute::<Value>();
    log_response(me);
}

// == GET account invoice address. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#invoice-address-info
// GET /account/invoice-address
fn get_account_invoice_address(client: &Heroku) {
    let me = client
        .get()
        .account()
        .account_invoice_address()
        .execute::<Value>();
    log_response(me);
}

// == GET account keys. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#key-list
// You can also get a key by id: .get().account().account_keys().key_id("ID_HERE").execute::<Value>();
//  or by fingerprint: .get().account().account_keys().key_fingerprint("FINGERPRINT_HERE").execute::<Value>();
// GET /account/keys
fn get_account_keys(client: &Heroku) {
    let me = client.get().account().account_keys().execute::<Value>();
    log_response(me);
}

// == GET account rate limit. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#rate-limit-info
// GET /account/rate-limits
fn get_account_rate_limits(client: &Heroku) {
    let me = client
        .get()
        .account()
        .account_rate_limits()
        .execute::<Value>();
    log_response(me);
}

// == GET user account addons. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-user
// You can also get an account by id: .get().accounts().account_id("ID_HERE").account_addons().execute::<Value>();
// GET /users/{account_email_or_id_or_self}/addons
fn get_user_account_addons(client: &Heroku) {
    let me = client
        .get()
        .accounts()
        .account_email("EMAIL_HERE")
        .account_addons()
        .execute::<Value>();
    log_response(me);
}

// == GET user account apps. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-list-owned-and-collaborated
// You can also get an account by id: .get().accounts().account_id("ID_HERE").account_apps().execute::<Value>();
// GET /users/{account_email_or_id_or_self}/apps
fn get_user_account_apps(client: &Heroku) {
    let me = client
        .get()
        .accounts()
        .account_email("EMAIL_HERE")
        .account_apps()
        .execute::<Value>();
    log_response(me);
}

// == GET user account sms number. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#sms-number-sms-number
// You can also get an account by id: .get().accounts().account_id("ID_HERE").account_sms_number().execute::<Value>();
// GET /users/{account_email_or_id_or_self}/sms-number
fn get_user_account_sms_number(client: &Heroku) {
    let me = client
        .get()
        .accounts()
        .account_email("EMAIL_HERE")
        .account_sms_number()
        .execute::<Value>();
    log_response(me);
}

//a generic method to log heroku responses and avoid code duplication
fn log_response<T>(me: Result<(HeaderMap, StatusCode, Option<T>), Error>)
where
    T: ToString,
{
    match me {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json.to_string());
            }
        }
        Err(e) => println!("Err {}", e),
    }
}
