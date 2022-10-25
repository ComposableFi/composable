# Runtime Testing

Runtime testing involves running parachain code to ensure quality.
Unlike type system or static analysis (excluding basic linting).
Here is hierarchy of testing you may find in this parachain

| Name          | Runtime   | Warp | Start from real state | Serde |
| ------------- | --------- | ---- | --------------------- | ----- |
| Unit          | no        | yes  | no                    | no    |
| Runtime Unit  | mock      | yes  | yes                   |
| Property test | mock      | yes  |
| Benchmark     | mock/real | yes  |
| Visualization | no/mock   | yes  |
| Simulation    | real      | yes  | yes                   |
| Simnode       | real      | yes  | yes                   |
| Local Relay   | real      | no   |                       |
| Deployment    | real      | no   |                       |