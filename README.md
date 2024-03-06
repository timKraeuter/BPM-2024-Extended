# BPM-2024

Artifacts for the paper _**Instantaneous, Understandable, and Actionable Soundness Checking of Industrial BPMN Models**_ submitted to BPM-2024.

Our tool is available [online](https://bpm-2024.whitefield-c9fed487.northeurope.azurecontainerapps.io/) and as an artifact (see the implementation section below).

## Instantaneous Soundness Checking

The models used in the paper are available in the `artifacts/instantaneous` directory:
1. [model-size](./artifacts/instantaneous/model-size) contains the BPMN models with increasing model size.
2. [parallel-branches](./artifacts/instantaneous/parallel-branches) contains the BPMN models with growing parallel branches and branch length.
3. [realistic-models](./artifacts/instantaneous/realistic-models) contains the realistic BPMN models.

In addition, each directory contains instructions to run the soundness checking benchmarks reported in the paper.

### Benchmark environment
- The benchmarks were run with hyperfine version **1.18.0** (binaries available for windows/linux in the **artifacts** folder).
- The benchmarks were run on Ubuntu 22.04.4 with an AMD Ryzen 7700X processor with 32 GB DDR5-5600 RAM on NVMe SSD storage.
- The benchmarks were run with our bpmn-analyzer tool version **0.2.0** (binaries available for windows/linux in the **artifacts** folder).

## Fixable Soundness Checking
The demo application showcasing the quick fixes is in [artifacts/tool-with-ui/](./artifacts/tool-with-ui/README.md).

## Implementation
The tool is available [online](https://bpm-2024.whitefield-c9fed487.northeurope.azurecontainerapps.io/).

In addition, the tool binaries are located in
1. [artifacts/](./artifacts/README.md) (for the CLI application)
2. [artifacts/tool-with-ui/](./artifacts/tool-with-ui/README.md) (tool including the UI as shown in the paper and provided above)

#### Screenshot during modeling:
![Screenshot 1 of the tool](./artifacts/images/Screenshot1.png)

#### Screenshot of counterexample visualization:
![Screenshot 2 of the tool](./artifacts/images/Screenshot2.png)

### Source code
We will make the source code available after review of the paper since the code is located in different repository and might compromise the double-blind review process.
If reviewers want access to the code before, we can anonymize the repositories on demand.

1. Analysis tool written in **Rust**: [TBD](https://github.com/)
2. Front-end implemented using the [bpmn-js](https://github.com/bpmn-io/bpmn-js) ecosystem, especially [bpmn-js-token-simulation](https://github.com/bpmn-io/bpmn-js-token-simulation): [TBD](https://github.com/)
3. CLI tools to generate the synthetic BPMN models: [TBD](https://github.com/)
