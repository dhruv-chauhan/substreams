use substreams::store::StoreGet;
use substreams::store::StoreGetString;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetString;
use substreams::store::{DeltaString, Deltas};

#[substreams::handlers::store]
pub fn store_ens_names_by_user(
    store_incoming: StoreGetString,
    store_incoming_deltas: Deltas<DeltaString>,
    store_outgoing: StoreGetString,
    store_outgoing_deltas: Deltas<DeltaString>,
    output: StoreSetString,
) {
    for incoming_delta in store_incoming_deltas.deltas {
        let owner = incoming_delta.get_key();

        let incoming = store_incoming.get_last(owner).unwrap_or_default();
        let outgoing = store_outgoing.get_last(owner).unwrap_or_default();
        let incoming_names: Vec<&str> = incoming.split(";").collect();
        let outgoing_names: Vec<&str> = outgoing.split(";").collect();

        let mut resultant_names: Vec<String> = Vec::new();
        for name in incoming_names.iter() {
            if !outgoing_names.contains(&name) {
                resultant_names.push(name.to_string());
            }
        }

        output.set(0, owner, &resultant_names.join(";"));
    }

    for outgoing_delta in store_outgoing_deltas.deltas {
        let owner = outgoing_delta.get_key();

        let incoming = store_incoming.get_last(owner).unwrap_or_default();
        let outgoing = store_outgoing.get_last(owner).unwrap_or_default();
        let incoming_names: Vec<&str> = incoming.split(";").collect();
        let outgoing_names: Vec<&str> = outgoing.split(";").collect();

        let mut resultant_names: Vec<String> = Vec::new();
        for name in incoming_names.iter() {
            if !outgoing_names.contains(&name) {
                resultant_names.push(name.to_string());
            }
        }

        output.set(0, owner, &resultant_names.join(";"));
    }
}
