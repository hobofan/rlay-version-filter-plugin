#! /usr/bin/env bash
set -euxo pipefail

$(cd version-filter-plugin && cargo build --release && cp target/release/libversion_filter.dylib /Users/hobofan/rlay/rlay-client/plugins)

url_dp=$(cat ./ontology/build/seeded/main.json | jq -r '.urlDataProperty')
version_ap=$(cat ./ontology/build/seeded/main.json | jq -r '.versioningAnnotationProperty')

individual_cid=$(curl --data '{
  "method":"rlay_experimentalStoreEntity",
  "params":[{
    "type": "Individual"
  }],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq -r '.result')

version_1_cid=$(curl --data '{
  "method":"rlay_experimentalStoreEntity",
  "params":[{
    "type": "Annotation",
    "property": "'"$version_ap"'",
    "value": "0x01"
  }],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq -r '.result')
version_2_cid=$(curl --data '{
  "method":"rlay_experimentalStoreEntity",
  "params":[{
    "type": "Annotation",
    "property": "'"$version_ap"'",
    "value": "0x02"
  }],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq -r '.result')

# "0x76687474703a2f2f6578616d706c652e636f6d2f6f6c64" // old url
# "0x76687474703a2f2f6578616d706c652e636f6d2f6e6577" // new url

old_url_assertion=$(curl --data '{
  "method":"rlay_experimentalStoreEntity",
  "params":[{
    "type": "DataPropertyAssertion",
    "subject": "'"$individual_cid"'",
    "property": "'"$url_dp"'",
    "target": "0x76687474703a2f2f6578616d706c652e636f6d2f6f6c64",
    "annotations": ["'"$version_1_cid"'"]
  }],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq -r '.result')

new_url_assertion=$(curl --data '{
  "method":"rlay_experimentalStoreEntity",
  "params":[{
    "type": "DataPropertyAssertion",
    "subject": "'"$individual_cid"'",
    "property": "'"$url_dp"'",
    "target": "0x76687474703a2f2f6578616d706c652e636f6d2f6e6577",
    "annotations": ["'"$version_2_cid"'"]
  }],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq -r '.result')

curl --data '{
  "method":"rlay_experimentalResolveEntity",
  "params":["'"$individual_cid"'"],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq '.'

# Should return 2 (Individual + new assertion)
curl --data '{
  "method":"rlay_experimentalResolveEntity",
  "params":[
    "'"$individual_cid"'",
    {
      "filters": [{
        "filter": "version",
        "params": {
          "version_property": "'"$version_ap"'"
        }
      }]
    }
  ],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq '.'

# Should return 3
curl --data '{
  "method":"rlay_experimentalResolveEntity",
  "params":[
    "'"$individual_cid"'",
    {
      "filters": [{
        "filter": "version",
        "params": {
          "version_property": "0xab",
          "keep_unversioned": true
        }
      }]
    }
  ],
"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST 0.0.0.0:8546 --silent | jq '.'

