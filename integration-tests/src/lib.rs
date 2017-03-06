extern crate rouille;
extern crate sda_protocol;
extern crate sda_server;
#[cfg(feature="http")]
extern crate sda_server_http;
#[cfg(feature="http")]
extern crate sda_client_http;
#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_term;
extern crate tempdir;

#[cfg(test)]
mod test {

    use std::{path, sync};
    use std::sync::Arc;

    use sda_server;
    use sda_protocol;

    use sda_server::SdaServer;
    use sda_protocol::{SdaAgentService, SdaAggregationService, SdaAdministrationService};

    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

    #[allow(dead_code)]
    static GLOBAL_PORT_OFFSET: AtomicUsize = ATOMIC_USIZE_INIT;
    static LOGS: sync::Once = sync::ONCE_INIT;

    fn ensure_logs() {
        use slog::DrainExt;
        LOGS.call_once(|| {
            let root = ::slog::Logger::root(::slog_term::streamer().stderr().use_utc_timestamp().build().fuse(), o!());
            ::slog_scope::set_global_logger(root);
        });
    }

    fn jfs_server(dir: &path::Path) -> Arc<SdaServer> {
        let agents = sda_server::jfs_stores::JfsAgentStore::new(dir.join("agents")).unwrap();
        let auth = sda_server::jfs_stores::JfsAuthStore::new(dir.join("auths")).unwrap();
        let agg = sda_server::jfs_stores::JfsAggregationsStore::new(dir.join("aggs")).unwrap();
        Arc::new(SdaServer {
            agent_store: Box::new(agents),
            auth_token_store: Box::new(auth),
            aggregation_store: Box::new(agg),
        })
    }

    fn new_agent() -> ::sda_protocol::Agent {
        sda_protocol::Agent {
            id: sda_protocol::AgentId::default(),
            verification_key: sda_protocol::Labeled {
                id: sda_protocol::VerificationKeyId::default(),
                body:
                    sda_protocol::VerificationKey::Sodium(sda_protocol::byte_arrays::B32::default()),
            },
        }
    }

    fn new_key_for_agent(alice: &sda_protocol::Agent) -> sda_protocol::SignedEncryptionKey {
        use sda_protocol::byte_arrays::*;
        sda_protocol::SignedEncryptionKey {
            body: sda_protocol::Labeled {
                id: sda_protocol::EncryptionKeyId::default(),
                body: sda_protocol::EncryptionKey::Sodium(B32::default()),
            },
            signer: alice.id,
            signature: sda_protocol::Signature::Sodium(B64::default()),
        }
    }

    fn with_server<F: Fn(Arc<SdaServer>, &SdaAgentService, &SdaAggregationService, &SdaAdministrationService) -> ()>(f: F) {
        let tempdir = ::tempdir::TempDir::new("sda-tests").unwrap();
        let server = jfs_server(tempdir.path());
        let server_for_service = server.clone();
        let agents: &SdaAgentService = &*server_for_service;
        let aggs: &SdaAggregationService = &*server_for_service;
        let admin: &SdaAdministrationService = &*server_for_service;
        f(server, agents, aggs, admin)
    }

    #[cfg(feature="http")]
    fn with_service<F: Fn(Arc<SdaServer>, &SdaAgentService, &SdaAggregationService, &SdaAdministrationService) -> ()>(f: F) {
        use std::sync::Ordering;
        ensure_logs();
        with_server(|server, _, _, _| {
            let running = Arc::new(sync::atomic::AtomicBool::new(true));
            let port_offset = GLOBAL_PORT_OFFSET.fetch_add(1, Ordering::SeqCst);
            let port = port_offset + 21000;
            let address = format!("127.0.0.1:{}", port);
            let server_for_thread = server.clone();
            let http_address = format!("http://{}/", address);
            let address_for_thread = address.clone();
            let running_for_thread = running.clone();
            let thread = ::std::thread::spawn(move || {
                let rouille_server = ::rouille::Server::new(address_for_thread, move |req| {
                        ::sda_server_http::handle(&*server_for_thread, req)
                    })
                    .unwrap();
                while running_for_thread.load(Ordering::SeqCst) {
                    rouille_server.poll();
                    ::std::thread::sleep(::std::time::Duration::new(0, 1000000));
                }
            });
            let http_client = ::sda_client_http::SdaHttpClient::new(&*http_address).unwrap();
            f(server, &http_client);
            running.store(false, Ordering::SeqCst);
        });
    }

    #[cfg(not(feature="http"))]
    fn with_service<F: Fn(Arc<SdaServer>, &SdaAgentService, &SdaAggregationService, &SdaAdministrationService) -> ()>(f: F) {
        ensure_logs();
        with_server(f)
    }

    #[test]
    pub fn ping() {
        with_service(|_, agents, aggs, admin| {
            agents.ping().unwrap();
            aggs.ping().unwrap();
            admin.ping().unwrap();
        });
    }

    #[test]
    pub fn agent_crud() {
        with_service(|_, agents, _, _| {
            let alice = new_agent();
            agents.create_agent(&alice, &alice).unwrap();
            let clone = agents.get_agent(&alice, &alice.id).unwrap();
            assert_eq!(Some(&alice), clone.as_ref());

            let bob = agents.get_agent(&alice, &sda_protocol::AgentId::default()).unwrap();
            assert!(bob.is_none());
        });
    }

