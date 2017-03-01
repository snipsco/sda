# HTTP Server

Serving the SDA service over HTTP.


## Resource endpoints

### Agents
```
GET  /agents/<AgentId>
POST /agents/<AgentId>

GET  /agents/<AgentId>/profile
POST /agents/<AgentId>/profile

GET    /agents/<AgentId>/keys/<EncryptionKeyId>
POST   /agents/<AgentId>/keys/<EncryptionKeyId>
DELETE /agents/<AgentId>/keys/<EncryptionKeyId>
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

Committee
```
GET  /aggregations/<AggregationId>/committee/candidates
GET  /aggregations/<AggregationId>/committee/suggestions
POST /aggregations/<AggregationId>/committee
GET  /aggregations/<AggregationId>/committee
```

Participate
```
GET  /aggregations/<AggregationId>/participations/<ParticipationId>
POST /aggregations/<AggregationId>/participations/<ParticipationId>
```

Clerking
```
GET  /aggregations/-/jobs
GET  /aggregations/<AggregationId>/jobs/<ClerkingJobId>
POST /aggregations/<AggregationId>/jobs/<ClerkingJobId>/result
```

Get status and result
```
GET /aggregations/<AggregationId>/status
GET /aggregations/<AggregationId>/result
```
