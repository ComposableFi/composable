A small server to serve coinmarketcap with an API of our token-data.

```
Usage: cmc-api <ADDRESS>

Arguments:
  <ADDRESS>  The address to bind the server to

Options:
  -h, --help  Print help information
```

It exposes the following endpoints:

```
/healthcheck
/total_supply
/circulating_supply
/explorer_url
/rich_list_url
```
    