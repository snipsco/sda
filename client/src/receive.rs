use SdaClient;
use crypto::*;
use errors::SdaClientResult;

use sda_protocol::*;

pub struct RecipientOutput {
    pub modulus: i64,
    pub values: Vec<i64>,
}

impl RecipientOutput {
    pub fn positive(&self) -> RecipientOutput {
        let positive_values = self.values.iter().map(|&v| if v < 0 { v + self.modulus } else { v }).collect();
        RecipientOutput {
            modulus: self.modulus,
            values: positive_values,
        }
    }
}

/// Basic tasks needed by a recipient.
pub trait Receive {

    fn upload_aggregation(&self, aggregation: &Aggregation) -> SdaClientResult<()>;

    /// Assign any committee to the aggregation if none already.
    fn begin_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()>;

    fn end_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()>;

    fn reveal_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<RecipientOutput>;

}

impl Receive for SdaClient {

    fn upload_aggregation(&self, aggregation: &Aggregation) -> SdaClientResult<()> {
        Ok(self.service.create_aggregation(&self.agent, aggregation)?)
    }

    fn begin_aggregation(&self, aggregation_id: &AggregationId) -> SdaClientResult<()> {
        let aggregation = self.service.get_aggregation(&self.agent, aggregation_id)?
            .ok_or(format!("Unknown aggregation, {:?}", aggregation_id))?;
        let candidates = self.service.suggest_committee(&self.agent, &aggregation_id)?;
        // select suitable committee, following service suggestion blindly
        let selected_clerks = candidates.iter()
            .take(aggregation.committee_sharing_scheme.output_size())
            .map(|candidate| (candidate.id, candidate.keys[0]) )
            .collect();
        let committee = Committee {
            aggregation: aggregation_id.clone(),
            clerks_and_keys: selected_clerks,
        };
        Ok(self.service.create_committee(&self.agent, &committee)?)
    }

    fn end_aggregation(&self, aggregation: &AggregationId) -> SdaClientResult<()> {
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

        // we'll need these guys later
        let aggregation = self.service.get_aggregation(&self.agent, aggregation_id)?
            .ok_or(format!("Unknown aggregation, {:?}", aggregation_id))?;
        let committee = self.service.get_committee(&self.agent, aggregation_id)?
            .ok_or(format!("Unknown committee, {:?}", aggregation_id))?;

        // take result of first ready snapshot
        let status = self.service.get_aggregation_status(&self.agent, aggregation_id)?
            .ok_or("Unknown aggregation")?;
        let snapshot = status.snapshots.iter()
            .filter(|snapshot| snapshot.result_ready)
            .nth(0)
            .ok_or("Aggregation not ready")?;
        let result = self.service.get_snapshot_result(&self.agent, aggregation_id, &snapshot.id)?
            .ok_or("Missing aggregation result")?;

        let encrypted_masks = result.recipient_encryptions;
        let encrypted_masked_output_shares = result.clerk_encryptions;

        // decrypt and combine masks
        let mask: Vec<Mask> = match encrypted_masks {
            None => vec![],
            Some(encrypted_masks) => {
                let mask_decryptor = self.crypto.new_share_decryptor(
                    &aggregation.recipient_key,
                    &aggregation.recipient_encryption_scheme)?;

                let decrypted_masks = encrypted_masks.iter()
                    .map(|encryption| Ok(mask_decryptor.decrypt(encryption)?))
                    .collect::<SdaClientResult<Vec<Vec<Mask>>>>()?;

                let mask_combiner = self.crypto.new_mask_combiner(
                    &aggregation.masking_scheme)?;

                mask_combiner.combine(&decrypted_masks)
            }
        };

        // decrypt and reconstruct shares
        let masked_output: Vec<MaskedSecret> = {

            let share_decryptor = self.crypto.new_share_decryptor(
                &aggregation.recipient_key,
                &aggregation.recipient_encryption_scheme)?;

            let masked_output_shares: Vec<(usize, Vec<Share>)> = encrypted_masked_output_shares.iter()
                .map(|clerking_result| {

                    // TODO we could avoid this scan if the server is guaranteed to result in right order
                    let clerk_index = committee.clerks_and_keys.iter()
                        .position(|&(id,_)| clerking_result.clerk == id)
                        .ok_or(format!("Missing clerk, {:?}", clerking_result.clerk))?;

                    let shares = share_decryptor.decrypt(&clerking_result.encryption)?;
                    Ok((clerk_index, shares))
                })
                .collect::<SdaClientResult<Vec<(usize, Vec<Share>)>>>()?;

            let secret_reconstructor = self.crypto.new_secret_reconstructor(
                &aggregation.committee_sharing_scheme)?;

            let masked_output = secret_reconstructor.reconstruct(&masked_output_shares);
            masked_output
        };

        // unmask secrets
        let secret_unmasker = self.crypto.new_secret_unmasker(
            &aggregation.masking_scheme)?;

        let output = secret_unmasker.unmask(&(mask, masked_output));
        Ok(RecipientOutput {
            modulus: aggregation.modulus,
            values: output,
        })
    }

}

impl SdaClient {

    // pub fn reveal_snapshot()

}
