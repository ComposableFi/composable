# About

Fast agile events, data changes and metrics tracking for on chain for monitoring and alerting.

## How to SRE/Product/QA

0. **What some Grafana company entry videos about Prometheus, Dashboards and Alerts if new to this area**
1. [The dashboard](https://composable.grafana.net/d/a89230b9-933b-45ce-a8aa-cde52bba81fd/xc)
2. [Alerts](https://composable.grafana.net/alerting/list)


## How to Developer

0. Run `nix develop` to get ability to compile and run whole observer stuff
1. Ensure `nix .#subxt-codegen-picasso` and `subxt-codegen-composable` for latest metadata
2. Run metrics collector via nix run `nix run .#grafana-observe`.
3. Run `cargo run` to start observations collection. 
4. Wait for couple minutes light clients warps to latest block snapshot.


## Architecture and design very succinctly  

Light (as light as possible client, warp) listens to blockchain as main source (sure can listen to other sources, including git assets registry).
Using local light clients increases reliability.

On changes detected (events, key subscription, logs), data gets into decoding.

Decoding makes raw strings and bytes into some well know data shapes(structures).

Then data sent to sinks. One is Prometheus sink (low cardinality time series metrics storage, read what is low cardinality Prometheus metrics on internet).
Sure can do any other sinks.

Sink shapes data for ingesting into sink. It may request data from sources to make enrich it source

All runs locally, sure can run BigQuery sink emulator via Nix.

Any sink can be source for other sinks too.

All is done async via channels (Go routines like).
So can make usage of any source by putting queue to make Go and Rust talk each other as needed.

**So look into queues as will grasp how it works**

Past is not important, no bother fix it or reindex, but that is not limitation. 

Final mile end users implementations (visualizations, analytics, alerts, dashboards, pager duty) outsourced to 3rd party components.
So can build own frontend if needed.

Correlation can be added on demand, preferable to use any storage which easy to access from Go and Rust and it can run local machine(fits what we use in cloud for long term index or what blockchains use(LevelDb?)).

We use string static typing (not string typing), hence data processing and subscription is done only to part of full data stream produced by chain.

Full reindexing and full data subscription is possible with tools we use, but that is not a goal of this solution (at least for now).