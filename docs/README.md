# Secure Distributed Aggregator

### Core

- [protocol](protocol/sda_protocol/index.html) documents the SDA service interface, as implemented by the server and consumed by the client
- [client](client/sda_client/index.html) contains the core client code, published as `sda-client`, with a minimum of options and dependencies
- [server](server/sda_server/index.html) is the minimum server

### Server and network

- [server-http](server-http/sda_server_http/index.html) is the REST interface for the server
- [client-http](client-http/sda_client_http/index.html) is the matching REST proxy
- [server-store-mongodb](server-store-server-mongodb/sda_server_store_mongodb/index.html) is the MongoDB production storage for the server
