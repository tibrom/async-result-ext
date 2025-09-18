use std::future::Future;

/// Asynchronous extensions for [`Result<T, E>`].
///
/// This trait provides async counterparts of common `Result` methods
/// (`map`, `and_then`, `map_err`, `inspect`) that accept asynchronous closures.
///
/// Example:
/// ```
/// use async_result_ext::AsyncResultExt;
///
/// #[tokio::main]
/// async fn main() {
///     let r: Result<i32, &str> = Ok(2);
///
///     let doubled = r.async_map(|v| async move { v * 2 }).await;
///     assert_eq!(doubled, Ok(4));
/// }
/// ```
pub trait AsyncResultExt<T, E> {
    /// Asynchronous version of [`Result::map`].
    ///
    /// Applies an async function `op` to the `Ok` value.
    /// If the result is `Err`, it is returned unchanged.
    ///
    /// ```
    /// use async_result_ext::AsyncResultExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let r: Result<i32, &str> = Ok(5);
    /// let res = r.async_map(|v| async move { v + 1 }).await;
    /// assert_eq!(res, Ok(6));
    /// # }
    /// ```
    fn async_map<U, F, Fut>(self, op: F) -> impl Future<Output = Result<U, E>>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U>;

    /// Asynchronous version of [`Result::and_then`].
    ///
    /// Chains async computations that return `Result`.
    ///
    /// ```
    /// use async_result_ext::AsyncResultExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let r: Result<i32, &str> = Ok(2);
    /// let res = r.async_and_then(|v| async move { Ok(v * 3) }).await;
    /// assert_eq!(res, Ok(6));
    /// # }
    /// ```
    fn async_and_then<U, F, Fut>(self, op: F) -> impl Future<Output = Result<U, E>>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Result<U, E>>;

    /// Asynchronous version of [`Result::map_or`].
    ///
    /// If the result is `Ok`, applies async function `op`.
    /// If `Err`, returns the provided `default` value.
    fn async_map_or<U, F, Fut>(self, default: U, op: F) -> impl Future<Output = U>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U>;

    /// Asynchronous version of [`Result::map_or_else`].
    ///
    /// If the result is `Ok`, applies async function `op`.
    /// If `Err`, computes an async fallback via `default`.
    fn async_map_or_else<U, D, F, Fut, DefFut>(self, default: D, op: F) -> impl Future<Output = U>
    where
        D: FnOnce(E) -> DefFut,
        F: FnOnce(T) -> Fut,
        DefFut: Future<Output = U>,
        Fut: Future<Output = U>;

    /// Asynchronous version of [`Result::map_err`].
    ///
    /// Transforms the error using an async function `op`.
    fn async_map_err<F, Fut, O>(self, op: F) -> impl Future<Output = Result<T, O>>
    where
        F: FnOnce(E) -> Fut,
        Fut: Future<Output = O>;

    /// Asynchronous version of [`Result::inspect`].
    ///
    /// Lets you asynchronously “peek” into the `Ok` value without modifying it.
    fn async_inspect<F, Fut>(self, op: F) -> impl Future<Output = Self>
    where
        F: FnOnce(&T) -> Fut,
        Fut: Future<Output = ()>;

    /// Asynchronous version of [`Result::inspect_err`].
    ///
    /// Lets you asynchronously “peek” into the `Err` value without modifying it.
    fn async_inspect_err<F, Fut>(self, op: F) -> impl Future<Output = Self>
    where
        F: FnOnce(&E) -> Fut,
        Fut: Future<Output = ()>;

    fn async_is_ok_and<F, Fut>(self, op: F) -> impl Future<Output = bool>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = bool>;
}

impl<T, E> AsyncResultExt<T, E> for Result<T, E> {
    async fn async_map<U, F, Fut>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U>,
    {
        match self {
            Ok(value) => Ok(op(value).await),
            Err(err) => Err(err),
        }
    }

    async fn async_and_then<U, F, Fut>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Result<U, E>>,
    {
        match self {
            Ok(value) => op(value).await,
            Err(err) => Err(err),
        }
    }

