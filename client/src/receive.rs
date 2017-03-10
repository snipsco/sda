use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use sda_protocol::*;

pub struct RecipientOutput(pub Vec<i64>);

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

    /// Assign any committee to the aggregation if none already.
    fn open_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()>;

    fn close_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()>;

    fn reveal_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<RecipientOutput>;

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

    fn open_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()> {
        let candidates = self.service.suggest_committee(&self.agent, &aggregation)?;
        // select suitable committee, following service suggestion blindly
        let selected_clerks = candidates.iter()
            .map(|c| (c.id, c.keys[0]) )
            .collect();
        let committee = Committee {
            aggregation: aggregation.clone(),
            clerks_and_keys: selected_clerks,
        };
        Ok(self.service.create_committee(&self.agent, &committee)?)
    }

    fn close_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()> {
        let status = self.service.get_aggregation_status(&self.agent, aggregation)?
            .ok_or("Unknown aggregation")?;

        if status.snapshots.len() >= 1 {
            return Ok(());
        }

        // create new snapshot
        let snapshot = Snapshot {
            id: SnapshotId::random(),
            aggregation: aggregation.clone(),
        };
        Ok(self.service.create_snapshot(&self.agent, &snapshot)?)
    }

    fn reveal_aggregation(&self, aggregation_id: &AggregationId) -> SdaClientResult<RecipientOutput> {

        // we'll need this guy later
        let aggregation = self.service.get_aggregation(&self.agent, aggregation_id)?
            .ok_or("Aggregation missing")?;

        // take first ready snapshot
        let status = self.service.get_aggregation_status(&self.agent, aggregation_id)?
            .ok_or("Unknown aggregation")?;
        let snapshot = status.snapshots.iter()
            .filter(|snapshot| snapshot.result_ready)
            .nth(1)
            .ok_or("Aggregation not ready")?;

        let result = self.service.get_snapshot_result(&self.agent, aggregation_id, &snapshot.id)?
            .ok_or("Missing aggregation result")?;

        let encrypted_masks = result.recipient_encryptions;
        let encrypted_masked_output = result.clerk_encryptions;

        // decrypt masks if needed
        let mask = match encrypted_masks {
            None => None,
            Some(encrypted_masks) => {
                let mask_decryptor = self.crypto.new_share_decryptor(
                    &aggregation.recipient_key,
                    &aggregation.recipient_encryption_scheme)?;

                let decrypted_masks = encrypted_masks.iter()
                    .map(|encryption| Ok(mask_decryptor.decrypt(encryption)?))
                    .collect::<SdaClientResult<Vec<Vec<Mask>>>>()?;

                let mask_combiner = self.crypto.new_mask_combiner(
                    &aggregation.masking_scheme)?;

                let mask = mask_combiner.combine(&decrypted_masks);
                Some(mask)
            }
        };

        // let masked_output = 

        Ok(RecipientOutput(vec![]))
    }

}
