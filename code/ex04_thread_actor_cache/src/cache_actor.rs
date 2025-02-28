use std::sync::mpsc::{Sender, Receiver};
use std::sync::OnceLock;
use timed_lru_cache::TypeErasedTimedLruCache;

pub enum CacheCommand {
    Get{ key: CacheKey, reply: oneshot::Sender<Option<i32>> },
    Set{ key: CacheKey, value: i32, reply: oneshot::Sender<()> },
    Quit { confirm: oneshot::Sender<()> },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Integer(i32),
}

static ACTOR_STORE: OnceLock<Sender<CacheCommand>> = OnceLock::new();

pub fn start_cache_actor() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        cache_actor(rx);
    });
    let _ = ACTOR_STORE.set(tx);
}

fn cache_actor(tx: Receiver<CacheCommand>) {
    let mut cache = TypeErasedTimedLruCache::<CacheKey>::new(10, 1.0);
    loop {
        let command = tx.recv();
        match command {
            Ok(CacheCommand::Get { key, reply }) => {
                let result = cache.get(&key);
                if let Err(e) = reply.send(result) {
                    eprintln!("Failed to send reply: {:?}", e);
                }
            }
            Ok(CacheCommand::Set { key, value, reply }) => {
                cache.insert(key, value);
                if let Err(e) = reply.send(()) {
                    eprintln!("Failed to send reply: {:?}", e);
                }
            }
            Ok(CacheCommand::Quit { confirm }) => {
                confirm.send(()).unwrap();
                break;
            }
            Err(e) => {
                eprintln!("Error receiving command: {:?}", e);
                break;
            }
        }
    }
    println!("Cache actor exiting");
}

fn get_cache_actor() -> Sender<CacheCommand> {
    ACTOR_STORE.get().unwrap().clone()
}

pub fn stop_cache_actor() {
    let (confirm_tx, confirm_rx) = oneshot::channel();
    get_cache_actor().send(CacheCommand::Quit { confirm: confirm_tx }).unwrap();
    confirm_rx.recv().unwrap();
}

pub fn get_integer(key: i32) -> Option<i32> {
    let (reply_tx, reply_rx) = oneshot::channel();
    get_cache_actor().send(CacheCommand::Get {
        key: CacheKey::Integer(key),
        reply: reply_tx,
    }).unwrap();
    reply_rx.recv().unwrap()
}

pub fn store_integer(key: i32, value: i32) {
    let (reply_tx, reply_rx) = oneshot::channel();
    get_cache_actor().send(CacheCommand::Set {
        key: CacheKey::Integer(key),
        value,
        reply: reply_tx,
    }).unwrap();
    reply_rx.recv().unwrap();
}