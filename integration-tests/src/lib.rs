#![allow(dead_code)]
extern crate rouille;
extern crate sda_protocol;
extern crate sda_server;
#[cfg(feature="http")]
extern crate sda_client_http;
#[cfg(feature="http")]
extern crate sda_client_store;
#[cfg(feature="http")]
extern crate sda_server_http;
#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_term;
extern crate tempdir;

mod crud;
mod aggregation;

use std::{path, sync};
use std::sync::Arc;

use sda_server::SdaServer;
use sda_protocol::*;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

#[allow(dead_code)]
static GLOBAL_PORT_OFFSET: AtomicUsize = ATOMIC_USIZE_INIT;
static LOGS: sync::Once = sync::ONCE_INIT;

fn ensure_logs() {
    use slog::DrainExt;
    LOGS.call_once(|| {
        let root = ::slog::Logger::root(::slog_term::streamer()
                                            .stderr()
                                            .use_utc_timestamp()
                                            .build()
                                            .fuse(),
                                        o!());
        ::slog_scope::set_global_logger(root);
    });
}

fn jfs_server(dir: &path::Path) -> Arc<SdaServer> {
    let agents = sda_server::jfs_stores::JfsAgentStore::new(dir.join("agents")).unwrap();
    let auth = sda_server::jfs_stores::JfsAuthStore::new(dir.join("auths")).unwrap();
    let agg = sda_server::jfs_stores::JfsAggregationsStore::new(dir.join("service")).unwrap();
    Arc::new(SdaServer {
        agent_store: Box::new(agents),
        auth_token_store: Box::new(auth),
        aggregation_store: Box::new(agg),
    })
}

fn new_agent() -> Agent {
    Agent {
        id: AgentId::default(),
        verification_key: Labeled {
            id: VerificationKeyId::default(),
            body:
                VerificationKey::Sodium(byte_arrays::B32::default()),
        },
    }
}

fn new_key_for_agent(alice: &Agent) -> SignedEncryptionKey {
    use byte_arrays::*;
    SignedEncryptionKey {
        body: Labeled {
            id: EncryptionKeyId::default(),
            body: EncryptionKey::Sodium(B32::default()),
        },
        signer: alice.id,
        signature: Signature::Sodium(B64::default()),
    }
}

fn new_full_agent(agents: &SdaAgentService) -> (Agent, SignedEncryptionKey) {
    let ag = new_agent();
    agents.create_agent(&ag, &ag).unwrap();
    let key = new_key_for_agent(&ag);
    agents.create_encryption_key(&ag, &key).unwrap();
    (ag, key)
}


struct TextContext<'a> {
    server: Arc<SdaServer>,
    agents: &'a SdaAgentService,
    aggregation: &'a SdaAggregationService,
    admin: &'a SdaAdministrationService,
}

fn with_server<F>(f: F)
    where F: Fn(&TextContext) -> ()
{
    let tempdir = ::tempdir::TempDir::new("sda-tests-servers").unwrap();
    let server = jfs_server(tempdir.path());
//    println!("tempdir: {:?}", tempdir.into_path());
    let services = server.clone();
    let tc = TextContext {
        server: server,
        agents: &*services,
        aggregation: &*services,
        admin: &*services,
    };
    f(&tc)
}

#[cfg(feature="http")]
fn with_service<F>(f: F)
    where F: Fn(&TextContext) -> ()
{
    use std::sync::atomic::Ordering;
    ensure_logs();
    with_server(|ctx| {
        let running = Arc::new(sync::atomic::AtomicBool::new(true));
        let port_offset = GLOBAL_PORT_OFFSET.fetch_add(1, Ordering::SeqCst);
        let port = port_offset + 21000;
        let address = format!("127.0.0.1:{}", port);
        let server_for_thread = ctx.server.clone();
        let http_address = format!("http://{}/", address);
        let address_for_thread = address.clone();
        let running_for_thread = running.clone();
        let thread = ::std::thread::spawn(move || {
            let rouille_server = ::rouille::Server::new(&*address_for_thread, move |req| {
                    ::sda_server_http::handle(&*server_for_thread, req)
                })
                .unwrap();
            while running_for_thread.load(Ordering::SeqCst) {
                rouille_server.poll();
                ::std::thread::sleep(::std::time::Duration::new(0, 1000000));
            }
        });
        let tempdir = ::tempdir::TempDir::new("sda-tests-clients").unwrap();
        let store = ::sda_client_store::Filebased::new(&tempdir).unwrap();
        let services = ::sda_client_http::SdaHttpClient::new(&*http_address,store).unwrap();
        let tc = TextContext {
            server: ctx.server.clone(),
            agents: &services,
            aggregation: &services,
            admin: &services,
        };
        f(&tc);
        running.store(false, Ordering::SeqCst);
        thread.join().unwrap();
    });
}

#[cfg(not(feature="http"))]
fn with_service<F>(f: F)
    where F: Fn(&TextContext) -> ()
{
    ensure_logs();
    with_server(f)
}
