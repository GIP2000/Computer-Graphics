use anyhow::Result;
pub trait Bindable {
    fn bind(&self) -> Result<()>;
}
