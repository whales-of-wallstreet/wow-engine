use crate::bridge::{Chain, debridge::DeBridgeClient, cctp::CctpClient, BridgeProvider};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOption {
    pub provider: String,
    pub path: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub estimated_fee_usd: f64,
    pub duration_seconds: u64,
    pub execution_payload: Option<String>,
}

pub struct RoutePlanner {
    debridge: DeBridgeClient,
    cctp: CctpClient,
}

impl RoutePlanner {
    pub fn new() -> Self {
        Self {
            debridge: DeBridgeClient::new(),
            cctp: CctpClient::new(),
        }
    }

    pub async fn find_best_route(
        &self,
        source_chain: Chain,
        dest_chain: Chain,
        source_asset: &str,
        dest_asset: &str,
        amount_in: u64,
    ) -> Result<Vec<RouteOption>, anyhow::Error> {
        let mut routes = Vec::new();

        // 1. Gather quote from deBridge DLN
        if let Ok(quote) = self.debridge.get_quote(source_chain, dest_chain, source_asset, dest_asset, amount_in).await {
            routes.push(RouteOption {
                provider: quote.provider,
                path: format!("{} -> {}", source_chain, dest_chain),
                amount_in: quote.amount_in,
                amount_out: quote.amount_out,
                estimated_fee_usd: quote.estimated_fee_usd,
                duration_seconds: quote.duration_seconds,
                execution_payload: quote.execution_payload,
            });
        }

        // 2. Gather quote from Circle CCTP (for USDC asset pairs)
        let is_usdc_pair = source_asset.to_uppercase().contains("USDC") 
            && dest_asset.to_uppercase().contains("USDC");
            
        if is_usdc_pair {
            if let Ok(quote) = self.cctp.get_quote(source_chain, dest_chain, source_asset, dest_asset, amount_in).await {
                routes.push(RouteOption {
                    provider: quote.provider,
                    path: format!("{} -(Native CCTP)-> {}", source_chain, dest_chain),
                    amount_in: quote.amount_in,
                    amount_out: quote.amount_out,
                    estimated_fee_usd: quote.estimated_fee_usd,
                    duration_seconds: quote.duration_seconds,
                    execution_payload: quote.execution_payload,
                });
            }
        }

        // Sort routes: highest amount_out first, then lowest estimated_fee_usd
        routes.sort_by(|a, b| {
            b.amount_out.cmp(&a.amount_out)
                .then_with(|| a.estimated_fee_usd.partial_cmp(&b.estimated_fee_usd).unwrap_or(std::cmp::Ordering::Equal))
        });

        Ok(routes)
    }
}

impl Default for RoutePlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_best_route_usdc() {
        let planner = RoutePlanner::new();
        let routes = planner.find_best_route(
            Chain::Solana,
            Chain::Stellar,
            "USDC",
            "USDC",
            10000,
        ).await.unwrap();

        assert_eq!(routes.len(), 2, "Should return exactly 2 routes for USDC transfer");
        
        // Route 0 must be Circle CCTP due to 1:1 burn/mint output (10000 out)
        assert_eq!(routes[0].provider, "Circle CCTP");
        assert_eq!(routes[0].amount_out, 10000);

        // Route 1 must be deBridge DLN due to 0.1% protocol fee (9990 out)
        assert_eq!(routes[1].provider, "deBridge DLN");
        assert_eq!(routes[1].amount_out, 9990);
    }
}
