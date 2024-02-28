# Benchmark: Degrees of Parallelism

This folder contains the models with growing **parallel branches** used in the paper.

## Generating/Unpacking models

We provide the models used in the paper **and** our CLI-application to generate these models.

**Model generation help:**
```bash
java -jar BPMNModelGenerator-1.0-all.jar -h
```
**Model generation:**
```bash
java -jar BPMNModelGenerator-1.0-all.jar -path="./models" -length-of-branches=20 -number-of-branches=20
```
Generates 400 models with 1-20 parallel branches of length 1-20 in the `./models` directory.
The source code for the CLI-application is available at [TBD after review](https://github.com/).

The precomputed models are located in the `models.zip` file and the 10 models used in the benchmark are located in the `models` directory.

## Benchmark

**Windows:**
```bash
cd ../..
hyperfine.exe -L bpmnModel instantaneous/parallel-branches/models/p05x01.bpmn,instantaneous/parallel-branches/models/p10x01.bpmn,instantaneous/parallel-branches/models/p15x01.bpmn,instantaneous/parallel-branches/models/p16x01.bpmn,instantaneous/parallel-branches/models/p17x01.bpmn,instantaneous/parallel-branches/models/p20x01.bpmn,instantaneous/parallel-branches/models/p05x03.bpmn,instantaneous/parallel-branches/models/p05x05.bpmn,instantaneous/parallel-branches/models/p03x10.bpmn,instantaneous/parallel-branches/models/p03x20.bpmn "bpmnanalyzer.exe -f {bpmnModel} -p safeness,option-to-complete,proper-completion,no-dead-activities" --output ./instantaneous/parallel-branches/output.txt --export-json ./instantaneous/parallel-branches/results.json
```
**Linux:**
```bash
cd ../..
hyperfine -L bpmnModel instantaneous/parallel-branches/models/p05x01.bpmn,instantaneous/parallel-branches/models/p10x01.bpmn,instantaneous/parallel-branches/models/p15x01.bpmn,instantaneous/parallel-branches/models/p16x01.bpmn,instantaneous/parallel-branches/models/p17x01.bpmn,instantaneous/parallel-branches/models/p20x01.bpmn,instantaneous/parallel-branches/models/p05x03.bpmn,instantaneous/parallel-branches/models/p05x05.bpmn,instantaneous/parallel-branches/models/p03x10.bpmn,instantaneous/parallel-branches/models/p03x20.bpmn "./bpmnanalyzer -f ./{bpmnModel} -p safeness,option-to-complete,proper-completion,no-dead-activities" --output ./instantaneous/parallel-branches/output.txt --export-json ./instantaneous/parallel-branches/results.json
```

We tested the following models **n x m** (**n**=number of parallel branches, **m**=length of parallel branches):
1. file `p05x01.bpmn`, 35 states (2^1 + 3)
2. file `p10x01.bpmn`, 1.027 states (2^10 + 3)
3. file `p15x01.bpmn`, 32.771 states (2^15 + 3)
4. file `p16x01.bpmn`, 65.539 states (2^16 + 3)
5. file `p17x01.bpmn`, 131.075 states (2^17 + 3)
6. file `p20x01.bpmn`, 1.048.579 states (2^20 + 3)
7. file `p05x03.bpmn`, 1.027 states (4^5 + 3)
8. file `p05x05.bpmn`, 7.779 states (6^5 + 3)
9. file `p03x10.bpmn`, 1.334 states (11^3 + 3)
10. file `p03x20.bpmn`, 9.264 states (21^3 + 3)

The number of states can be calculated using the following formula `(m+1)^n + 3`.

**Generate the plot**
```bash
py plot.py ./results.json
```

## Results

The benchmark results are found in the `results.json` file.