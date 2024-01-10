# Overview

MANTIS Blackbox is algorithmic server doing some data mangling and number crunching to find routing solutions. 

Blackbox server depends on Simulation package. Simulation solve and verifies solution given existing data and heuristic constraints.

Here is software diagram of interacting components.


```mermaid
flowchart LR

    input((I want X for Y))
 subgraph Blackbox
    input -- get indexer data --> raw[raw data]
    raw -- unify data to CVM format --> data
    subgraph Simulation        
        data -- CS oracle --> od[oracalized data]
        od -- solve --> or[OR solver]
        od -- solve --> cs[CS solver]
        cs --> g[graph]
        g --> tg[traversed graph]
        tg --> route
        or -- scale in --> sd[scaled data]
        sd -- presolve --> ds[draft solution]
        ds -- solve --> fs[final solution]
        fs --> route
        route --> simulate
        simulate --> e{Evaluate}
        e --> |good solution or too many attempts| output
        e --> |bad| heuristic
        heuristic -- retry --> data
    end    
    output((output))
 end
```

Details of each step are outlined elsewhere in more low level places.