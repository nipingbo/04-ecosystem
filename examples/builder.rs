use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

use anyhow::Result;

#[allow(unused)]
#[derive(Debug, Builder)]
#[builder(build_fn(name = "priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into, strip_option), default)]
    email: Option<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User::build()
        .name("Alice")
        .skill("Programming")
        .skill("Debugging")
        .email("raymond.ni@sap.com")
        .dob("1990-01-01T00:00:00Z")
        .build()?;

    println!("{:?}", user);
    Ok(())
}

impl User {
    pub fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(&self) -> Result<User> {
        let mut user = self.priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as u32;
        Ok(user)
    }

    pub fn dob(&mut self, dob: &str) -> &mut Self {
        self.dob = DateTime::parse_from_rfc3339(dob)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();
        self
    }
}
