use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use sda_protocol::*;

/// Basic tasks needed by a recipient.
pub trait Receive {

    // fn new_aggregation(&self,
    //                    title: &str,
    //                    vector_dimension: usize,
    //                    recipient_key: &EncryptionKeyId,
    //                    masking_scheme: &LinearMaskingScheme,
    //                    committee_sharing_scheme: &LinearSecretSharingScheme,
    //                    recipient_encryption_scheme: &AdditiveEncryptionScheme,
    //                    committee_encryption_scheme: &AdditiveEncryptionScheme)
    //                    -> SdaClientResult<Aggregation>;

    fn upload_aggregation(&self, aggregation: &Aggregation) -> SdaClientResult<()>;

}

impl Receive for SdaClient {

    // fn new_aggregation(&self,
    //                    title: &str,
    //                    vector_dimension: usize,
    //                    recipient_key: &EncryptionKeyId,
    //                    masking_scheme: &LinearMaskingScheme,
    //                    committee_sharing_scheme: &LinearSecretSharingScheme,
    //                    recipient_encryption_scheme: &AdditiveEncryptionScheme,
    //                    committee_encryption_scheme: &AdditiveEncryptionScheme)
    //                    -> SdaClientResult<Aggregation>
    // {
    //     // ensure key matches with scheme
    //     if !recipient_key.suitable_for(recipient_encryption_scheme) {
    //         Err("Encryption key unsuitable")?
    //     }
    //
    //     Ok(Aggregation {
    //         id: AggregationId::random(),
    //         title: title.to_string(),
    //         vector_dimension: vector_dimension,
    //         recipient: self.agent.id().clone(),
    //         recipient_key: recipient_key.clone(),
    //         masking_scheme: masking_scheme.clone(),
    //         committee_sharing_scheme: committee_sharing_scheme.clone(),
    //         recipient_encryption_scheme: recipient_encryption_scheme.clone(),
    //         committee_encryption_scheme: committee_encryption_scheme.clone(),
    //     })
    // }

    fn upload_aggregation(&self, aggregation: &Aggregation) -> SdaClientResult<()> {
        Ok(self.service.create_aggregation(&self.agent, aggregation)?)
    }

}
