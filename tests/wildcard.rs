use http_types::{Body, Method, StatusCode};
use tide::{http, Request, Response};

async fn add_one(cx: Request<()>) -> Result<String, tide::Error> {
    return match cx.param::<i64>("num"){
        Ok(num) => Ok((num + 1).to_string()),
        Err(err) => Err(tide::Error::new(StatusCode::BadRequest, err))
    }
}

async fn add_two(cx: Request<()>) -> Result<String, tide::Error> {
    let one;
    match cx.param::<i64>("one") {
        Ok(num) => one = num,
        Err(err) => return Err(tide::Error::new(StatusCode::BadRequest, err)),
    };
    let two;
    match cx.param::<i64>("two") {
        Ok(num) => two = num,
        Err(err) => return Err(tide::Error::new(StatusCode::BadRequest, err)),
    };
    Ok((one + two).to_string())
}

async fn echo_path(cx: Request<()>) -> Result<String, tide::Error> {
    return match cx.param::<String>("path"){
        Ok(path) => Ok(path),
        Err(err) => Err(tide::Error::new(StatusCode::BadRequest, err))
    }
}

// async fn echo_empty(cx: Request<()>) -> Result<String, tide::Error> {
//     let path: String = cx.param("").client_err()?;
//     Ok(path)
// }

#[async_std::test]
async fn wildcard() {
    let mut app = tide::Server::new();
    app.at("/add_one/:num").get(add_one);

    let req = http::Request::new(Method::Get, "http://localhost/add_one/3".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::Ok);
    let body = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"4");

    let req = http::Request::new(Method::Get, "http://localhost/add_one/-7".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::Ok);
    let body = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"-6");
}

#[async_std::test]
async fn invalid_segment_error() {
    let mut app = tide::new();
    app.at("/add_one/:num").get(add_one);

    let req = http::Request::new(Method::Get, "http://localhost/add_one/a".parse().unwrap());
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BadRequest);
}

#[async_std::test]
async fn not_found_error() {
    let mut app = tide::new();
    app.at("/add_one/:num").get(add_one);

    let req = http::Request::new(Method::Get, "http://localhost/add_one/".parse().unwrap());
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NotFound);
}

#[async_std::test]
async fn wild_path() {
    let mut app = tide::new();
    app.at("/echo/*path").get(echo_path);

    let req = http::Request::new(Method::Get, "http://localhost/echo/some_path".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::Ok);
    let body: String = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"some_path");

    let req = http::Request::new(Method::Get, "http://localhost/echo/multi/segment/path".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::Ok);
    let body: String = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"multi/segment/path");

    let req = http::Request::new(Method::Get, "http://localhost/echo/".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NotFound);
    let body: String = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"");
}

#[async_std::test]
async fn multi_wildcard() {
    let mut app = tide::new();
    app.at("/add_two/:one/:two/").get(add_two);

    let req = http::Request::new(Method::Get, "http://localhost/add_two/1/2/".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::Ok);
    let body: String = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"3");

    let req = http::Request::new(Method::Get, "http://localhost/add_two/-1/2/".parse().unwrap());
    let mut res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
    let body: String = res.take_body().into_string().await.unwrap();
    assert_eq!(body.as_bytes(), b"1");

    let req = http::Request::new(Method::Get, "http://localhost/add_two/1".parse().unwrap());
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::NotFound);
}

// #[test]
// fn wild_last_segment() {
//     let mut app = tide::Server::new();
//     app.at("/echo/:path/*").get(echo_path);
//     let mut server = make_server(app.into_http_service()).unwrap();

//     let req = http::Request::get("/echo/one/two")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 200);
//     let body = block_on(res.into_body().into_vec()).unwrap();
//     assert_eq!(&*body, &*b"one");

//     let req = http::Request::get("/echo/one/two/three/four")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 200);
//     let body = block_on(res.into_body().into_vec()).unwrap();
//     assert_eq!(&*body, &*b"one");
// }

// #[test]
// fn invalid_wildcard() {
//     let mut app = tide::Server::new();
//     app.at("/echo/*path/:one/").get(echo_path);
//     let mut server = make_server(app.into_http_service()).unwrap();

//     let req = http::Request::get("/echo/one/two")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 404);
// }

// #[test]
// fn nameless_wildcard() {
//     let mut app = tide::Server::new();
//     app.at("/echo/:").get(|_| async move { "" });

//     let mut server = make_server(app.into_http_service()).unwrap();

//     let req = http::Request::get("/echo/one/two")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 404);

//     let req = http::Request::get("/echo/one").body(Body::empty()).unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 200);
// }

// #[test]
// fn nameless_internal_wildcard() {
//     let mut app = tide::Server::new();
//     app.at("/echo/:/:path").get(echo_path);
//     let mut server = make_server(app.into_http_service()).unwrap();

//     let req = http::Request::get("/echo/one").body(Body::empty()).unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 404);

//     let req = http::Request::get("/echo/one/two")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 200);
//     let body = block_on(res.into_body().into_vec()).unwrap();
//     assert_eq!(&*body, &*b"two");

//     let req = http::Request::get("/echo/one/two")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 200);
//     let body = block_on(res.into_body().into_vec()).unwrap();
//     assert_eq!(&*body, &*b"two");
// }

// #[test]
// fn nameless_internal_wildcard2() {
//     let mut app = tide::Server::new();
//     app.at("/echo/:/:path").get(echo_empty);
//     let mut server = make_server(app.into_http_service()).unwrap();

//     let req = http::Request::get("/echo/one/two")
//         .body(Body::empty())
//         .unwrap();
//     let res = server.simulate(req).unwrap();
//     assert_eq!(res.status(), 200);
//     let body = block_on(res.into_body().into_vec()).unwrap();
//     assert_eq!(&*body, &*b"one");
// }
