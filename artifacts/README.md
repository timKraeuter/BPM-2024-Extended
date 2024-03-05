# Running the tool
Run the tool from the command-line in this directory for a given BPMN file (**-f** option) and specific propertes (**-p** option).

## Windows
**Usage example:**
```bash
./bpmnanalyzer.exe -f instantaneous/parallel-branches/models/p03x10.bpmn -p safeness,option-to-complete,proper-completion,no-dead-activities
```
**Print options:**
```bash
./bpmnanalyzer.exe -h
```

## Linux

**Usage example:**
```bash
./bpmnanalyzer -f instantaneous/parallel-branches/models/p03x10.bpmn -p safeness,option-to-complete,proper-completion,no-dead-activities
```

**Print options:**
```bash
./bpmnanalyzer -h
```

To make the file executable on linux run the following command:

```bash
chmod +x bpmnanalyzer
```

Run the same for hyperfine if you want to run the benchmarks.
```bash
chmod +x hyperfine
```