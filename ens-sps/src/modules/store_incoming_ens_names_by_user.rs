use substreams::store::Appender;
use substreams::store::StoreAppend;
use substreams::Hex;

use crate::pb;
use pb::contract::v1 as contract;

#[substreams::handlers::store]
pub fn store_incoming_ens_names_by_user(map_events: contract::Events, output: StoreAppend<String>) {
    for event in map_events.oldethregistrar_name_registereds.iter() {
        output.append(
            0,
            Hex(&event.owner.to_vec()).to_string(),
            event.name.to_string(),
        )
    }
    for event in map_events.baseregistrar_transfers.iter() {
        // TODO: label_hash from token_id
        let label_hash = Hex::encode(event.token_id.clone().to_signed_bytes_be());
        output.append(
            0,
            Hex(&event.to.to_vec()).to_string(),
            label_hash.to_string(),
        )
    }
}
