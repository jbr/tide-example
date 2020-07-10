pub mod articles;

pub mod welcome {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "welcome.html")]
    pub struct WelcomeTemplate<'a> {
        name: &'a str,
    }

    impl<'a> WelcomeTemplate<'a> {
        pub fn new(name: &'a str) -> Self {
            Self { name }
        }
    }
}
