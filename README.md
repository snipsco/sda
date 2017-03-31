# Overview

## Purpose

SDA is a framework used at Snips for Secure Distributed Aggregation. It
implements a simple and efficient [multi-party computation protocol](https://en.wikipedia.org/wiki/Secure_multi-party_computation) for
computing aggregations (sums for now) of data from several participants while
keeping all inputs private.

In exchange for computing only a certain class of functions, the system has been
optimised to run on relatively weak and sporadic devices such as mobile phones.
In particular, one aim of SDA is to combine locally trained machine learning
models from mobile phones into a global model, and doing so privately.

## Walkthrough

A runnable version of this scenario [docs/simple-cli-example.sh](can be found
here).

SDA relies on interaction between several _agents_ helped by a single _server_.

### Server

First we can build and start the server:

```
cd server-cli
cargo build
cargo run -- --jfs tmp/simple-data/server httpd
```

This starts a SDA server that will listen for localhost on port 8888, and use
json files in the directory to store its state. `--help` will show options
to move data around or change the listening socket. For production setup
a MongoDB alternative storage is offered.

### Agents

Next we need a _recipient_. This is the person or organisation that is setting
up the aggregation specification and who will receive the final aggregated
result.

As well as the recipient, we will create three possible _clerks_: these three
agents, by providing the server with a public encryption key, become candidates
to take part in distributed computations. Private data is never at risk of being
exposed to any clerk, and the protocol has furthermore been designed to minimise
their work load. 

For this walkthrough we will use the [command line client](/cli) to show the
interactions of agents, but there is also a [client library](/client) for
incorporating SDA in other applications. In the following, the -i allows us to
specify different identity storages to simulate several agents on the same
computer.

```
(cd cli && cargo build)
alias sda=./cli/target/debug/sda

for i in recipient clerk-1 clerk-2 clerk-3
do
    sda -i tmp/simple-data/agent/$i agent create
    sda -i tmp/simple-data/agent/$i agent keys create
done
```

We will also create three _participants_ in the process: they will offer the
data to be aggregated, without making it public to the server or any other
agent in the operation.

```
for i in part-1 part-2 part-3
do
    sda -i tmp/simple-data/agent/$i agent create
done
```

### Aggregation

Now that we have enough clerks to share the secrets and spread the trust, the
recipient can create the aggregation (having created the participants do not
matter).

```
AGGID=ad3142d8-9a83-4f40-a64a-a8c90b701bde
RECIPIENT_KEY_ID=$(grep -l '"ek"' tmp/simple-data/agent/recipient/keys/* | sed 's/.*\///;s/\.json//')
sda -i tmp/simple-data/agent/recipient aggregations create --id $AGGID "aggro" 10 433 $RECIPIENT_KEY_ID 3
sda -i tmp/simple-data/agent/recipient aggregations begin $AGGID
```

We need a bit of shell plumbing to grab the recipient key for now, but the
gist of these command is to create an aggregation (with a provided id), and a
name. It will aggregate participations of 10 numbers, taken from a 
(semi-exclusive) 0..433 interval. We also specify the key we want the final 
result to be encrypted under and the number of ways (3) we want to split the
participants secrets.

The "begin" command will actually pick a committee of 3 clerks (among the
clerks and recipient) and thus "open" the aggregation for participation.

### Participation

At this point each participant can send its contribution to be aggregated.

```
sda -i tmp/simple-data/agent/part-1 participate $AGGID 0 1 2 3 4 5 6 7 8 9
sda -i tmp/simple-data/agent/part-2 participate $AGGID 0 0 0 0 0 0 0 0 0 0
sda -i tmp/simple-data/agent/part-3 participate $AGGID 0 1 0 1 0 1 0 1 0 1
```

These secret inputs are split between the elected clerks, each part encrypted
with the corresponding key, and sent to the server. Splitting the participations
are done using [secret sharing](https://en.wikipedia.org/wiki/Secret_sharing) so
that no group of agents below a specified _privacy threshold_ can recover the
inputs from the shares. Likewise, any outsider such as the server cannot recover
the inputs since to do so one would need to decrypt the shares using secret keys
known only by the clerks.

### Clerking

When the recipient determines that enough participations have been made, the
aggregation can be closed to move it to the next stage:

```
sda -i tmp/simple-data/agent/recipient aggregations end $AGGID
```

The server will organize the data in "jobs" that the three clerk from the
committee will have to eventually work upon.

```
for i in recipient clerk-1 clerk-2 clerk-3
do
    sda -i tmp/simple-data/agent/$i clerk --once
done
```

Without the `--once` parameter, the command would behave as a long running
process that checks the server queue periodically and perform whatever task the
server has in store.

Here it will just check once. Three out of the four potential clerks have actual
clerking jobs, the remaining one none.

Each clerk actually aggregates its part of the multi-party computation protocol,
and then sends back its share of the result to the server, encrypted under the
recipient's key.

### Final reveal

The recipient can reconstruct the final aggregated output from the results of
the clerks:

```
sda -i tmp/simple-data/agent/recipient aggregations reveal $AGGID
```

In our case, it should read: `0 2 2 4 4 6 6 8 8 10`.

## Doing more, with APIs

The command line client only expose a subset of the API, at least for now. It is
not meant to be the primary mode of interacting with an SDA service. The Rust
API to the client, or the REST API to the server allow more flexibility:

* tweaking the sharing scheme: the Packed Shamir Scheme provides resilience
    over clerks failure, as well as reducing message size.
* using a masking scheme to protect participants privacy against a collusion
    between clerks
* using the [Paillier cryptosystem](https://en.wikipedia.org/wiki/Paillier_cryptosystem) 
  to scale up the system to any number of participants
* allowing candidate clerks to link their profile to some external
    authenticating system to improve participants trust in the system
* allow recipient to actually chose the clerks that should get in the committee
    for its aggregation

## Structure

### Core
- [protocol](/protocol) documents the SDA service interface, as implemented by the server and consumed by the client
- [client](/client) contains the core client code, published as `sda-client`, with a minimum of options and dependencies
- [server](/server) is the minimum server

### Server and network

- [server-http](/server-http) is the REST interface for the server
- [client-http](/client-http) is the matching REST proxy
- [server-store-mongodb](/server-store-server) is the MongoDB production storage for the server

Various combination for these crates can be tested with [integration-tests](/integration-tests).

# Command line interface

- [cli](/cli) is the agent command line interface. The executable name is `sda`.
- [server-cli](/server-cli) binds all of the server pieces together in a command line interface called `sdad`.

### Wrappers

These wrappers are meant to be used in application (typically mobile or
embedded apps). They have not been released yet, they need a bit of cleanup
but will come soon.

- /embeddable-client wraps [client](/client) and [client-http](/client-http) to exposes the client functionality in a C-friendly
- /javaclient builds on top of it a semantic interface for Java application (including Android)
- /swiftclient does the same for Swift, targetting macOS and iOS application integration.

# License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
 


