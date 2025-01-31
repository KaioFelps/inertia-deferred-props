use serde::Serialize;

#[derive(Serialize)]
pub struct User<'a> {
    pub id: u32,
    pub name: &'a str,
}
