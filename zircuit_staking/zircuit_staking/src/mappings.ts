import { log } from "@graphprotocol/graph-ts";
import { Protobuf } from "as-proto/assembly";
import { JSON } from "assemblyscript-json";
import { Event } from "../generated/schema";
import { Events } from "./pb/contract/v1/Events";


export function handleEvents(bytes: Uint8Array): void {
  const events: Events = Protobuf.decode<Events>(bytes, Events.decode);

  // Below you will find examples of how to save the decoded events.
  // These are only examples, you can modify them to suit your needs.
  for (let i = 0; i < events.ztakingpoolBlocklistChangeds.length; i++) {
    const e = events.ztakingpoolBlocklistChangeds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("blocked", e.blocked);
    obj.set("migrator", e.migrator);
    evt.jsonValue = obj.toString();
    evt.type = "blocklistChanged";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolDeposits.length; i++) {
    const e = events.ztakingpoolDeposits[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("amount", e.amount);
    obj.set("depositor", e.depositor);
    obj.set("eventId", e.eventId);
    obj.set("token", e.token);
    evt.jsonValue = obj.toString();
    evt.type = "deposit";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolEip712DomainChangeds.length; i++) {
    const e = events.ztakingpoolEip712DomainChangeds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    evt.jsonValue = obj.toString();
    evt.type = "eip712DomainChanged";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolMigrates.length; i++) {
    const e = events.ztakingpoolMigrates[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("amounts", e.amounts);
    obj.set("destination", e.destination);
    obj.set("eventId", e.eventId);
    obj.set("migrator", e.migrator);
    obj.set("tokens", e.tokens);
    obj.set("user", e.user);
    evt.jsonValue = obj.toString();
    evt.type = "migrate";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolOwnershipTransferStarteds.length; i++) {
    const e = events.ztakingpoolOwnershipTransferStarteds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("newOwner", e.newOwner);
    obj.set("previousOwner", e.previousOwner);
    evt.jsonValue = obj.toString();
    evt.type = "ownershipTransferStarted";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolOwnershipTransferreds.length; i++) {
    const e = events.ztakingpoolOwnershipTransferreds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("newOwner", e.newOwner);
    obj.set("previousOwner", e.previousOwner);
    evt.jsonValue = obj.toString();
    evt.type = "ownershipTransferred";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolPauseds.length; i++) {
    const e = events.ztakingpoolPauseds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("account", e.account);
    evt.jsonValue = obj.toString();
    evt.type = "paused";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolSignerChangeds.length; i++) {
    const e = events.ztakingpoolSignerChangeds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("newSigner", e.newSigner);
    evt.jsonValue = obj.toString();
    evt.type = "signerChanged";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolTokenStakabilityChangeds.length; i++) {
    const e = events.ztakingpoolTokenStakabilityChangeds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("enabled", e.enabled);
    obj.set("token", e.token);
    evt.jsonValue = obj.toString();
    evt.type = "tokenStakabilityChanged";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolUnpauseds.length; i++) {
    const e = events.ztakingpoolUnpauseds[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("account", e.account);
    evt.jsonValue = obj.toString();
    evt.type = "unpaused";
    evt.save();
  }
  
  for (let i = 0; i < events.ztakingpoolWithdraws.length; i++) {
    const e = events.ztakingpoolWithdraws[i];
    let evt = new Event(ID(e.evtTxHash, i));
    let obj = new JSON.Obj();
    obj.set("evtTxHash", e.evtTxHash);
    obj.set("evtIndex", e.evtIndex);
    obj.set("evtBlockTime", e.evtBlockTime);
    obj.set("evtBlockNumber", e.evtBlockNumber);
    obj.set("amount", e.amount);
    obj.set("eventId", e.eventId);
    obj.set("token", e.token);
    obj.set("withdrawer", e.withdrawer);
    evt.jsonValue = obj.toString();
    evt.type = "withdraw";
    evt.save();
  }
  
}

function ID(trxHash: string, i: u32): string {
  return trxHash + "-" + i.toString();
}
