# Runtime Upgrades and Versioning

This diagram shows our compile time and runtime dependencies to guide the upgrade process.

We have to follow Substrate/Cumulus/Polkadot version and relevant ORML closely.

Runtime WASM version of Picasso is bumped automatically. Compile time version of Composable to be defined in release process document.

```plantuml
@startuml

frame "GitHub" as github {
    folder "Substrate v0.9.*" as substrate
    folder "Cumulus v0.9.*" as cumulus
    folder "Polkadot v0.9.*" as polkadot
    folder "ORML v0.x.y" as orml 
    folder "Composable v.a.b" as composable 
}

cloud "Shared security (and versioning)" {
    node "Picasso Node tested with v0.9.23" as picasso {
        artifact "Upgradable runtime (WASM) Picasso tested with v0.9.24" as picasso_runtime {
            component "Runtime Configuration" as runtime_configuration {
                    component "Pallet A" as pallet_a
                    component "Pallet B" as pallet_b
            }
        }
    }

    node "Kusama Node v0.9.30" as kusama {
        artifact "Upgradable runtime (WASM) Kusama v0.9.30" as kusama_runtime
    } 
}

picasso -.-> kusama : Upgraded Picasso runtime
picasso -.-> kusama : XCM
picasso -.-> kusama : Parachain protocol messages
pallet_a -0)- runtime_configuration
pallet_b -0)- runtime_configuration
runtime_configuration -0)- picasso_runtime

cumulus --^ substrate
polkadot --^ cumulus
polkadot --^ substrate
orml --^ substrate
orml --^ cumulus
orml --^ polkadot
composable --^ substrate
composable --^ cumulus
composable --^ polkadot
composable --^ orml

picasso --:|> composable: Build from
kusama --:|> polkadot: Build from


@enduml
```
