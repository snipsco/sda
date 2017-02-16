
pub trait KeyService {
    // TODO
}

// use super::*;

// pub trait SecurityAgent { // TODO rename to
//     fn save_authenticator(&mut self, server: &str, authenticator: &str) -> SdaClientResult<()>;
//     fn seal(pk: &RawData, data: &[u8]) -> SdaClientResult<Vec<u8>>;
//     fn unseal(&self, data: &[u8]) -> SdaClientResult<Vec<u8>>;
// }

// mod file {

//     use std::path::{Path, PathBuf};
//     use std::collections::HashMap;

//     #[derive(Debug)]
//     pub struct FileSecurityAgent {
//         storagePath: PathBuf,
//         state: FileSecurityAgentState,
//     }

//     #[derive(Debug,Serialize,Deserialize)]
//     struct FileSecurityAgentState {
//         id: Uuid,
//         alias: Option<String>,
//         encryption_key: EncryptionKey,
//         service_auth_tokens: HashMap<String, String>,
//     }

//     impl FileSecurityAgent {

//         pub fn new<P: AsRef<Path>>(identity: &str,
//                                    alias: Option<&str>,
//                                    storage: P)
//                                    -> SdaClientResult<Agent> {
//             let filename = storage.as_ref().join(identity);
//             if !filename.exists() {
//                 try!(Agent::init_state(&filename));
//             }
//             let state = try!(Agent::load(&filename));
//             let mut agent = Agent {
//                 storage: filename,
//                 state: state,
//             };
//             if let Some(a) = alias {
//                 if agent.state.alias.is_none() || agent.state.alias.as_ref().unwrap() != a {
//                     agent.state.alias = Some(a.into());
//                     try!(agent.save());
//                 }
//             }
//             Ok(agent)
//         }

//         fn init_state<P: AsRef<Path>>(filename: P) -> SdaClientResult<()> {
//             let id: String = ::rand::thread_rng().gen_ascii_chars().take(8).collect();
//             let (pk, sk) = Self::gen_keys(&*id);
//             let state = AgentState {
//                 id: id,
//                 alias: None,
//                 pk: pk,
//                 sk: sk,
//                 server_authenticators: HashMap::new(),
//             };
//             let mut writer = try!(fs::File::create(&filename)
//                 .chain_err(|| format!("failed to create {:?}", &filename.as_ref())));
//             try!(to_writer(&mut writer, &state));
//             Ok(())
//         }

//         fn load<P: AsRef<Path>>(filename: P) -> SdaClientResult<AgentState> {
//             let mut reader = try!(fs::File::open(&filename)
//                 .chain_err(|| format!("failed to read {:?}", filename.as_ref())));
//             Ok(try!(from_reader(&mut reader)))
//         }

//         fn save(&mut self) -> SdaClientResult<()> {
//             let mut writer = try!(fs::File::create(&self.storage)
//                 .chain_err(|| format!("failed to write {:?}", self.storage)));
//             try!(to_writer(&mut writer, &self.state));
//             Ok(())
//         }

//         fn gen_keys(id: &str) -> (PublicKey, SecretKey) {
//             let (pk, sk) = crypto::box_::gen_keypair();
//             (PublicKey {
//                 id: id.to_string(),
//                 key: RawData(pk.as_ref().to_vec()),
//                 authenticator: None,
//             },
//              SecretKey {
//                  id: id.to_string(),
//                  key: RawData(sk[..].to_vec()),
//                  authenticator: None,
//              })
//         }

//     }

//     impl SecurityAgent for FileSecurityAgent {

//         fn save_authenticator(&mut self, server: &str, authenticator: &str) -> SdaClientResult<()> {
//             let server: String = server.into();
//             let authenticator: String = authenticator.into();
//             if self.state.server_authenticators.get(&server) == Some(&authenticator) {
//                 return Ok(());
//             }
//             self.state.server_authenticators.insert(server, authenticator);
//             try!(self.save());
//             Ok(())
//         }

//         fn seal(pk: &RawData, data: &[u8]) -> SdaClientResult<Vec<u8>> {
//             let pk = try!(crypto::box_::PublicKey::from_slice(&*pk.0)
//                 .ok_or("could not decode public key"));
//             Ok(crypto::sealedbox::seal(data, &pk))
//         }

//         fn unseal(&self, data: &[u8]) -> SdaClientResult<Vec<u8>> {
//             let pk = try!(crypto::box_::PublicKey::from_slice(&*self.state.pk.key.0)
//                 .ok_or("could not decode public key"));
//             let sk = try!(crypto::box_::SecretKey::from_slice(&*self.state.sk.key.0)
//                 .ok_or("could not decode secret key"));
//             Ok(try!(crypto::sealedbox::open(data, &pk, &sk).map_err(|_| "decryption failure")))
//         }

//     }

//     #[cfg(test)]
//     mod tests {

//         extern crate tempdir;
//         use super::Agent;

//         #[test]
//         fn test_alias() {
//             let tmp = tempdir::TempDir::new("test-sda-agent").unwrap();

//             let agent = Agent::new("foo", None, &tmp).unwrap();
//             assert_eq!(agent.state.alias, None);

//             let agent = Agent::new("foo", None, &tmp).unwrap();
//             assert_eq!(agent.state.alias, None);

//             let agent = Agent::new("foo", Some("alias"), &tmp).unwrap();
//             assert_eq!(agent.state.alias, Some("alias".into()));

//             let agent = Agent::new("foo", None, &tmp).unwrap();
//             assert_eq!(agent.state.alias, Some("alias".into()));

//             let agent = Agent::new("foo", Some("new_name"), &tmp).unwrap();
//             assert_eq!(agent.state.alias, Some("new_name".into()));
//         }
//     }

// }
// pub use file::*;