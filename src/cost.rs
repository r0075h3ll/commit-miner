use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub input_per_million_usd: f64,
    pub output_per_million_usd: f64,
    pub checked_on: String,
}

fn price(model: &str) -> Option<Price> {
    // OpenRouter model IDs only: https://openrouter.ai/api/v1/models (checked 2026-09-17).
    // An alias (":free", ":nitro", no version) can move to a differently priced
    // backend, so only exact recommended IDs are priced here.
    let (input, output) = match model {
        // Open-weight / open-source.
        "meta-llama/llama-3.3-70b-instruct" => (0.10, 0.32),
        "meta-llama/llama-4-scout" => (0.10, 0.30),
        "meta-llama/llama-4-maverick" => (0.1875, 0.6525),
        "qwen/qwen-2.5-72b-instruct" => (0.36, 0.40),
        "qwen/qwen3-30b-a3b" => (0.12, 0.50),
        "deepseek/deepseek-chat-v3.1" => (0.25, 0.95),
        "mistralai/mistral-small-3.2-24b-instruct" => (0.09375, 0.25),
        // Low-cost GPT.
        "openai/gpt-4o-mini" => (0.15, 0.60),
        "openai/gpt-4.1-mini" => (0.40, 1.60),
        "openai/gpt-4.1-nano" => (0.10, 0.40),
        "openai/gpt-5-mini" => (0.25, 2.00),
        "openai/gpt-5-nano" => (0.05, 0.40),
        _ => return None,
    };
    Some(Price {
        input_per_million_usd: input,
        output_per_million_usd: output,
        checked_on: "2026-09-17".into(),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelCost {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub price: Option<Price>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CostEstimate {
    pub models: BTreeMap<String, ModelCost>,
}
impl CostEstimate {
    pub fn record(&mut self, model: &str, input: u64, output: u64) {
        let usage = self
            .models
            .entry(model.into())
            .or_insert_with(|| ModelCost {
                input_tokens: 0,
                output_tokens: 0,
                price: price(model),
            });
        usage.input_tokens += input;
        usage.output_tokens += output;
    }
    pub fn usd(&self) -> Option<f64> {
        if self.models.is_empty() {
            return None;
        }
        self.models.values().try_fold(0., |sum, usage| {
            let p = usage.price.as_ref()?;
            Some(
                sum + (usage.input_tokens as f64 * p.input_per_million_usd
                    + usage.output_tokens as f64 * p.output_per_million_usd)
                    / 1_000_000.,
            )
        })
    }
    pub fn label(&self) -> String {
        match self.usd() {
            Some(usd) if usd > 0. && usd < 0.000001 => "est. <$0.000001".into(),
            Some(usd) => format!("est. ${usd:.6}"),
            None => "cost unavailable".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MODEL: &str = "meta-llama/llama-3.3-70b-instruct";
    #[test]
    fn calculates_reported_tokens_and_preserves_the_price_snapshot() {
        let mut cost = CostEstimate::default();
        cost.record(MODEL, 1_000_000, 1_000_000);
        assert!((cost.usd().unwrap() - 0.42).abs() < 1e-12);
        assert_eq!(cost.label(), "est. $0.420000");
        cost.record(MODEL, 1_000_000, 0);
        let saved = serde_json::to_vec(&cost).unwrap();
        let restored: CostEstimate = serde_json::from_slice(&saved).unwrap();
        assert_eq!(
            restored.models[MODEL].price.as_ref().unwrap().checked_on,
            "2026-09-17"
        );
        assert!((restored.usd().unwrap() - 0.52).abs() < 1e-12);
    }
    #[test]
    fn unknown_models_and_tiny_costs_are_not_shown_as_free() {
        let mut cost = CostEstimate::default();
        cost.record(MODEL, 1, 0);
        assert_eq!(cost.label(), "est. <$0.000001");
        cost.record("some-vendor/some-future-model", 10, 0);
        assert_eq!(cost.label(), "cost unavailable");
        assert!(price("openai/gpt-6-hypothetical").is_none());
    }
}
