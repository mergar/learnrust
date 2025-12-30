use warp::Filter;

#[tokio::main]
async fn main() {
    // Создаем маршрут, который отвечает "hello world" на любой GET запрос
    let hello = warp::get().map(|| "hello world");

    // Запускаем сервер на порту 3000
    warp::serve(hello)
        .run(([0, 0, 0, 0], 6000))
        .await;
}
