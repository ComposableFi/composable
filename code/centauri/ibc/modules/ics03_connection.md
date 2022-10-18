## ICS03_CONNECTION

IBC connections are direct links between chains, there should only be one connection for between two specific chains, which translates to only one connection per light client.  

### Connection Context

The connection context encapsulates all the storage requirements for connections in the context object,  
implement the `ConnectionReader` and `ConnectionKeeper` for the context object

```text
    impl ConnectionReader for Context { ... }
    
    impl ConnectionKeeper for Context { ... }  
```

### Connection Messages 

There are four messages that describe the connection handshake process
