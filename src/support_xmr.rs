pub mod support_xmr {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct SupportXMRResponse {
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
}