    #[test]
    pub fn profile_crud() {
        with_service(|_, agents, _, _| {

            let alice = new_agent();

            agents.create_agent(&alice, &alice).unwrap();
            let no_profile = agents.get_profile(&alice, &alice.id).unwrap();
            assert!(no_profile.is_none());

            let alice_profile = sda_protocol::Profile {
                owner: alice.id,
                name: Some("alice".into()),
                ..sda_protocol::Profile::default()
            };
            agents.upsert_profile(&alice, &alice_profile).unwrap();

            let clone = agents.get_profile(&alice, &alice.id).unwrap();
            assert_eq!(Some(&alice_profile), clone.as_ref());

            let still_alice_profile = sda_protocol::Profile {
                owner: alice.id,
                name: Some("still alice".into()),
                ..sda_protocol::Profile::default()
            };
            agents.upsert_profile(&alice, &still_alice_profile).unwrap();

            let clone = agents.get_profile(&alice, &alice.id).unwrap();
            assert_eq!(Some(&still_alice_profile), clone.as_ref());
        });
    }

    #[test]
    pub fn profile_crud_acl() {
        with_service(|_, agents, _, _| {
            let alice = new_agent();

            let bob = new_agent();
            let alice_fake_profile = sda_protocol::Profile {
                owner: alice.id,
                name: Some("bob".into()),
                ..sda_protocol::Profile::default()
            };

            let denied = agents.upsert_profile(&bob, &alice_fake_profile);
            match denied {
                Err(sda_protocol::SdaError(sda_protocol::SdaErrorKind::PermissionDenied, _)) => {}
                e => panic!("unexpected result: {:?}", e),
            }
        });
    }

    #[test]
    pub fn encryption_key_crud() {
        use sda_protocol::byte_arrays::*;
        with_service(|_, agents, _, _| {
            let alice = new_agent();
            let bob = new_agent();
            agents.create_agent(&alice, &alice).unwrap();

            let alice_key = sda_protocol::SignedEncryptionKey {
                body: sda_protocol::Labeled {
                    id: sda_protocol::EncryptionKeyId::default(),
                    body: sda_protocol::EncryptionKey::Sodium(B32::default()),

                },
                signer: alice.id,
                signature: sda_protocol::Signature::Sodium(B64::default()),
            };

            agents.create_encryption_key(&alice, &alice_key).unwrap();
            let still_alice = agents.get_encryption_key(&bob, &alice_key.body.id).unwrap();
            assert_eq!(Some(&alice_key), still_alice.as_ref());
        });
    }

    #[test]
    pub fn auth_tokens_crud() {
        use sda_server::stores::AuthToken;
        with_service(|server, agents, _, _| {
            let alice = new_agent();
            let alice_token = AuthToken {
                id: alice.id,
                body: "tok".into(),
            };
            assert!(server.check_auth_token(&alice_token).is_err());
            // TODO check error kind is InvalidCredentials
            agents.create_agent(&alice, &alice).unwrap();
            server.upsert_auth_token(&alice_token).unwrap();
            assert!(server.check_auth_token(&alice_token).is_ok());
            let alice_token_new = AuthToken {
                id: alice.id,
                body: "token".into(),
            };
            assert!(server.check_auth_token(&alice_token_new).is_err());
            server.upsert_auth_token(&alice_token_new).unwrap();
            assert!(server.check_auth_token(&alice_token_new).is_ok());
            assert!(server.check_auth_token(&alice_token).is_err());
            server.delete_auth_token(&alice.id).unwrap();
            assert!(server.check_auth_token(&alice_token_new).is_err());
            assert!(server.check_auth_token(&alice_token).is_err());
        });
    }

    #[test]
    pub fn aggregation_crud() {
        with_service(|_, _, aggs, admin| {
            use sda_protocol as p;
            let alice = new_agent();
            let alice_key = new_key_for_agent(&alice);
            assert_eq!(0, aggs.list_aggregations_by_title(&alice, "foo").unwrap().len());
            let agg = sda_protocol::Aggregation {
                id: sda_protocol::AggregationId::default(),
                title: "foo".into(),
                vector_dimension: 4,
                recipient: alice.id,
                recipient_key: alice_key.id,
                masking_scheme: p::LinearMaskingScheme::None,
                committee_sharing_scheme: p::LinearSecretSharingScheme::Additive {
                    share_count: 3,
                    modulus: 13
                },
                recipient_encryption_scheme: p::AdditiveEncryptionScheme::Sodium,
                committee_encryption_scheme: p::AdditiveEncryptionScheme::Sodium,
            };
            admin.create_aggregation(&alice, &agg).unwrap();
            assert_eq!(0, aggs.list_aggregations_by_title(&alice, "bar").unwrap().len());
            assert_eq!(1, aggs.list_aggregations_by_title(&alice, "foo").unwrap().len());
            assert_eq!(1, aggs.list_aggregations_by_title(&alice, "oo").unwrap().len());

            assert_eq!(0, aggs.list_aggregations_by_recipient(&alice, &new_agent().id).unwrap().len());
            assert_eq!(1, aggs.list_aggregations_by_recipient(&alice, &alice.id).unwrap().len());

            let agg2 = aggs.get_aggregation(&alice, &agg.id).unwrap();
            assert_eq!(Some(&agg), agg2.as_ref());

            admin.delete_aggregation(&alice, &agg.id).unwrap();
        });
    }
}
