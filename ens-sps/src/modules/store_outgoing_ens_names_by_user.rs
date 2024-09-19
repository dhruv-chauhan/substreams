use std::str::FromStr;

use substreams::scalar::BigInt;
use substreams::store::Appender;
use substreams::store::StoreAppend;
use substreams::{log, Hex};

use crate::pb;
use pb::contract::v1 as contract;

#[substreams::handlers::store]
pub fn store_outgoing_ens_names_by_user(map_events: contract::Events, output: StoreAppend<String>) {
    for event in map_events.baseregistrar_transfers.iter() {
        let label_hash = Hex::encode(
            BigInt::from_str(event.token_id.clone().as_str())
                .unwrap()
                .to_signed_bytes_be(),
        );
        output.append(
            0,
            Hex(&event.from.to_vec()).to_string(),
            label_hash.to_string(),
        );
        log::info!("Transfer O Tx: {:?}", event.evt_tx_hash);

        // output.append(
        //     0,
        //     Hex(&event.from.to_vec()).to_string(),
        //     event.evt_tx_hash.clone(),
        // );
    }
}
