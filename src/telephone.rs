use serenity::{
    model::id::{ChannelId, UserId},
    prelude::{TypeMapKey},
};

use log::{info, warn};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fmt,
    fs::File,
    io::{Error as IOError, Write},
};

#[derive(Debug)]
pub struct Chain {
    open: bool,
    name: String,
    queue: VecDeque<UserId>,
    creator: UserId,
    created_in_channel: ChannelId,
}

#[derive(Debug)]
pub enum ChainErrorType {
    DoesNotExist(String),
    NotOpen(String),
    AlreadyExists(String),
    NotYourTurn(String, UserId),
    Io(IOError),
}

#[derive(Debug)]
pub struct ChainError {
    which: ChainErrorType,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.which {
            ChainErrorType::DoesNotExist(name) => write!(f, "Chain '{}' does not exist!", name),
            ChainErrorType::NotOpen(name) => write!(f, "Chain '{}' is not open yet!", name),
            ChainErrorType::AlreadyExists(name) => {
                write!(f, "A chain called '{}' already exists!", name)
            }
            ChainErrorType::NotYourTurn(name, userid) => {
                write!(f, "It is not user {}'s turn in chain '{}'", userid, name)
            }
            ChainErrorType::Io(why) => why.fmt(f),
        }
    }
}

impl From<ChainErrorType> for ChainError {
    fn from(which: ChainErrorType) -> Self {
        ChainError { which: which }
    }
}

impl Error for ChainError {}

type ChainResult<T> = Result<T, ChainError>;
pub type ChainMap = HashMap<String, Chain>;

pub struct ChainStorage;
impl TypeMapKey for ChainStorage {
    type Value = ChainMap;
}

fn add_to_chain(chain: &mut Chain, user: UserId) -> ChainResult<()> {
    if chain.open {
        chain.queue.push_back(user);
        info!("Added user {} to chain '{}'", user, chain.name);
        Ok(())
    } else {
        Err(ChainErrorType::NotOpen(chain.name.clone()).into())
    }
}

pub fn add_to_chain_map(map: &mut ChainMap, name: &String, user: UserId) -> ChainResult<()> {
    let chain = match map.get_mut(name) {
        Some(res) => res,
        None => {
            return Err(ChainErrorType::DoesNotExist(name.clone()).into());
        }
    };

    add_to_chain(chain, user)
}

fn create_chain(name: &String, user: UserId, channel: ChannelId) -> ChainResult<Chain> {
    // TODO: let this be more configurable
    info!("Creating chain '{}'", name);
    Ok(Chain {
        open: true,
        name: name.clone(),
        queue: VecDeque::new(),
        creator: user,
        created_in_channel: channel,
    })
}

pub fn create_chain_in_map(
    map: &mut ChainMap,
    name: &String,
    user: UserId,
    channel: ChannelId,
) -> ChainResult<()> {
    if map.contains_key(name) {
        Err(ChainErrorType::AlreadyExists(name.clone()).into())
    } else {
        let chain = create_chain(name, user, channel)?;
        let _ = map.insert(name.clone(), chain);
        Ok(())
    }
}

fn submit_to_chain(chain: &mut Chain, user: UserId, content: Vec<u8>) -> ChainResult<()> {
    // TODO: add better way to store images
    let fname = format!("{}_{}.png", chain.name, user);
    let mut file = match File::create(&fname) {
        Ok(res) => res,
        Err(why) => {
            warn!("Error creating file: {:?}", why);
            return Err(ChainErrorType::Io(why).into());
        }
    };

    if let Err(why) = file.write(&content) {
        warn!("Error writing to file: {:?}", why);

        return Err(ChainErrorType::Io(why).into());
    }

    info!("Did submit data from {} to chain '{}'", user, chain.name);
    Ok(())
}

pub fn submit_to_chain_map(
    map: &mut ChainMap,
    name: &String,
    user: UserId,
    content: Vec<u8>,
) -> ChainResult<()> {
    let chain = match map.get_mut(name) {
        Some(res) => res,
        None => {
            return Err(ChainErrorType::DoesNotExist(name.clone()).into());
        }
    };

    submit_to_chain(chain, user, content)
}
