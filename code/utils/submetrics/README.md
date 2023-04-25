# Overview

1. subxt to parse types (already have and using)
2. smoldot(light client) to subscribe (no event loss, parse issues detector)
3. prometheus basic push, so dashboards and alerts is not concern of this script

So, it just loop `receive events -> decode -> map to prometheus -> push into prometheus -> log execution -> store successful block`.

For deploy `release -> generate subxt -> compile new loop -> trigger terraform (re) deploy (approve via terraform cloud)`
