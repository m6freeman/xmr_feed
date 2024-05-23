mod constants;
mod support_xmr;
pub use constants::ADDRESS;
pub use support_xmr::support_xmr::SupportXMRResponse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conversion_rate: f32 = 0.000000000001;
    let address: &str = constants::ADDRESS;
    let url: String = format!("https://www.supportxmr.com/api/miner/{address}/stats");
    let response = minreq::get(&url).send()?;
    let body: &str = response.as_str()?;
    if response.status_code == 200 {
        let parsed: SupportXMRResponse = serde_json::from_str(body)?;
        println!(
            "{} {}kh/s {:.5}/{:.5}",
            &address[..6],
            &parsed.hash,
            &parsed.amt_due * conversion_rate,
            &parsed.amt_paid * conversion_rate
        );
        Ok(())
    } else {
        println!("{}", response.reason_phrase);
        Ok(())
    }
}
