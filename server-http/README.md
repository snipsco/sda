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
