//! # Vaccine Delivery Module
//! 
//! Models different vaccine delivery mechanisms and their properties.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::vaccines::{VaccineType, DeliverySystem};
use crate::biology::{BiologyError, BiologyResult};

/// Properties of delivery routes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRoute {
    /// Route type
    pub route: RouteType,
    /// Absorption rate (0-1)
    pub absorption_rate: f64,
    /// Local tissue response strength (0-1)
    pub local_response: f64,
    /// Systemic distribution efficiency (0-1)
    pub systemic_distribution: f64,
    /// Pain/discomfort level (0-1)
    pub discomfort: f64,
    /// Risk of complications (0-1)
    pub risk: f64,
}

/// Types of delivery routes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RouteType {
    Intramuscular {
        muscle: String,
        depth: f64, // in mm
    },
    Subcutaneous {
        site: String,
        depth: f64, // in mm
    },
    Intradermal {
        site: String,
        depth: f64, // in mm
    },
    Oral {
        formulation: OralFormulation,
        coating: Option<String>,
    },
    Nasal {
        device: String,
        particle_size: f64, // in micrometers
    },
    Transdermal {
        patch_type: String,
        area: f64, // in cm^2
    },
}

/// Oral vaccine formulations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OralFormulation {
    Tablet,
    Capsule,
    Liquid,
    Sublingual,
}

/// Delivery device properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryDevice {
    /// Device type
    pub device_type: DeviceType,
    /// Delivery volume (in mL)
    pub volume: f64,
    /// Flow rate (in mL/s)
    pub flow_rate: f64,
    /// Needle gauge (if applicable)
    pub needle_gauge: Option<u8>,
    /// Device accuracy (0-1)
    pub accuracy: f64,
}

/// Types of delivery devices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    Syringe {
        material: String,
        safety_features: Vec<String>,
    },
    Autoinjector {
        mechanism: String,
        safety_features: Vec<String>,
    },
    NasalSpray {
        mechanism: String,
        dose_counter: bool,
    },
    MicroinjectionDevice {
        precision: f64,
        automation_level: String,
    },
    PatchApplicator {
        adhesive_type: String,
        release_mechanism: String,
    },
}

impl DeliveryRoute {
    /// Create a new delivery route with default properties
    pub fn new(route: RouteType) -> Self {
        let (absorption_rate, local_response, systemic_distribution, discomfort, risk) = 
            match &route {
                RouteType::Intramuscular { .. } => (0.9, 0.7, 0.8, 0.6, 0.3),
                RouteType::Subcutaneous { .. } => (0.8, 0.6, 0.7, 0.4, 0.2),
                RouteType::Intradermal { .. } => (0.7, 0.8, 0.6, 0.5, 0.3),
                RouteType::Oral { .. } => (0.4, 0.3, 0.5, 0.1, 0.1),
                RouteType::Nasal { .. } => (0.6, 0.7, 0.5, 0.2, 0.2),
                RouteType::Transdermal { .. } => (0.5, 0.4, 0.6, 0.2, 0.1),
            };

        DeliveryRoute {
            route,
            absorption_rate,
            local_response,
            systemic_distribution,
            discomfort,
            risk,
        }
    }

    /// Calculate overall delivery efficiency
    pub fn calculate_efficiency(&self) -> f64 {
        (self.absorption_rate + self.systemic_distribution) / 2.0
    }

    /// Check if route is suitable for a given vaccine type
    pub fn is_suitable_for(&self, vaccine: &VaccineType) -> bool {
        match (vaccine, &self.route) {
            (VaccineType::MRNA { .. }, RouteType::Intramuscular { .. }) => true,
            (VaccineType::LiveAttenuated { .. }, RouteType::Subcutaneous { .. }) => true,
            (VaccineType::Inactivated { .. }, RouteType::Intramuscular { .. }) => true,
            _ => false,
        }
    }
}

