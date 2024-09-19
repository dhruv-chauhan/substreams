use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetString;
use substreams::store::{DeltaString, Deltas};

#[substreams::handlers::store]
pub fn store_user_by_ens_name(store_deltas: Deltas<DeltaString>, output: StoreSetString) {
    for delta in store_deltas.deltas {
        let owner = delta.get_key();
        let name = delta.new_value.clone();

        if name != "" {
            output.set(0, name, owner);
        }
    }
}
