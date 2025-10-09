# Relationship Types in OpenHuman

## Overview
This document defines the standard relationship types used to connect concepts in the OpenHuman documentation.

## Relationship Types

### Structural Relationships
- `is_part_of`: Component is physically part of another structure
- `contains`: Structure physically contains another component
- `connects_to`: Physical connection between structures
- `located_in`: Spatial relationship

### Functional Relationships
- `regulates`: Direct regulation or control
- `inhibits`: Negative regulation
- `activates`: Positive regulation or stimulation
- `interacts_with`: Direct molecular interaction
- `signals_to`: Signaling pathway connection

### Process Relationships
- `precedes`: Temporal sequence, happens before
- `follows`: Temporal sequence, happens after
- `triggers`: Initiates another process
- `depends_on`: Functional dependency
- `modulates`: Indirect regulation

### Clinical Relationships
- `causes`: Direct causation of condition
- `contributes_to`: Partial causation
- `indicates`: Diagnostic relationship
- `treats`: Therapeutic relationship

### Hierarchical Relationships
- `type_of`: Taxonomic relationship
- `subtype_of`: More specific classification
- `instance_of`: Specific example of concept

## Usage Example
```markdown
## Relationships
- `is_part_of`: [[heart]] - Component of the cardiovascular system
- `regulates`: [[cardiac_output]] - Controls blood flow
- `depends_on`: [[calcium_ions]] - Required for function
```

## Implementation
Each document should include a Relationships section using these standard types to create a semantic network of biological knowledge. 