extern crate sda_protocol;
extern crate sda_server;
extern crate sda_tests;
use sda_protocol::*;
use sda_tests::*;


#[test]
pub fn ping() {
    with_service(|ctx| {
        ctx.server.ping().unwrap();
        // ctx.service.ping().unwrap();
        // ctx.service.ping().unwrap();
    });
}

#[test]
pub fn agent_crud() {
    with_service(|ctx| {
        let alice = new_agent();
        ctx.server.create_agent(&alice, &alice).unwrap();
        let clone = ctx.server.get_agent(&alice, &alice.id).unwrap();
        assert_eq!(Some(&alice), clone.as_ref());

        let bob = ctx.server.get_agent(&alice, &sda_protocol::AgentId::default()).unwrap();
        assert!(bob.is_none());
    });
}

#[test]
pub fn profile_crud() {
    with_service(|ctx| {

        let alice = new_agent();

        ctx.server.create_agent(&alice, &alice).unwrap();
        let no_profile = ctx.server.get_profile(&alice, &alice.id).unwrap();
        assert!(no_profile.is_none());

        let alice_profile = sda_protocol::Profile {
            owner: alice.id,
            name: Some("alice".into()),
            ..sda_protocol::Profile::default()
        };
        ctx.server.upsert_profile(&alice, &alice_profile).unwrap();

        let clone = ctx.server.get_profile(&alice, &alice.id).unwrap();
        assert_eq!(Some(&alice_profile), clone.as_ref());

        let still_alice_profile = sda_protocol::Profile {
            owner: alice.id,
            name: Some("still alice".into()),
            ..sda_protocol::Profile::default()
        };
        ctx.server.upsert_profile(&alice, &still_alice_profile).unwrap();

        let clone = ctx.server.get_profile(&alice, &alice.id).unwrap();
        assert_eq!(Some(&still_alice_profile), clone.as_ref());
    });
}

#[test]
pub fn profile_acl() {
    with_service(|ctx| {
        let alice = new_agent();
        let bob = new_agent();
        ctx.server.create_agent(&bob, &bob).unwrap();

        let alice_fake_profile = sda_protocol::Profile {
            owner: alice.id,
            name: Some("bob".into()),
            ..sda_protocol::Profile::default()
        };

        let denied = ctx.server.upsert_profile(&bob, &alice_fake_profile);
        match denied {
            Err(sda_protocol::SdaError(sda_protocol::SdaErrorKind::PermissionDenied, _)) => {}
            e => panic!("unexpected result: {:?}", e),
        }
    });
}

#[test]
pub fn encryption_key_crud() {
    use sda_protocol::byte_arrays::*;
    with_service(|ctx| {
        let alice = new_agent();
        let bob = new_agent();
        ctx.server.create_agent(&alice, &alice).unwrap();
        ctx.server.create_agent(&bob, &bob).unwrap();

        let alice_key = sda_protocol::SignedEncryptionKey {
            body: sda_protocol::Labeled {
                id: sda_protocol::EncryptionKeyId::default(),
                body: sda_protocol::EncryptionKey::Sodium(B32::default()),
            },
            signer: alice.id,
            signature: sda_protocol::Signature::Sodium(B64::default()),
        };
        ctx.server.create_encryption_key(&alice, &alice_key).unwrap();
        let still_alice = ctx.server.get_encryption_key(&bob, &alice_key.body.id).unwrap();
        assert_eq!(Some(&alice_key), still_alice.as_ref());
    });
}

#[test]
pub fn auth_tokens_crud() {
    use sda_server::stores::AuthToken;
    with_service(|ctx| {
        let alice = new_agent();
        let alice_token = AuthToken {
            id: alice.id,
            body: "tok".into(),
        };
        assert!(ctx.server.0.check_auth_token(&alice_token).is_err());
        // TODO check error kind is InvalidCredentials
        ctx.server.create_agent(&alice, &alice).unwrap();
        ctx.server.0.upsert_auth_token(&alice_token).unwrap();
        assert!(ctx.server.0.check_auth_token(&alice_token).is_ok());
        let alice_token_new = AuthToken {
            id: alice.id,
            body: "token".into(),
        };
        assert!(ctx.server.0.check_auth_token(&alice_token_new).is_err());
        ctx.server.0.upsert_auth_token(&alice_token_new).unwrap();
        assert!(ctx.server.0.check_auth_token(&alice_token_new).is_ok());
        assert!(ctx.server.0.check_auth_token(&alice_token).is_err());
        ctx.server.0.delete_auth_token(&alice_token.id).unwrap();
        assert!(ctx.server.0.check_auth_token(&alice_token_new).is_err());
        assert!(ctx.server.0.check_auth_token(&alice_token).is_err());
    });
}

#[test]
pub fn aggregation_crud() {
    with_service(|ctx| {
        use sda_protocol as p;
        let (alice, alice_key) = new_full_agent(&ctx.service);
        assert_eq!(0,
                   ctx.service.list_aggregations(&alice, None, None).unwrap().len());
        let agg = sda_protocol::Aggregation {
            id: sda_protocol::AggregationId::default(),
            title: "foo".into(),
            vector_dimension: 4,
            recipient: alice.id,
            recipient_key: alice_key.id,
            masking_scheme: p::LinearMaskingScheme::None,
            committee_sharing_scheme: p::LinearSecretSharingScheme::Additive {
                share_count: 3,
                modulus: 13,
            },
            recipient_encryption_scheme: p::AdditiveEncryptionScheme::Sodium,
            committee_encryption_scheme: p::AdditiveEncryptionScheme::Sodium,
        };
        ctx.service.create_aggregation(&alice, &agg).unwrap();
        assert_eq!(0,
                   ctx.service.list_aggregations(&alice, Some("bar"), None).unwrap().len());
        assert_eq!(1,
                   ctx.service.list_aggregations(&alice, Some("foo"), None).unwrap().len());
        assert_eq!(1,
                   ctx.service.list_aggregations(&alice, Some("oo"), None).unwrap().len());

        assert_eq!(0,
                   ctx.service
                       .list_aggregations(&alice, None, Some(&new_agent().id))
                       .unwrap()
                       .len());
        assert_eq!(1,
                   ctx.service
                       .list_aggregations(&alice, None, Some(&alice.id))
                       .unwrap()
                       .len());

        let agg2 = ctx.service.get_aggregation(&alice, &agg.id).unwrap();
        assert_eq!(Some(&agg), agg2.as_ref());

        ctx.service.delete_aggregation(&alice, &agg.id).unwrap();
    });
}
