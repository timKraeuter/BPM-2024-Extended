Run the tool for a given BPMN file (**-f** option) and specific propertes (**-p** option) using the following command:

**Windows**
```bash
./bpmnanalyzer.exe -f instantaneous/parallel-branches/models/p03x10.bpmn -p safeness,option-to-complete,proper-completion,no-dead-activities
```

**Linux**
```bash
./bpmnanalyzer -f instantaneous/parallel-branches/models/p03x10.bpmn -p safeness,option-to-complete,proper-completion,no-dead-activities
```
This binary of our tool is a CLI application build for the benchmarks described in the paper.

**Print help Windows**
```bash
./bpmnanalyzer.exe -h
```
**Print help Linux**
```bash
./bpmnanalyzer -h
```