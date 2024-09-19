use substreams::{
    log,
    store::{DeltaString, Deltas},
};

use crate::pb;
use pb::contract::v1 as contract;

#[substreams::handlers::map]
fn map_test(
    store_incoming: Deltas<DeltaString>,
    store_outgoing: Deltas<DeltaString>,
) -> Result<contract::Events, substreams::errors::Error> {
    let events = contract::Events::default();

    for delta_incoming in store_incoming.deltas {
        log::info!("Delta: {:?}", delta_incoming.new_value);
    }

    for delta_outgoing in store_outgoing.deltas {
        log::info!("Delta: {:?}", delta_outgoing.new_value);
    }

    Ok(events)
}
