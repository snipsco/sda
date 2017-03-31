## Purpose

SDA is Snips framework for Secure Distributed Aggregation. It implements a
simple and efficient Multi Player Computation scheme allowing to compute
aggregations (sums for now) of data from several participants while keeping
them private.

## Walkthrough

A runnable version of this scenario [docs/simple-cli-example.sh](can be found
here).

SDA relies on interaction between _agents_ helped by a _server_.

### Server

First we can build and start the server:

```
cd server-cli
cargo build
cargo run -- --jfs tmp/simple-data/server httpd
```

This start a SDA server that will listen for localhost on port 8888, and use
json files in the directory to store its state. `--help` will show options
to move data around or change the listening socket. For production setup
a MongoDB alternative storage is offered.

### Agents

Next we need a _recipient_. This is the person or organisation that is setting
up the aggregation specification and will receive the final aggregated result.

We will use the command line client here to show the interaction between
agents.

As well as recipient, we will create three possible _clerks_: these three
agents, by providing the server with a public encryption key, become
candidates to take part in distributed computations. The -i allows us to
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

We will also create three _participants_ in the process: they will offer their
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
result to be encrypted in and the number of ways (3) we want to split the
participants secrets.

The "begin" command will actually pick a committee of 3 clerks (among the
clerks and recipient) and thus "open" the aggregation for participation.

### Participation

At this point each participants can send its contribution to be aggregated.

```
sda -i tmp/simple-data/agent/part-1 participate $AGGID 0 1 2 3 4 5 6 7 8 9
sda -i tmp/simple-data/agent/part-2 participate $AGGID 0 0 0 0 0 0 0 0 0 0
sda -i tmp/simple-data/agent/part-3 participate $AGGID 0 1 0 1 0 1 0 1 0 1
```

The secret participations are splitted in three, each part encrypted with
one of the picked clerks and sent to the server. To read a participant vector,
one would need to decrypt the shares using keys secretly hold by three agents.

### Clerking

The recipient can close the participation to move it to the next stage:

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
process that checks the server queue periodically and perform whatever
task the server has in store.

Here it will just check once. Three out of the four potential clerks have
actual clerking jobs, the remaining one none.

Each clerk actually aggregates its part of the Multi Player Computation
process, and then send back its share of the result to the server,
encrypted with the recipient key.

### Final reveal

The recipient can reconstruct the final result from the clerks results:

```
sda -i tmp/simple-data/agent/recipient aggregations reveal $AGGID
```

In our case, it should read: `0 2 2 4 4 6 6 8 8 10`.

## Structure

### Core
- [protocol](/protocol) documents the SDA service interface, as implemented by the server and consumed by the client
- [client](/client) contains the core client code, published as `sda-client`, with a minimum of options and dependencies
- [server](/server) is the minimum server

### Server and network

- [server-http](/server-http) is the Rest interface for the server
- [client-http](/client-http) is the matching Rest proxy
- [server-store-mongodb](/server-store-server) is the MongoDB production storage for the server

Various combination for these crates can be tested with [integration-tests](/integration-tests).

# Command lines interface

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
 


