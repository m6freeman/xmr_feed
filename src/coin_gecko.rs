pub mod coin_gecko {

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct CoinGeckoResponse {
        #[serde(rename = "monero")]
        pub monero: Monero,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct Monero {
        #[serde(rename = "usd")]
        pub usd: f32,
    }

    pub fn get_to_usd(url: String) -> Result<CoinGeckoResponse, Box<dyn std::error::Error>> {
        let coin_gecko_response = minreq::get(&url)
            .with_header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                 AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/124.0.0.0 Safari/537.36",
            )
            .with_header("Accept", "application/json")
            .send()?;
        if coin_gecko_response.status_code == 200 {
            let coin_gecko_response_obj: CoinGeckoResponse =
                serde_json::from_str(coin_gecko_response.as_str().unwrap())?;
            return Ok(coin_gecko_response_obj);
        }
        Err(coin_gecko_response.status_code.to_string().into())
    }
}
