#![allow(dead_code)]
extern crate rouille;
extern crate sda_protocol;
extern crate sda_server;
extern crate sda_client;
extern crate sda_client_store;
#[cfg(feature="http")]
extern crate sda_client_http;
#[cfg(feature="http")]
extern crate sda_server_http;
#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_term;
extern crate tempdir;

use std::sync;
use std::sync::Arc;

use sda_server::SdaServerService;
use sda_protocol::*;
use sda_client::*;
use sda_client_store::*;

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

pub fn new_agent() -> Agent {
    Agent {
        id: AgentId::default(),
        verification_key: Labeled {
            id: VerificationKeyId::default(),
            body:
                VerificationKey::Sodium(byte_arrays::B32::default()),
        },
    }
}

pub fn new_key_for_agent(alice: &Agent) -> SignedEncryptionKey {
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

pub fn new_full_agent(agents: &Arc<SdaService>) -> (Agent, SignedEncryptionKey) {
    let ag = new_agent();
    agents.create_agent(&ag, &ag).unwrap();
    let key = new_key_for_agent(&ag);
    agents.create_encryption_key(&ag, &key).unwrap();
    (ag, key)
}

pub fn new_client(agent: Agent, service: &Arc<SdaService>) -> SdaClient {
    let tempdir = ::tempdir::TempDir::new("sda-tests-clients-keystores").unwrap();
    let keystore = sda_client_store::Filebased::new(&tempdir.path()).unwrap();
    SdaClient::new(agent, Arc::new(keystore), service.clone())
}


pub struct TestContext {
    pub server: Arc<SdaServerService>,
    pub service: Arc<SdaService>,
}

pub fn with_server<F>(f: F)
    where F: Fn(&TestContext) -> ()
{
    let tempdir = ::tempdir::TempDir::new("sda-tests-servers").unwrap();
    let server: SdaServerService = sda_server::SdaServerService::new_jfs_server(tempdir.path()).unwrap();
    let s: Arc<SdaServerService> = Arc::new(server);
    let service: Arc<SdaService> = s.clone() as _;
//    println!("tempdir: {:?}", tempdir.into_path());
    let tc = TestContext {
        server: s,
        service: service,
    };
    f(&tc)
}

#[cfg(feature="http")]
pub fn with_service<F>(f: F)
    where F: Fn(&TestContext) -> ()
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
        let tc = TestContext {
            server: ctx.server.clone(),
            service: Arc::new(services),
        };
        f(&tc);
        running.store(false, Ordering::SeqCst);
        thread.join().unwrap();
    });
}

#[cfg(not(feature="http"))]
pub fn with_service<F>(f: F)
    where F: Fn(&TestContext) -> ()
{
    ensure_logs();
    with_server(f)
}
