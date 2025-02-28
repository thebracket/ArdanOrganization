mod cache_actor;

#[tokio::main]
async fn main() {
    cache_actor::start_cache_actor().await;

    for i in 0..10 {
        cache_actor::store_integer(i, i * 2).await;
        let n = cache_actor::get_integer(i).await;
        println!("{}: {:?}", i, n);
    }

    cache_actor::stop_cache_actor().await;
}
