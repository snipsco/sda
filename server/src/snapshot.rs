use {SdaServer, SdaServerResult};
use sda_protocol::*;

use stores::AggregationsStore;

pub fn snapshot(server: &SdaServer, snapshot: &Snapshot) -> SdaServerResult<()> {
    server.aggregation_store.snapshot_participations(&snapshot.aggregation, &snapshot.id)?;
    let committee = server.get_committee(&snapshot.aggregation)?.ok_or("lost committe")?;
    let encryptions = server.aggregation_store
        .iter_snapshot_clerk_jobs_data(&snapshot.aggregation,
                                       &snapshot.id,
                                       committee.clerks_and_keys.len())?;
    let clerks_ids = committee.clerks_and_keys.into_iter().map(|c| c.0);

    for (clerk, shares) in clerks_ids.zip(encryptions) {
        server.clerking_job_store.enqueue_clerking_job(&ClerkingJob {
            id: ClerkingJobId::random(),
            clerk: clerk,
            aggregation: snapshot.aggregation.clone(),
            snapshot: snapshot.id.clone(),
            encryptions: shares?,
        })?;
    }
    server.aggregation_store.create_snapshot(&snapshot)?;

    Ok(())
}
