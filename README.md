# BPM-2024-Artifacts

Artifacts for the paper _**Instantaneous, Understandable, and Actionable Soundness Checking of Industrial BPMN Models**_ submitted to BPM-2024.

## Instantaneous Soundness Checking

The models used in the paper are available in the `arttifacst/instantaneous` directory:
1. [model-size](./artifacts/instantaneous/model-size) contains the BPMN models with increasing model size.
2. [industrial-models](./artifacts/instantaneous/industrial-models) contains the industrial BPMN models.
3. [degrees-of-parallelism](./artifacts/instantaneous/degrees-of-parallelism) contains the BPMN models with increasing degree of parallelism.

In addition, each directory contains instructions to run the soundness checking benchmarks reported in the paper.

### Benchmark environment
- The benchmarks were run with hyperfine version **1.18.0**.
- The benchmarks were run on a Windows 11 machine with an AMD Ryzen 7700X processor with 32 GB DDR5-5600 RAM on NVMe SSD storage.
