macro_rules! assert_body {
    ($response:expr, $expected:expr) => {
        use actix_web::test;
        let body = String::from_utf8(test::read_body($response).await.as_ref().to_owned());
        assert_eq!(body, Ok($expected));
    };
}

pub(crate) use assert_body;
