# Benchmark: Realistic Models

This folder describes the benchmarks of the realistic models.


## Contained models

1. Camunda BPMN for research models
   - We used the Camunda BPMN for research models available from [Camunda](https://github.com/camunda/bpmn-for-research).
   - We used the five provided solution models and slightly adopted them to be executable standalone.
   - The original solution models can be found in the `camunda-bpmn-for-research-original` directory.
   - The adopted models can be found in the `camunda-adopted` directory.
2. Models from houhou et al. (2022)
   - The models e001, e002, and e020 are provided in the paper ["A First-Order Logic verification framework for communication-parametric and time-aware BPMN collaborations"](https://www.doi.org/10.1016/j.is.2021.101765) by Houhou et al. (2022).

## Benchmark

**Windows:**
```bash
cd ../..
hyperfine.exe -V
```

**Linux:**
```bash
cd ../..
hyperfine -V
```

## Results

The benchmark results are found in the `results.json` file.
