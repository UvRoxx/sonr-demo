use cosmwasm_std::{Addr, StdResult};
use cosmwasm_std::testing::MockStorage;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub totoal_people: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnyDataIWantOnChain {
    pub references_to_ipfs_obj: Vec<String>,
    pub some_imp_address: Addr,

}


// ITEM
pub const STATE: Item<State> = Item::new("state");
pub const ANYDATA: Item<AnyDataIWantOnChain> = Item::new("any_data");


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Data {
    pub name: String,
    pub age: i32,
}

// MAP
const PEOPLE: Map<&str, Data> = Map::new("people");

fn demo() -> StdResult<()> {
    let mut store = MockStorage::new();
    let data = Data {
        name: "John".to_string(),
        age: 32,
    };

    // load and save with extra key argument
    let empty = PEOPLE.may_load(&store, "john")?;
    assert_eq!(None, empty);
    PEOPLE.save(&mut store, "john", &data)?;
    let loaded = PEOPLE.load(&store, "john")?;
    assert_eq!(data, loaded);

    // nothing on another key
    let missing = PEOPLE.may_load(&store, "jack")?;
    assert_eq!(None, missing);

    // update function for new or existing keys
    let birthday = |d: Option<Data>| -> StdResult<Data> {
        match d {
            Some(one) => Ok(Data {
                name: one.name,
                age: one.age + 1,
            }),
            None => Ok(Data {
                name: "Newborn".to_string(),
                age: 0,
            }),
        }
    };

    let old_john = PEOPLE.update(&mut store, "john", birthday)?;
    assert_eq!(33, old_john.age);
    assert_eq!("John", old_john.name.as_str());

    let new_jack = PEOPLE.update(&mut store, "jack", birthday)?;
    assert_eq!(0, new_jack.age);
    assert_eq!("Newborn", new_jack.name.as_str());

    // update also changes the store
    assert_eq!(old_john, PEOPLE.load(&store, "john")?);
    assert_eq!(new_jack, PEOPLE.load(&store, "jack")?);

    // removing leaves us empty
    PEOPLE.remove(&mut store, "john");
    let empty = PEOPLE.may_load(&store, "john")?;
    assert_eq!(None, empty);

    Ok(())
}


//ALLOWANCE

const ALLOWANCE: Map<(&str, &str), u64> = Map::new("allow");

fn demo_allowance() -> StdResult<()> {
    let mut store = MockStorage::new();

    // save and load on a composite key
    let empty = ALLOWANCE.may_load(&store, ("owner", "spender"))?;
    assert_eq!(None, empty);
    ALLOWANCE.save(&mut store, ("owner", "spender"), &777)?;
    let loaded = ALLOWANCE.load(&store, ("owner", "spender"))?;
    assert_eq!(777, loaded);

    // doesn't appear under other key (even if a concat would be the same)
    let different = ALLOWANCE.may_load(&store, ("owners", "pender")).unwrap();
    assert_eq!(None, different);

    // simple update
    ALLOWANCE.update(&mut store, ("owner", "spender"), |v| {
        Ok(v.unwrap_or_default() + 222)
    })?;
    let loaded = ALLOWANCE.load(&store, ("owner", "spender"))?;
    assert_eq!(999, loaded);

    Ok(())
}

