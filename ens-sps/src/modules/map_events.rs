use hex_literal::hex;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

use crate::abi;
use crate::pb;
use pb::contract::v1 as contract;

substreams_ethereum::init!();

const OLDETHREGISTRAR_TRACKED_CONTRACT: [u8; 20] = hex!("283af0b28c62c092c9727f1ee09c02ca627eb7f5");
const BASEREGISTRAR_TRACKED_CONTRACT: [u8; 20] = hex!("57f1887a8bf19b14fc0df6fd9b2acc9af147ea85");
const ETHREGISTRAR_TRACKED_CONTRACT: [u8; 20] = hex!("253553366da8546fc250f225fe3d25d0c782303b");
const NAMEWRAPPER_TRACKED_CONTRACT: [u8; 20] = hex!("d4416b13d2b3a9abae7acd5d6c2bbdbe25686401");
const DNSREGISTRAR_TRACKED_CONTRACT: [u8; 20] = hex!("b32cb5677a7c971689228ec835800432b339ba2b");

fn map_oldethregistrar_events(blk: &eth::Block, events: &mut contract::Events) {
    events.oldethregistrar_name_registereds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == OLDETHREGISTRAR_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::oldethregistrar_contract::events::NameRegistered::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::OldethregistrarNameRegistered {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                cost: event.cost.to_string(),
                                expires: event.expires.to_string(),
                                label: Vec::from(event.label),
                                name: event.name,
                                owner: event.owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}
fn map_baseregistrar_events(blk: &eth::Block, events: &mut contract::Events) {
    events.baseregistrar_transfers.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == BASEREGISTRAR_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::baseregistrar_contract::events::Transfer::match_and_decode(log)
                        {
                            return Some(contract::BaseregistrarTransfer {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                token_id: event.token_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}
fn map_ethregistrar_events(blk: &eth::Block, events: &mut contract::Events) {
    events.ethregistrar_name_registereds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == ETHREGISTRAR_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::ethregistrar_contract::events::NameRegistered::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::EthregistrarNameRegistered {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                base_cost: event.base_cost.to_string(),
                                expires: event.expires.to_string(),
                                label: Vec::from(event.label),
                                name: event.name,
                                owner: event.owner,
                                premium: event.premium.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}
fn map_namewrapper_events(blk: &eth::Block, events: &mut contract::Events) {
    events.namewrapper_transfer_batches.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NAMEWRAPPER_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::namewrapper_contract::events::TransferBatch::match_and_decode(log)
                        {
                            return Some(contract::NamewrapperTransferBatch {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                ids: event
                                    .ids
                                    .into_iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>(),
                                operator: event.operator,
                                to: event.to,
                                values: event
                                    .values
                                    .into_iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.namewrapper_transfer_singles.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == NAMEWRAPPER_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::namewrapper_contract::events::TransferSingle::match_and_decode(log)
                        {
                            return Some(contract::NamewrapperTransferSingle {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                id: event.id.to_string(),
                                operator: event.operator,
                                to: event.to,
                                value: event.value.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}
fn map_dnsregistrar_events(blk: &eth::Block, events: &mut contract::Events) {
    events.dnsregistrar_claims.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == DNSREGISTRAR_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::dnsregistrar_contract::events::Claim::match_and_decode(log)
                        {
                            return Some(contract::DnsregistrarClaim {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                dnsname: event.dnsname,
                                inception: event.inception.to_u64(),
                                node: Vec::from(event.node),
                                owner: event.owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_oldethregistrar_events(&blk, &mut events);
    map_baseregistrar_events(&blk, &mut events);
    map_ethregistrar_events(&blk, &mut events);
    map_namewrapper_events(&blk, &mut events);
    map_dnsregistrar_events(&blk, &mut events);
    substreams::skip_empty_output();
    Ok(events)
}
