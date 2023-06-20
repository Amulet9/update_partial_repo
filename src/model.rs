#[derive(ormlite::Model, Debug)]
pub struct Test {
    #[ormlite(primary_key)]
    pub property: u32,
}