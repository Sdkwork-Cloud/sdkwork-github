use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static REQUESTS_TOTAL: AtomicU64 = AtomicU64::new(0);
static REQUESTS_FAILED: AtomicU64 = AtomicU64::new(0);
static REQUEST_DURATION_MS_TOTAL: AtomicU64 = AtomicU64::new(0);

pub fn record_success(duration: Duration) {
    REQUESTS_TOTAL.fetch_add(1, Ordering::Relaxed);
    REQUEST_DURATION_MS_TOTAL.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
}

pub fn record_failure() {
    REQUESTS_TOTAL.fetch_add(1, Ordering::Relaxed);
    REQUESTS_FAILED.fetch_add(1, Ordering::Relaxed);
}

pub fn snapshot() -> ProviderMetricsSnapshot {
    ProviderMetricsSnapshot {
        requests_total: REQUESTS_TOTAL.load(Ordering::Relaxed),
        requests_failed: REQUESTS_FAILED.load(Ordering::Relaxed),
        request_duration_ms_total: REQUEST_DURATION_MS_TOTAL.load(Ordering::Relaxed),
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProviderMetricsSnapshot {
    pub requests_total: u64,
    pub requests_failed: u64,
    pub request_duration_ms_total: u64,
}
