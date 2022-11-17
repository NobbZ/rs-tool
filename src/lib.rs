use actix_web::{get, post, HttpResponse, Responder};

#[cfg(test)]
pub(crate) mod test_helper;

#[macro_export]
macro_rules! app {
    () => {{
        use $crate::{echo, hello, manual_hello};
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(hello)
            .service(echo)
            .route("/hey", actix_web::web::get().to(manual_hello))
    }};
}

#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello, World!"
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};
    use rstest::*;

    use crate::test_helper::assert_body;

    use super::*;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(app!()).await;
        let req = test::TestRequest::default().to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(test::read_body(res).await, "Hello, World!".as_bytes())
    }

    #[rstest]
    #[case("")]
    #[case("some short text")]
    #[case("some even longer text, that I can not really come up with and therefore say LOREM IPSUM, DOLOR ET SIMET!")]
    #[actix_web::test]
    async fn test_echo(#[case] text: String) {
        let app = test::init_service(app!()).await;
        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload(text.clone())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        assert_body!(res, text);
    }
}
