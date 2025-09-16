# async-result-ext

Async extensions for Rust’s [`Result<T, E>`].  
This crate provides asynchronous counterparts of the standard `Result` methods (`map`, `and_then`, `map_err`, `inspect`), allowing you to use async closures seamlessly.

---

## ✨ Motivation

The standard library’s `Result` methods (`map`, `and_then`, etc.) only work with **synchronous** closures.  
When writing async code, you often need to `.await` inside these transformations. This crate fills that gap.

Instead of:

```rust
let res: Result<i32, &str> = Ok(5);

let mapped = match res {
    Ok(v) => Ok(async { v * 2 }.await),
    Err(e) => Err(e),
};
```

You can just write:

```rust
use async_result_ext::AsyncResultExt;

let res: Result<i32, &str> = Ok(5);
let mapped = res.async_map(|v| async move { v * 2 }).await;

assert_eq!(mapped, Ok(10));
```

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
async-result-ext = "0.1.0"
```

---

## 🚀 Usage

```rust
use async_result_ext::AsyncResultExt;

#[tokio::main]
async fn main() {
    let r: Result<i32, &str> = Ok(2);

    // async_map: transform Ok values asynchronously
    let doubled = r.async_map(|v| async move { v * 2 }).await;
    assert_eq!(doubled, Ok(4));

    // async_and_then: chain async computations returning Result
    let chained = doubled.async_and_then(|v| async move { Ok(v + 3) }).await;
    assert_eq!(chained, Ok(7));

    // async_map_or: provide a default for Err cases
    let res = Err::<i32, &str>("fail");
    let fallback = res.async_map_or(100, |v| async move { v * 10 }).await;
    assert_eq!(fallback, 100);

    // async_map_err: transform errors asynchronously
    let err: Result<i32, &str> = Err("oops");
    let mapped_err = err.async_map_err(|e| async move { e.len() }).await;
    assert_eq!(mapped_err, Err(4));

    // async_inspect & async_inspect_err: peek into values without changing them
    let ok: Result<i32, &str> = Ok(42);
    ok.async_inspect(|v| async move {
        println!("Got value: {v}");
    }).await;

    let err: Result<i32, &str> = Err("fail");
    err.async_inspect_err(|e| async move {
        eprintln!("Error: {e}");
    }).await;
}
```

---

## 📖 Provided Methods

- `async_map` – async version of [`Result::map`]
- `async_and_then` – async version of [`Result::and_then`]
- `async_map_or` – async version of [`Result::map_or`]
- `async_map_or_else` – async version of [`Result::map_or_else`]
- `async_map_err` – async version of [`Result::map_err`]
- `async_inspect` – async version of [`Result::inspect`]
- `async_inspect_err` – async version of [`Result::inspect_err`]

---

## ⚡ Features

- Minimal and lightweight  
- No dependencies (except your async runtime, e.g. Tokio or async-std)  
- Familiar API – mirrors the standard library’s `Result` methods  

---

## 🔧 License

MIT License. See [LICENSE.txt](LICENSE) for details.
