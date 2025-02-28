use tokio::sync::mpsc::{Sender, Receiver};
use std::sync::OnceLock;
use timed_lru_cache::TypeErasedTimedLruCache;

pub enum CacheCommand {
    Get{ key: CacheKey, reply: tokio::sync::oneshot::Sender<Option<i32>> },
    Set{ key: CacheKey, value: i32, reply: tokio::sync::oneshot::Sender<()> },
    Quit { confirm: tokio::sync::oneshot::Sender<()> },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Integer(i32),
}

static ACTOR_STORE: OnceLock<Sender<CacheCommand>> = OnceLock::new();

pub async fn start_cache_actor() {
    let (tx, rx) = tokio::sync::mpsc::channel(1024);
    tokio::spawn(cache_actor(rx));
    let _ = ACTOR_STORE.set(tx);
}

async fn cache_actor(mut tx: Receiver<CacheCommand>) {
    let mut cache = TypeErasedTimedLruCache::<CacheKey>::new(10, 1.0);
    loop {
        let command = tx.recv().await;
        match command {
            Some(CacheCommand::Get { key, reply }) => {
                let result = cache.get(&key);
                if let Err(e) = reply.send(result) {
                    eprintln!("Failed to send reply: {:?}", e);
                }
            }
            Some(CacheCommand::Set { key, value, reply }) => {
                cache.insert(key, value);
                if let Err(e) = reply.send(()) {
                    eprintln!("Failed to send reply: {:?}", e);
                }
            }
            Some(CacheCommand::Quit { confirm }) => {
                confirm.send(()).unwrap();
                break;
            }
            None => {
                break;
            }
        }
    }
    println!("Cache actor exiting");
}

fn get_cache_actor() -> Sender<CacheCommand> {
    ACTOR_STORE.get().unwrap().clone()
}

pub async fn stop_cache_actor() {
    let (confirm_tx, confirm_rx) = tokio::sync::oneshot::channel();
    get_cache_actor().send(CacheCommand::Quit { confirm: confirm_tx }).await.unwrap();
    confirm_rx.await.unwrap();
}

pub async fn get_integer(key: i32) -> Option<i32> {
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    get_cache_actor().send(CacheCommand::Get {
        key: CacheKey::Integer(key),
        reply: reply_tx,
    }).await.unwrap();
    reply_rx.await.unwrap()
}

pub async fn store_integer(key: i32, value: i32) {
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    get_cache_actor().send(CacheCommand::Set {
        key: CacheKey::Integer(key),
        value,
        reply: reply_tx,
    }).await.unwrap();
    reply_rx.await.unwrap();
}