pub async fn do_something_async() -> i32 {
    let x = async { do_something_async_internal() };

    x.await.await
}

async fn do_something_async_internal() -> i32 {
    1
}
