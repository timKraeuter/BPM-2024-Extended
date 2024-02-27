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

The precomputed models are located in the `models.zip` file.

## Benchmark

Unpack the `models.zip` file and run the following command:

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
1. 5x1
2. 10x1
3. 15x1
4. 16x1
5. 17x1
6. 20x1
7. 5x3
8. 5x5
9. 3x10
10. 3x20

## Results

The benchmark results are found in the `results.json` file.