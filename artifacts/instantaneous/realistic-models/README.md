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

In addition, the [modelInfo.csv](./modelInfo.csv) file states the number of gateways, flow nodes, sequence flows, and flow elements for each model.

## Benchmark

**Windows:**
```bash
cd ../..
hyperfine.exe -L bpmnModel instantaneous/realistic-models/models/e001.bpmn,instantaneous/realistic-models/models/e002.bpmn,instantaneous/realistic-models/models/e020.bpmn,instantaneous/realistic-models/models/camunda-adopted/credit-scoring-synchronous.bpmn,instantaneous/realistic-models/models/camunda-adopted/credit-scoring-asynchronous.bpmn,instantaneous/realistic-models/models/camunda-adopted/dispatch-of-goods.bpmn,instantaneous/realistic-models/models/camunda-adopted/recourse.bpmn,instantaneous/realistic-models/models/camunda-adopted/self-service-restaurant.bpmn "bpmnanalyzer.exe -f {bpmnModel} -p safeness,option-to-complete,proper-completion,no-dead-activities" --output ./instantaneous/realistic-models/output.txt --export-json ./instantaneous/realistic-models/results.json
```

**Linux:**
```bash
cd ../..
hyperfine -L bpmnModel instantaneous/realistic-models/models/e001.bpmn,instantaneous/realistic-models/models/e002.bpmn,instantaneous/realistic-models/models/e020.bpmn,instantaneous/realistic-models/models/camunda-adopted/credit-scoring-synchronous.bpmn,instantaneous/realistic-models/models/camunda-adopted/credit-scoring-asynchronous.bpmn,instantaneous/realistic-models/models/camunda-adopted/dispatch-of-goods.bpmn,instantaneous/realistic-models/models/camunda-adopted/recourse.bpmn,instantaneous/realistic-models/models/camunda-adopted/self-service-restaurant.bpmn "bpmnanalyzer.exe -f {bpmnModel} -p safeness,option-to-complete,proper-completion,no-dead-activities" --output ./instantaneous/realistic-models/output.txt --export-json ./instantaneous/realistic-models/results.json
```

## Results

The benchmark results are found in the `results.json` file.
