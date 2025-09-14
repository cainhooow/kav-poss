use salvo::{
    Request, Response,
    http::cookie::{Cookie, CookieBuilder, Expiration, SameSite},
};
use time::{Duration, OffsetDateTime};

#[derive(Default, Debug)]
pub struct CookieService {}

impl CookieService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn session_cookie<'c>(
        &'c self,
        name: impl Into<String>,
        value: impl Into<String>,
        duration: Expiration,
    ) -> Cookie<'static> {
        let cookie_builder = CookieBuilder::new(name.into(), value.into())
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(duration);

        let cookie = cookie_builder.build();
        cookie
    }

    pub fn generate_sessions(
        &self,
        access_token: &str,
        refresh_token: &str,
        res: &mut Response,
    ) {
        let access_cookie = self.session_cookie(
            "kvsession",
            access_token,
            salvo::http::cookie::Expiration::DateTime(
                OffsetDateTime::now_utc() + Duration::minutes(15),
            ),
        );

        let refresh_cookie = self.session_cookie(
            "kvrefresh",
            refresh_token,
            salvo::http::cookie::Expiration::DateTime(
                OffsetDateTime::now_utc() + Duration::days(7),
            ),
        );

        res.add_cookie(access_cookie);
        res.add_cookie(refresh_cookie);
    }
}
