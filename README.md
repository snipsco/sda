## Structure

### Core
- [protocol](/protocol) documents the SDA service interface, as implemented by the server
- [client](/client) contains the core client code, published as `sda-client`, with a minimum of dependencies
- [server](/server) contains the core server code, published as `sda-server`, with a minumum of dependencies

### Implementation
- [httpproxy](/httpproxy) translates calls to the service interface into HTTP REST calls

### Wrappers
- [javaclient](/javaclient) wraps [client](/client) and [httpproxy](/httpproxy) and exposes the former to Java applications
