use {SdaServer, SdaServerResult};
use sda_protocol::*;

use stores::AggregationsStore;

pub fn snapshot(server: &SdaServer, snapshot: &Snapshot) -> SdaServerResult<()> {
    let aggregation = server.aggregation_store.get_aggregation(&snapshot.aggregation)?.ok_or("lost aggregation")?;
    server.aggregation_store.snapshot_participations(&snapshot.aggregation, &snapshot.id)?;
    let committee = server.get_committee(&snapshot.aggregation)?.ok_or("lost committee")?;
    let encryptions = server.aggregation_store
        .iter_snapshot_clerk_jobs_data(&snapshot.aggregation,
                                       &snapshot.id,
                                       committee.clerks_and_keys.len())?;
    let clerks_ids = committee.clerks_and_keys.into_iter().map(|c| c.0);

    for (clerk, shares) in clerks_ids.zip(encryptions) {
        server.clerking_job_store
            .enqueue_clerking_job(&ClerkingJob {
                id: ClerkingJobId::random(),
                clerk: clerk,
                aggregation: snapshot.aggregation.clone(),
                snapshot: snapshot.id.clone(),
                encryptions: shares?,
            })?;
    }

    if aggregation.masking_scheme.has_mask() {
        let recipient_encryptions:Vec<Encryption> = server.aggregation_store
            .iter_snapped_participations(&snapshot.aggregation, &snapshot.id)?
            .map(|part| -> SdaServerResult<Encryption> {
                Ok(part?.recipient_encryption
                    .ok_or("participation should have had a recipient encryption")?)
            }).collect::<SdaServerResult<Vec<Encryption>>>()?;
        server.aggregation_store.create_snapshot_mask(&snapshot.id, recipient_encryptions)?;
    }

    server.aggregation_store.create_snapshot(&snapshot)?;

    Ok(())
}
