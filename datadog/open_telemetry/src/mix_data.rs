use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::metrics::{SdkMeterProvider, PeriodicReader};
use opentelemetry_sdk::Resource;
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

struct DatadogExporter {
    client: Client,
    api_key: String,
    base_url: String,
}

impl DatadogExporter {
    pub fn new(api_key: String) -> Self {
        DatadogExporter {
            api_key,
            client: Client::new(),
            base_url: "https://api.datadoghq.com/api/v1".to_string(),
        }
    }

    async fn export_metric(
        &self, 
        name: &str, 
        value: f64, 
        metric_type: &str,
        attributes: &[KeyValue]
    ) -> Result<(), Box<dyn Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        // Convert OpenTelemetry attributes to Datadog tags
        let tags = attributes.iter()
            .map(|kv| format!("{}:{}", kv.key, kv.value.as_str().unwrap_or("unknown")))
            .collect::<Vec<String>>();

        let payload = json!({
            "series": [{
                "metric": name,
                "points": [[now, value]],
                "type": metric_type,
                "tags": tags,
            }]
        });

        let response = self.client
            .post(&format!("{}/series", self.base_url))
            .header("Content-Type", "application/json")
            .header("DD-API-KEY", &self.api_key)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to send metric: {} - {}",
                response.status(),
                response.text().await?
            ).into());
        }

        Ok(())
    }
}

fn init_meter_provider(dd_api_key: String) -> SdkMeterProvider {
    let provider = SdkMeterProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name("my-service")
                .build(),
        )
        .build();
    
    global::set_meter_provider(provider.clone());
    provider
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dd_api_key = std::env::var("DATADOG_API_KEY")
        .expect("DATADOG_API_KEY must be set");
    
    // Initialize the MeterProvider
    let meter_provider = init_meter_provider(dd_api_key.clone());
    let dd_exporter = DatadogExporter::new(dd_api_key);

    // Create a meter
    let meter = global::meter("my-service");

    // Create various metric instruments
    let counter = meter.u64_counter("requests_total").build();
    let histogram = meter
        .f64_histogram("request_duration")
        .with_description("Request duration in seconds")
        .with_unit("s")
        .build();
    let gauge = meter
        .f64_gauge("system_memory_usage")
        .with_description("Current memory usage")
        .with_unit("bytes")
        .build();

    // Example attributes
    let attributes = &[
        KeyValue::new("service", "api"),
        KeyValue::new("environment", "production"),
    ];

    // Simulate some metrics
    for _ in 0..3 {
        // Record counter
        counter.add(1, attributes);
        dd_exporter.export_metric(
            "requests_total",
            1.0,
            "count",
            attributes
        ).await?;

        // Record histogram
        let duration = 0.5;
        histogram.record(duration, attributes);
        dd_exporter.export_metric(
            "request_duration",
            duration,
            "histogram",
            attributes
        ).await?;

        // Record gauge
        let memory = 1024.0 * 1024.0; // 1MB
        gauge.record(memory, attributes);
        dd_exporter.export_metric(
            "system_memory_usage",
            memory,
            "gauge",
            attributes
        ).await?;

        sleep(Duration::from_secs(1)).await;
    }

    // Shutdown the provider
    meter_provider.shutdown()?;

    Ok(())
}