# HTTP Server

Serving the SDA service over HTTP.


## Resource endpoints

### Agents
```
GET  /agents/<AgentId>
POST /agents/me

GET  /agents/<AgentId>/profile
POST /agents/me/profile

GET    /agents/any/keys/<EncryptionKeyId>
POST   /agents/me/keys
DELETE /agents/me/keys/<EncryptionKeyId>
```

### Aggregations

Search
```
GET /aggregations?title=filter&recipient=agentid 
```

Basic aggregation object
```
GET    /aggregations/<AggregationId>
POST   /aggregations
DELETE /aggregations/<AggregationId>
```

Committee
```
GET  /aggregations/<AggregationId>/committee/suggestions
POST /aggregations/implied/committee
GET  /aggregations/<AggregationId>/committee
```

Participate
```
POST /aggregations/participations
```

Snapshot participation
```
POST /aggregations/implied/snapshot
```

Clerking
```
GET  /aggregations/any/jobs
POST /aggregations/implied/jobs/<ClerkingJobId>/result
```

Get status and result
```
GET /aggregations/<AggregationId>/status
GET /aggregations/<AggregationId>/snapshots/<SnapshotId>/result
```
