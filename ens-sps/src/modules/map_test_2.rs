use substreams::{
    log,
    store::{DeltaString, Deltas},
};

use crate::pb;
use pb::contract::v1 as contract;

#[substreams::handlers::map]
fn map_test_2(store: Deltas<DeltaString>) -> Result<contract::Events, substreams::errors::Error> {
    let events = contract::Events::default();

    for delta in store.deltas {
        log::info!(
            "[Delta 2]; owner: {:?} : name: {:?}",
            delta.key,
            delta.new_value
        );
    }

    Ok(events)
}
