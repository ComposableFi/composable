# Picasso Integration Tester

Picasso Integration Tester is a collection of different implementation tests for the Picasso Polkadot Parachain.

## Installation


```bash
$ npm ci
```

## Usage

### To run the devnet dummy data initializer
```bash
$ npm run init
```

### To run the tests:
```bash
$ npm run test
```


## ToDo:
* Add all general test cases.
* Add non-conventional test cases.
* Enhance code documentation.


## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.


### Notes for developers

On any tests waiting for a transaction result, you need to change the timeout setting.

Else the test will timeout before any results, causing a headache and wondering where the error lies. (Story fictitious)

e.g.
```typescript
describe('Imaginary Test', function () {
  this.timeout(0); // <--
  it('Imaginary test part', async (done) => {
      // Test Stuff...
  });
});
```


## License
Temporary:
[GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/)

Final License TBD.