impl DeliveryDevice {
    /// Create a new delivery device
    pub fn new(device_type: DeviceType, volume: f64) -> Self {
        let (flow_rate, needle_gauge, accuracy) = match &device_type {
            DeviceType::Syringe { .. } => (1.0, Some(23), 0.95),
            DeviceType::Autoinjector { .. } => (2.0, Some(25), 0.98),
            DeviceType::NasalSpray { .. } => (0.1, None, 0.90),
            DeviceType::MicroinjectionDevice { precision, .. } => (0.01, Some(30), *precision),
            DeviceType::PatchApplicator { .. } => (0.001, None, 0.99),
        };

        DeliveryDevice {
            device_type,
            volume,
            flow_rate,
            needle_gauge,
            accuracy,
        }
    }

    /// Calculate delivery precision
    pub fn calculate_precision(&self) -> f64 {
        self.accuracy * match &self.device_type {
            DeviceType::Autoinjector { .. } => 1.0,
            DeviceType::MicroinjectionDevice { precision, .. } => *precision,
            _ => 0.9,
        }
    }

    /// Check if device is suitable for route
    pub fn is_suitable_for_route(&self, route: &RouteType) -> bool {
        match (&self.device_type, route) {
            (DeviceType::Syringe { .. }, RouteType::Intramuscular { .. }) => true,
            (DeviceType::Syringe { .. }, RouteType::Subcutaneous { .. }) => true,
            (DeviceType::Autoinjector { .. }, RouteType::Intramuscular { .. }) => true,
            (DeviceType::NasalSpray { .. }, RouteType::Nasal { .. }) => true,
            (DeviceType::PatchApplicator { .. }, RouteType::Transdermal { .. }) => true,
            _ => false,
        }
    }
}

/// Delivery system optimizer
pub struct DeliveryOptimizer {
    routes: Vec<DeliveryRoute>,
    devices: Vec<DeliveryDevice>,
}

impl DeliveryOptimizer {
    /// Create a new delivery optimizer
    pub fn new() -> Self {
        DeliveryOptimizer {
            routes: Vec::new(),
            devices: Vec::new(),
        }
    }

    /// Add a delivery route
    pub fn add_route(&mut self, route: DeliveryRoute) {
        self.routes.push(route);
    }

    /// Add a delivery device
    pub fn add_device(&mut self, device: DeliveryDevice) {
        self.devices.push(device);
    }

    /// Find optimal delivery method for a vaccine
    pub fn optimize(&self, vaccine: &VaccineType) -> Option<(DeliveryRoute, DeliveryDevice)> {
        let mut best_combination = None;
        let mut best_score = 0.0;

        for route in &self.routes {
            if !route.is_suitable_for(vaccine) {
                continue;
            }

            for device in &self.devices {
                if !device.is_suitable_for_route(&route.route) {
                    continue;
                }

                let score = route.calculate_efficiency() * device.calculate_precision();
                if score > best_score {
                    best_score = score;
                    best_combination = Some((route.clone(), device.clone()));
                }
            }
        }

        best_combination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_route() {
        let route = DeliveryRoute::new(RouteType::Intramuscular {
            muscle: "deltoid".into(),
            depth: 25.0,
        });

        assert!(route.calculate_efficiency() > 0.8);
    }

    #[test]
    fn test_delivery_device() {
        let device = DeliveryDevice::new(
            DeviceType::Autoinjector {
                mechanism: "spring".into(),
                safety_features: vec!["needle_guard".into()],
            },
            0.5,
        );

        assert!(device.calculate_precision() > 0.95);
    }

    #[test]
    fn test_delivery_optimizer() {
        let mut optimizer = DeliveryOptimizer::new();
        
        optimizer.add_route(DeliveryRoute::new(RouteType::Intramuscular {
            muscle: "deltoid".into(),
            depth: 25.0,
        }));

        optimizer.add_device(DeliveryDevice::new(
            DeviceType::Autoinjector {
                mechanism: "spring".into(),
                safety_features: vec!["needle_guard".into()],
            },
            0.5,
        ));

        let vaccine = VaccineType::MRNA {
            sequence: "AUGGGC".into(),
            modifications: vec![],
            delivery_system: DeliverySystem::LipidNanoparticle {
                composition: HashMap::new(),
                size: 100.0,
                charge: 0.0,
            },
        };

        let optimal = optimizer.optimize(&vaccine);
        assert!(optimal.is_some());
    }
} 