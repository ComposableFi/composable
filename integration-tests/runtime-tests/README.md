# Picasso Integration Tester

Picasso Integration Tester is a collection of different implementation tests for the Picasso Polkadot Parachain.

## Installation

```bash
$ npm ci
```


## Usage

### To run the fully automated test suite on Docker:
```bash
$ docker-compose up
```

### To run the devnet dummy data initializer
```bash
$ npm run init
```

### To run the tests:
```bash
$ npm run test
```

### To run the type generator:
```bash
$ npm run gen
```

### If you want to check for dependency updates:
```bash
$ npm run check_dep_updates
```



## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.


### Notes for developers

On any tests waiting for a transaction result, you need to change the timeout setting.

Else the test will timeout before any results, causing a headache and wondering where the error lies. (Story fictitious)

e.g.
```typescript
describe('Imaginary Test', function () {
  // Timeout set to 2 minutes
  this.timeout(2*60*1000); // <--
  it('Imaginary test part', async function (done) {
    // Test Stuff...
  });
});
```
