use async_trait::async_trait;

use crate::module::Module;

pub mod mcmode;

#[async_trait]
pub trait Verifier {
    async fn verify(&self, module: &mut Module);
}
