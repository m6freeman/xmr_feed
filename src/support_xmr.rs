pub mod support_xmr {

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct SupportXMRStatus {
        #[serde(rename = "hash")]
        pub hash: f32,
        #[serde(rename = "identifier")]
        pub identifier: String,
        #[serde(rename = "lastHash")]
        pub last_hash: f32,
        #[serde(rename = "totalHashes")]
        pub total_hashes: f32,
        #[serde(rename = "validShares")]
        pub valid_shares: f32,
        #[serde(rename = "invalidShares")]
        pub invalid_shares: f32,
        #[serde(rename = "expiry")]
        pub expiry: f32,
        #[serde(rename = "amtPaid")]
        pub amt_paid: f32,
        #[serde(rename = "amtDue")]
        pub amt_due: f32,
        #[serde(rename = "txnCount")]
        pub txn_count: f32,
    }

    pub fn get_status(url: String) -> Result<SupportXMRStatus, Box<dyn std::error::Error>> {
        let support_xmr_response = minreq::get(&url)
            .with_header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/124.0.0.0 Safari/537.36",
            )
            .with_header("Accept", "application/json")
            .send()?;
        if support_xmr_response.status_code == 200 {
            let support_xmr_status: SupportXMRStatus =
                serde_json::from_str(support_xmr_response.as_str().unwrap())?;
            return Ok(support_xmr_status);
        }
        Err(support_xmr_response.status_code.to_string().into())
    }
}
