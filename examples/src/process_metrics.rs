use anyhow::Result;
use fastmetrics::{
    format::text,
    registry::{Register, Registry},
};
use fastmetrics_process::ProcessMetrics;

fn main() -> Result<()> {
    let metrics = ProcessMetrics::default();

    let mut registry = Registry::builder().with_namespace("process").build()?;
    metrics.register(&mut registry)?;

    let mut encoded = String::new();
    text::encode(&mut encoded, &registry)?;
    println!("\n=== Exported Metrics ===\n{encoded}");

    Ok(())
}
