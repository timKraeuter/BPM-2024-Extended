# Benchmark: Degrees of Parallelism

This folder contains the models with growing **parallel branches** used in the paper.

## Generating models

We provide the models used in the paper **and** the CLI-application to generate these models.

**Model generation help:**
```bash
java -jar BPMNModelGenerator-1.0-all.jar -h
```
TODO: use the long names here to make it more understandable
**Model generation:**
```bash
java -jar BPMNModelGenerator-1.0-all.jar -p "./models" -l 20 -b 20
```

The models are located in the folder `./models.`

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