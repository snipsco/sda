#!/bin/sh

set -e

# kill server from potential previous iteration
killall sdad || true

# discard data from previous iterations
rm -rf tmp/simple-data

# build then start server in background
(cd server-cli && cargo build)
(./server-cli/target/debug/sdad --jfs tmp/simple-data/server httpd &)

# build client
(cd cli && cargo build)

alias sda=./cli/target/debug/sda

# create recipient, plus three clerks, all with encryption keys
for i in recipient clerk-1 clerk-2 clerk-3
do
    sda -i tmp/simple-data/agent/$i agent create
    sda -i tmp/simple-data/agent/$i agent keys create
done

# create participants. they don't need encryption keys
for i in part-1 part-2 part-3
do
    sda -i tmp/simple-data/agent/$i agent create
done

alias recipient="./cli/target/debug/sda -i tmp/simple-data/agent/recipient"
AGGID=ad3142d8-9a83-4f40-a64a-a8c90b701bde
RECIPIENT_KEY_ID=$(grep -l '"ek"' tmp/simple-data/agent/recipient/keys/* | sed 's/.*\///;s/\.json//')

# create aggregation, and open it (creating clerk committee)
recipient aggregations create --id $AGGID "aggro" 10 433 $RECIPIENT_KEY_ID 3 
recipient aggregations begin $AGGID

# participants... participate
sda -i tmp/simple-data/agent/part-1 participate $AGGID 0 1 2 3 4 5 6 7 8 9
sda -i tmp/simple-data/agent/part-2 participate $AGGID 0 0 0 0 0 0 0 0 0 0
sda -i tmp/simple-data/agent/part-3 participate $AGGID 0 1 0 1 0 1 0 1 0 1

# close the aggregation
recipient aggregations end $AGGID

# have all potential clerks try and clerk
for i in recipient clerk-1 clerk-2 clerk-3
do
    sda -i tmp/simple-data/agent/$i clerk --once
done

# reconstruct the result
recipient aggregations reveal $AGGID
