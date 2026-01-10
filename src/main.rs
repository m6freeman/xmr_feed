mod coin_gecko;
mod secrets;
mod support_xmr;
pub use secrets::PUBLIC_ADDRESS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conversion_rate: f32 = 0.000000000001;
    let public_address: &str = secrets::PUBLIC_ADDRESS;

    let support_xmr_url: String =
        format!("https://www.supportxmr.com/api/miner/{public_address}/stats");
    let support_xmr_response: support_xmr::support_xmr::SupportXMRStatus =
        match support_xmr::support_xmr::get_status(support_xmr_url) {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            }
        };

    let coin_gecko_url: String =
        format!("https://api.coingecko.com/api/v3/simple/price?ids=monero&vs_currencies=usd");
    let coin_gecko_response: coin_gecko::coin_gecko::CoinGeckoResponse =
        match coin_gecko::coin_gecko::get_to_usd(coin_gecko_url) {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            }
        };

    println!(
        "{} {}kh/s {:.5}/{:.5} ${:.2}",
        &public_address[..6],
        &support_xmr_response.hash,
        &support_xmr_response.amt_due * conversion_rate,
        &support_xmr_response.amt_paid * conversion_rate,
        &coin_gecko_response.monero.usd
            * ((&support_xmr_response.amt_due + &support_xmr_response.amt_paid) * conversion_rate)
    );
    Ok(())
}
