mod cache_actor;

fn main() {
    cache_actor::start_cache_actor();

    for i in 0..10 {
        cache_actor::store_integer(i, i * 2);
        let n = cache_actor::get_integer(i);
        println!("{}: {:?}", i, n);
    }

    cache_actor::stop_cache_actor();
}
