# HTTP Server

Serving the SDA service over HTTP.


## Resource endpoints

### Agents
```
GET  /agents/<AgentId>
POST /agents/<AgentId>

GET  /agents/<AgentId>/profile
POST /agents/<AgentId>/profile

GET    /agents/<AgentId>/keys/<SignedEncryptionKeyId>
POST   /agents/<AgentId>/keys/<SignedEncryptionKeyId>
DELETE /agents/<AgentId>/keys/<SignedEncryptionKeyId>
```

### Aggregations

Search
```
GET /aggregations?name=filter&recipient=agentid
```

Basic aggregation object
```
GET    /aggregations/<AggregationId>
POST   /aggregations/<AggregationId>
DELETE /aggregations/<AggregationId>
```

Participate
```
POST /aggregations/<AggregationId>/participations/<ParticipationId>
GET  /aggregations/<AggregationId>/participations/<ParticipationId>
```

Clerking
```
GET  /aggregations/<AggregationId>/jobs
GET  /aggregations/<AggregationId>/jobs/<ClerkingJobId>
POST /aggregations/<AggregationId>/jobs/<ClerkingJobId>/result
```

Get status and result
```
GET /aggregations/<AggregationId>/status
GET /aggregations/<AggregationId>/result
```

### Committees
```
POST /committee/<CommitteeId>
GET  /committee/<CommitteeId>
```
