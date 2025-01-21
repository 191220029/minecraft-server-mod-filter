use reqwest::blocking::Client;

use crate::module::Module;

pub mod mcmode;

pub trait Verifier {
    fn verify(&self, client: &Client, module: &mut Module);
}
