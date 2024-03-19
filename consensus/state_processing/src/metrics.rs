use lazy_static::lazy_static;
pub use lighthouse_metrics::*;

lazy_static! {
    /*
     * Participation Metrics
     */
    pub static ref PARTICIPATION_PREV_EPOCH_HEAD_ATTESTING_GWEI_TOTAL: Result<IntGauge> = try_create_int_gauge(
        "beacon_participation_prev_epoch_head_attesting_gwei_total",
        "Total effective balance (gwei) of validators who attested to the head in the previous epoch"
    );
    pub static ref PARTICIPATION_PREV_EPOCH_TARGET_ATTESTING_GWEI_TOTAL: Result<IntGauge> = try_create_int_gauge(
        "beacon_participation_prev_epoch_target_attesting_gwei_total",
        "Total effective balance (gwei) of validators who attested to the target in the previous epoch"
    );
    pub static ref PARTICIPATION_PREV_EPOCH_SOURCE_ATTESTING_GWEI_TOTAL: Result<IntGauge> = try_create_int_gauge(
        "beacon_participation_prev_epoch_source_attesting_gwei_total",
        "Total effective balance (gwei) of validators who attested to the source in the previous epoch"
    );
    /*
     * Processing metrics
     */
    pub static ref PROCESS_EPOCH_TIME: Result<Histogram> = try_create_histogram(
        "beacon_state_processing_process_epoch",
        "Time required for process_epoch",
    );
    /*
     * Participation Metrics (progressive balances)
     */
    pub static ref PARTICIPATION_PREV_EPOCH_TARGET_ATTESTING_GWEI_PROGRESSIVE_TOTAL: Result<IntGauge> = try_create_int_gauge(
        "beacon_participation_prev_epoch_target_attesting_gwei_progressive_total",
        "Progressive total effective balance (gwei) of validators who attested to the target in the previous epoch"
    );
    pub static ref PARTICIPATION_CURR_EPOCH_TARGET_ATTESTING_GWEI_PROGRESSIVE_TOTAL: Result<IntGauge> = try_create_int_gauge(
        "beacon_participation_curr_epoch_target_attesting_gwei_progressive_total",
        "Progressive total effective balance (gwei) of validators who attested to the target in the current epoch"
    );
}
