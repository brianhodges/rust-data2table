#[derive(Debug)]
pub struct Country<'a> {
    pub name: &'a str,
    pub population: &'a str,
}