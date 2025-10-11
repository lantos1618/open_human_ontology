use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_neurological_data(db: &mut GroundTruthDatabase) {
    let mut neuro_data = GroundTruthData::new(
        "Neurological".to_string(),
        "Normal neurological parameters in healthy adults".to_string(),
    );

    neuro_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cerebrospinal_fluid_volume_ml".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(30.0),
        min_value: Some(100.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("21233481".to_string()),
            doi: Some("10.1148/radiol.10100410".to_string()),
            citation: "Edsbagge M et al. (2011) Radiology 259(1):218-225".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(156),
            population: "Healthy adults 20-70 years".to_string(),
        },
    });

    neuro_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_protein_mg_dl".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("23429095".to_string()),
            doi: Some("10.1212/WNL.0b013e318286c50c".to_string()),
            citation: "McCudden CR et al. (2013) Neurology 80(10):960-967".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5200),
            population: "Adults >18 years".to_string(),
        },
    });

    neuro_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_glucose_mg_dl".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(10.0),
        min_value: Some(45.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("23429095".to_string()),
            doi: Some("10.1212/WNL.0b013e318286c50c".to_string()),
            citation: "McCudden CR et al. (2013) Neurology 80(10):960-967".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5200),
            population: "Adults >18 years".to_string(),
        },
    });

    neuro_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "brain_volume_ml".to_string(),
        expected_value: 1350.0,
        standard_deviation: Some(120.0),
        min_value: Some(1100.0),
        max_value: Some(1600.0),
        reference: ClinicalReference {
            pmid: Some("29506344".to_string()),
            doi: Some("10.1016/j.neurobiolaging.2018.02.006".to_string()),
            citation: "Potvin O et al. (2018) Neurobiol Aging 66:163-172".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(46421),
            population: "Healthy adults 18-97 years".to_string(),
        },
    });

    neuro_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gray_matter_volume_ml".to_string(),
        expected_value: 680.0,
        standard_deviation: Some(60.0),
        min_value: Some(550.0),
        max_value: Some(800.0),
        reference: ClinicalReference {
            pmid: Some("29506344".to_string()),
            doi: Some("10.1016/j.neurobiolaging.2018.02.006".to_string()),
            citation: "Potvin O et al. (2018) Neurobiol Aging 66:163-172".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(46421),
            population: "Healthy adults 18-97 years".to_string(),
        },
    });

    neuro_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "white_matter_volume_ml".to_string(),
        expected_value: 490.0,
        standard_deviation: Some(50.0),
        min_value: Some(380.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("29506344".to_string()),
            doi: Some("10.1016/j.neurobiolaging.2018.02.006".to_string()),
            citation: "Potvin O et al. (2018) Neurobiol Aging 66:163-172".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(46421),
            population: "Healthy adults 18-97 years".to_string(),
        },
    });

    db.add_dataset("neurological".to_string(), neuro_data);
}
