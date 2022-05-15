mod test;

#[derive(Debug)]
pub struct IntegrationTest {
    pub name: &'static str,
    pub test_fn: fn(),
    pub indev: Option<bool>,
}

inventory::collect!(IntegrationTest);
