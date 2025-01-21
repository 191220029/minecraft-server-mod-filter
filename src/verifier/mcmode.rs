use async_trait::async_trait;
use log::{error, info};
use regex::Regex;

use crate::{module::ServerFlag, utils::get_body};

use super::Verifier;

pub struct McModeVerifier;

#[async_trait]
impl Verifier for McModeVerifier {
    async fn verify(&self, module: &mut crate::module::Module) {
        let url = format!(
            "https://search.mcmod.cn/s?key={}&site=&filter=1&mold=0",
            module.name.replace(' ', "+")
        );
        let body = if let Ok(body) = get_body(&url).await {
            body
        } else {
            error!("Fail to visit {}.", url);
            module.server_flag = ServerFlag::ModNotFound(url);
            return;
        };
        info!("successfully get body from {}.", url);

        let re = Regex::new(r#"https://www\.mcmod\.cn/class/[^"]+"#).unwrap();

        // Iterate over all matches
        for capture in re.captures_iter(&body) {
            let mod_url = capture.get(0).unwrap().as_str();
            let body = if let Ok(body) = get_body(mod_url).await {
                body
            } else {
                error!("Fail to visit {}.", mod_url);
                module.server_flag = ServerFlag::ServerInfoNotFound(mod_url.to_string());
                return;
            };
            info!("successfully get body from {}.", mod_url);

            if body.contains("服务端无效") {
                info!("Mark {} as {:?}.", module.name, ServerFlag::ServerInvalid);
                module.server_flag = ServerFlag::ServerInvalid;
                return;
            } else {
                info!("Mark {} as {:?}.", module.name, ServerFlag::ServerNeeded);
                module.server_flag = ServerFlag::ServerNeeded
            }
        }

        error!("Fail to get search results from {}.", url);
        module.server_flag = ServerFlag::ModNotFound(url);
        return;
    }
}

#[cfg(test)]
mod test_mc_mode_verify {
    use crate::{
        module::{Module, ServerFlag},
        verifier::{mcmode::McModeVerifier, Verifier},
    };

    #[ignore]
    #[test]
    fn test_mc_mode_verify() {
        let mut module = Module::new("Just Enough Resources", "");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { McModeVerifier.verify(&mut module).await });

        assert_eq!(module.server_flag, ServerFlag::ServerInvalid)
    }
}
