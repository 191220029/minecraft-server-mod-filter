use log::{error, info, warn};
use regex::Regex;
use reqwest::blocking::Client;

use crate::{module::ServerFlag, utils::get_body};

use super::Verifier;

pub struct McModeVerifier;

impl Verifier for McModeVerifier {
    fn verify(&self, client: &Client, module: &mut crate::module::Module) {
        let url = format!(
            "https://search.mcmod.cn/s?key={}&site=&filter=1&mold=0",
            module.name.replace(' ', "+")
        );
        let body = if let Ok(body) = get_body(client, &url) {
            body
        } else {
            error!("Fail to visit {}.", url);
            module.server_flag = ServerFlag::ModNotFound(url);
            return;
        };

        if body.contains("<p class=\"warning\">搜索太频繁，请稍后再试。</p>") {
            warn!("too frequent search. hold on a tick");
            panic!("too frequent search. hold on a tick");
        }

        info!("successfully get body from {}", url);

        let re = Regex::new(r#"https://www\.mcmod\.cn/class/[^"]+"#).unwrap();

        // Iterate over all matches
        for capture in re.captures_iter(&body) {
            let mod_url = capture.get(0).unwrap().as_str();
            let body = if let Ok(body) = get_body(client, mod_url) {
                body
            } else {
                error!("Fail to visit {}.", mod_url);
                module.server_flag = ServerFlag::ServerInfoNotFound(mod_url.to_string());
                return;
            };
            info!("successfully get body from {}", mod_url);

            if body.contains("服务端无效") {
                info!("Mark {} as {:?}.", module.name, ServerFlag::ServerInvalid);
                module.server_flag = ServerFlag::ServerInvalid;
                return;
            } else {
                info!("Mark {} as {:?}.", module.name, ServerFlag::ServerNeeded);
                module.server_flag = ServerFlag::ServerNeeded;
                return;
            }
        }

        error!("Fail to get search results from {}.", url);
        module.server_flag = ServerFlag::ModNotFound(url);
        return;
    }
}

#[cfg(test)]
mod test_mc_mode_verify {
    use reqwest::blocking::Client;

    use crate::{
        module::{Module, ServerFlag},
        verifier::{mcmode::McModeVerifier, Verifier},
    };

    #[ignore]
    #[test]
    fn test_mc_mode_verify() {
        env_logger::init();
        let mut module = Module::new("Just Enough Resources", "");
        McModeVerifier.verify(&Client::new(), &mut module);

        assert_eq!(module.server_flag, ServerFlag::ServerInvalid)
    }
}
