# Heart

## Description
The heart is a muscular organ that serves as the central pump of the [[systems/cardiovascular|cardiovascular system]]. It beats approximately 100,000 times per day, pumping blood throughout the body.

## Anatomical Structure

```mermaid
graph TD
    H[Heart] --> RA[Right Atrium]
    H --> LA[Left Atrium]
    H --> RV[Right Ventricle]
    H --> LV[Left Ventricle]
    
    subgraph Valves
    RA --> TV[Tricuspid Valve]
    LA --> MV[Mitral Valve]
    RV --> PV[Pulmonary Valve]
    LV --> AV[Aortic Valve]
    end
    
    subgraph Blood Vessels
    RA --> SVC[Superior Vena Cava]
    RA --> IVC[Inferior Vena Cava]
    LA --> PVeins[Pulmonary Veins]
    RV --> PA[Pulmonary Artery]
    LV --> Ao[Aorta]
    end
```

## Layers
1. **Pericardium** - Outer protective sac
2. **Epicardium** - Outer heart layer
3. **Myocardium** - Heart muscle
4. **Endocardium** - Inner lining

## Electrical Conduction System

```mermaid
graph TD
    SA[Sinoatrial Node] -->|Impulse| AV[Atrioventricular Node]
    AV --> HB[Bundle of His]
    HB --> RBB[Right Bundle Branch]
    HB --> LBB[Left Bundle Branch]
    RBB --> PF1[Purkinje Fibers]
    LBB --> PF2[Purkinje Fibers]
```

## Blood Supply
- Right coronary artery
- Left coronary artery
  - Left anterior descending
  - Circumflex artery

## Related Processes
- [[processes/physiological/cardiac_cycle]]
- [[processes/physiological/blood_pressure]]
- [[processes/cellular/cardiac_muscle_contraction]]

## Clinical Significance
- Common pathologies
- Diagnostic procedures
- Treatment approaches

## References
1. Netter's Atlas of Human Anatomy
2. Clinical Anatomy by Regions
3. Cardiovascular Physiology 