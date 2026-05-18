use crate::anchor::Sep24InteractiveResponse;
use reqwest::Client;

pub struct Sep24Client {
    #[allow(dead_code)]
    client: Client,
}

impl Sep24Client {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn initiate_deposit(
        &self,
        anchor_domain: &str,
        asset_code: &str,
        account: &str,
    ) -> Result<Sep24InteractiveResponse, anyhow::Error> {
        let tx_id = format!("tx_sep24_{}", uuid_fast());
        
        // Simulates query to anchor's `https://<domain>/sep24/transactions/deposit/interactive`
        let interactive_url = format!(
            "https://{}/sep24/interactive?asset_code={}&account={}&transaction_id={}&callback=postMessage",
            anchor_domain, asset_code, account, tx_id
        );

        Ok(Sep24InteractiveResponse {
            r#type: "interactive_customer_info_needed".to_string(),
            url: interactive_url,
            id: tx_id,
        })
    }

    pub async fn initiate_withdrawal(
        &self,
        anchor_domain: &str,
        asset_code: &str,
        account: &str,
    ) -> Result<Sep24InteractiveResponse, anyhow::Error> {
        let tx_id = format!("tx_sep24_{}", uuid_fast());
        
        // Simulates query to anchor's `https://<domain>/sep24/transactions/withdraw/interactive`
        let interactive_url = format!(
            "https://{}/sep24/interactive/withdraw?asset_code={}&account={}&transaction_id={}&callback=postMessage",
            anchor_domain, asset_code, account, tx_id
        );

        Ok(Sep24InteractiveResponse {
            r#type: "interactive_customer_info_needed".to_string(),
            url: interactive_url,
            id: tx_id,
        })
    }
}

fn uuid_fast() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", start & 0xFFFFFFFFFFFFFFF)
}