    async fn async_map_or<U, F, Fut>(self, default: U, op: F) -> U
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U>,
    {
        match self {
            Ok(value) => op(value).await,
            Err(_) => default,
        }
    }
    async fn async_map_or_else<U, D, F, Fut, DefFut>(self, default: D, op: F) -> U
    where
        D: FnOnce(E) -> DefFut,
        F: FnOnce(T) -> Fut,
        DefFut: Future<Output = U>,
        Fut: Future<Output = U>,
    {
        match self {
            Ok(value) => op(value).await,
            Err(err) => default(err).await,
        }
    }

    async fn async_map_err<F, Fut, O>(self, op: F) -> Result<T, O>
    where
        F: FnOnce(E) -> Fut,
        Fut: Future<Output = O>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(op(err).await),
        }
    }

    async fn async_inspect<F, Fut>(self, op: F) -> Self
    where
        F: FnOnce(&T) -> Fut,
        Fut: Future<Output = ()>,
    {
        if let Ok(ref value) = self {
            op(value).await;
        }
        self
    }

    async fn async_inspect_err<F, Fut>(self, op: F) -> Self
    where
        F: FnOnce(&E) -> Fut,
        Fut: Future<Output = ()>,
    {
        if let Err(ref err) = self {
            op(err).await;
        }
        self
    }

    async fn async_is_ok_and<F, Fut>(self, op: F) -> bool
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = bool>,
    {
        {
            match self {
                Err(_) => false,
                Ok(v) => op(v).await,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_map() {
        let r: Result<i32, &str> = Ok(2);
        let res = r.async_map(|v| async move { v * 3 }).await;
        assert_eq!(res, Ok(6));

        let r: Result<i32, &str> = Err("error");
        let res = r.async_map(|v| async move { v * 3 }).await;
        assert_eq!(res, Err("error"));
    }

    #[tokio::test]
    async fn test_async_and_then() {
        let r: Result<i32, &str> = Ok(2);
        let res = r.async_and_then(|v| async move { Ok(v * 5) }).await;
        assert_eq!(res, Ok(10));

        let r: Result<i32, &str> = Err("fail");
        let res = r.async_and_then(|v| async move { Ok(v * 5) }).await;
        assert_eq!(res, Err("fail"));
    }

    #[tokio::test]
    async fn test_async_map_or() {
        let r: Result<i32, &str> = Ok(2);
        let res = r.async_map_or(100, |v| async move { v * 4 }).await;
        assert_eq!(res, 8);

        let r: Result<i32, &str> = Err("fail");
        let res = r.async_map_or(100, |v| async move { v * 4 }).await;
        assert_eq!(res, 100);
    }

    #[tokio::test]
    async fn test_async_map_err() {
        let r: Result<i32, &str> = Ok(10);
        let res = r.async_map_err(|e| async move { e.len() }).await;
        assert_eq!(res, Ok(10));

        let r: Result<i32, &str> = Err("fail");
        let res = r.async_map_err(|e| async move { e.len() }).await;
        assert_eq!(res, Err(4));
    }

    #[tokio::test]
    async fn test_async_inspect_err() {
        let r: Result<i32, &str> = Err("oops");
        let mut seen = "";
        let res = r
            .async_inspect_err(|e| async {
                seen = e;
            })
            .await;
        assert_eq!(res, Err("oops"));
        assert_eq!(seen, "oops");

        let r: Result<i32, &str> = Ok(10);
        let res = r
            .async_inspect_err(|e| async {
                seen = e;
            })
            .await;
        assert_eq!(res, Ok(10));
    }

    #[tokio::test]
    async fn test_async_map_or_else() {
        let r: Result<i32, &str> = Ok(3);
        let res = r
            .async_map_or_else(|e| async move { e.len() as i32 }, |v| async move { v * 2 })
            .await;
        assert_eq!(res, 6);

        let r: Result<i32, &str> = Err("error");
        let res = r
            .async_map_or_else(|e| async move { e.len() as i32 }, |v| async move { v * 2 })
            .await;
        assert_eq!(res, 5);
    }

    #[tokio::test]
    async fn async_is_ok_and() {
        let r: Result<i32, &str> = Ok(5);
        let res = r.async_is_ok_and(|_| async move { false }).await;
        assert!(!res);

        let r: Result<i32, &str> = Err("error");
        let res = r.async_is_ok_and(|_| async move { false }).await;
        assert!(!res);
    }
}
