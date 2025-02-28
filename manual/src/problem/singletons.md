# Singletons

There *are* systems of which only one should ever exist. Global configuration, for example. Rust even supports a type designed to make a static singleton that can only be initialized once: `OnceLock`. There are also `Lazy` and similar types to assist with making static singletones.

## OnceLock Usage

```rust
use std::sync::OnceLock;

static JUST_ONCE: OnceLock<usize> = OnceLock::new();

fn main() {
    // Set once with `get_or_init`
    JUST_ONCE.get_or_init(|| {
        5
    });

    let Some(n) = JUST_ONCE.get() else { panic!("Not initialized!"); };
    println!("{n}");
}    
```

`OnceLock` exists for shared state that will only be initialized once. It's immutable - but the contents are subject to Rust's "interior mutability" rule. So if the structure you place inside can be locked (either internally or externally), you can use it for mutable data.

## Lazy Usage

```rust
use std::sync::LazyLock;
use std::sync::Mutex;

static LAZY: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("Hello".to_string()));

fn main() {
    println!("{}", LAZY.lock().unwrap());
}

```

## Don't Do This. Please.

```rust
static mut MY_SHARED: i32 = 12;

fn main() {
    unsafe { MY_SHARED = 13; }
    println!("{}", unsafe { MY_SHARED });
}
```

This won't even compile in Rust Edition 2024, but you *could* do it in previous Rust editions. It's an invitation to data races, corruption and pain. **Don't do this**!


> The big takeaway is that Rust doesn't *stop* you from having global shared state, but `static mut` is prohibited as of Rust's 2024 edition.