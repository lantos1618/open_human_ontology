# Alcohol Dehydrogenase 1B (ADH1B)

## Description
ADH1B is a key enzyme in alcohol metabolism that catalyzes the oxidation of ethanol to acetaldehyde. The ADH1B*2 variant, common in East Asian populations, shows significantly higher activity than the wild-type enzyme.

## Relationships
- `is_part_of`: [[ethanol_metabolism]] - Primary metabolic pathway
- `produces`: [[acetaldehyde]] - Direct product
- `interacts_with`: [[NAD_plus]] - Essential cofactor
- `modulates`: [[alcohol_flush_reaction]] - Affects symptom intensity
- `synergizes_with`: [[ALDH2]] - Combined genetic effect
- `located_in`: [[liver_cytosol]] - Primary location
- `type_of`: [[oxidoreductase]] - Enzyme classification

## Structure

```mermaid
graph TD
    subgraph Protein Structure
        A[Catalytic Domain] --> B[NAD+ Binding]
        B --> C[Substrate Pocket]
        C --> D[Zinc Binding Sites]
    end
    
    subgraph Variants
        E[ADH1B*1] --> F[Normal Activity]
        G[ADH1B*2] --> H[High Activity]
        I[ADH1B*3] --> J[Variable Activity]
    end
```

## Molecular Details

### 1. Protein Properties
- Length: 375 amino acids
- Molecular weight: 40 kDa
- Quaternary structure: Homodimer
- Cofactor: NAD+
- Metal ions: 2 Zinc atoms per subunit

### 2. Genetic Variants
```mermaid
graph TD
    A[ADH1B Gene] --> B[ADH1B*1]
    A --> C[ADH1B*2]
    A --> D[ADH1B*3]
    
    B -->|Arg47| E[Standard Activity]
    C -->|His47| F[40x Higher Activity]
    D -->|Cys369| G[Variable Activity]
```

## Catalytic Mechanism

```mermaid
sequenceDiagram
    participant E as Ethanol
    participant Z as Zinc
    participant N as NAD+
    participant A as Acetaldehyde
    
    E->>Z: Coordination
    N->>Z: Hydride Transfer
    Z->>A: Product Release
    Note over A: NADH + H+
```

### 1. Kinetic Parameters
- Km (ethanol):
  - ADH1B*1: ~1 mM
  - ADH1B*2: ~0.2 mM
- Vmax (relative):
  - ADH1B*1: 1x
  - ADH1B*2: ~40x

### 2. Substrate Specificity
- Primary: Ethanol
- Secondary:
  - Methanol
  - Longer chain alcohols
  - Retinol

## Population Genetics

### 1. Allele Distribution
```mermaid
graph LR
    subgraph Populations
    A[East Asian] -->|40-70%| B[ADH1B*2]
    C[European] -->|<5%| B
    D[African] -->|Rare| B
    end
```

### 2. Evolutionary Aspects
- Selection pressure
- Rice cultivation correlation
- Geographic distribution

## Clinical Significance

### 1. Alcohol Metabolism
- Rapid acetaldehyde production
- Enhanced alcohol sensitivity
- Reduced alcoholism risk

### 2. Health Implications
```mermaid
graph TD
    A[ADH1B*2] --> B[Rapid Ethanol Oxidation]
    B --> C[Acetaldehyde Accumulation]
    C --> D[Protective Effect]
    C --> E[Cancer Risk]
```

## Research Applications

### 1. Pharmacogenetics
- Drug metabolism
- Personalized medicine
- Treatment strategies

### 2. Population Studies
- Alcohol use disorders
- Cancer risk assessment
- Genetic screening

## Interactions

### 1. Enzyme Networks
- [[ALDH2]] pathway
- [[CYP2E1]] system
- [[catalase]] pathway

### 2. Regulatory Factors
- Hormonal regulation
- Transcriptional control
- Post-translational modifications

## References
1. Molecular Genetics and Evolution
2. Enzyme Structure and Function
3. Population Studies
4. Clinical Applications 