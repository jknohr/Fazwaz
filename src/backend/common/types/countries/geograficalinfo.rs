use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxInfo {
    property_tax_rate: f32,
    transfer_tax_rate: f32,
    special_tax_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipRights {
    can_sell: bool,
    can_inherit: bool,
    can_lease: bool,
    can_mortgage: bool,
    requires_permit: bool,
    foreign_ownership_allowed: bool,
    max_foreign_ownership_percentage: Option<f32>,
    typical_duration_years: Option<u32>,
    minimum_holding_period: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipType {
    name: String,
    local_name: Option<String>,
    category: String,  // Residential, Commercial, Agricultural, etc.
    description: String,
    rights: OwnershipRights,
    required_documents: Vec<String>,
    restrictions: Vec<String>,
    tax_info: TaxInfo,
    legal_references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    name: String,
    local_name: Option<String>,
    currency_code: String,
    ownership_types: Vec<OwnershipType>,
    special_zones: Option<Vec<String>>,
    foreign_ownership_rules: String,
    required_permits: Vec<String>,
    legal_framework: Vec<String>,
}

pub struct GlobalOwnershipSystem {
    countries: HashMap<String, Country>,
}

impl GlobalOwnershipSystem {
    pub fn new() -> Self {
        let mut system = Self {
            countries: HashMap::new(),
        };
        system.initialize_countries();
        system
    }

    fn initialize_countries(&mut self) {
        // Thailand
        self.countries.insert(
            "Thailand".to_string(),
            Country {
                name: "Thailand".to_string(),
                local_name: Some("ประเทศไทย".to_string()),
                currency_code: "THB".to_string(),
                ownership_types: vec![
                    OwnershipType {
                        name: "Freehold Condominium".to_string(),
                        local_name: Some("กรรมสิทธิ์ห้องชุด".to_string()),
                        category: "Residential".to_string(),
                        description: "Full ownership of condominium unit with foreign quota".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: true,
                            requires_permit: false,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: Some(49.0),
                            typical_duration_years: None,
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Condominium Title Deed (โฉนด)".to_string(),
                            "Foreign Exchange Transaction Form".to_string(),
                            "Foreign Quota Certification".to_string(),
                        ],
                        restrictions: vec![
                            "49% foreign ownership quota per condominium".to_string(),
                            "Must bring funds from abroad".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.02,
                            transfer_tax_rate: 0.01,
                            special_tax_rules: vec![
                                "Specific Business Tax 3.3%".to_string(),
                                "Withholding Tax 1%".to_string(),
                            ],
                        },
                        legal_references: vec![
                            "Condominium Act B.E. 2522".to_string(),
                            "Land Code B.E. 2497".to_string(),
                        ],
                    },
                    OwnershipType {
                        name: "Thai Company Freehold".to_string(),
                        local_name: Some("กรรมสิทธิ์บริษัทไทย".to_string()),
                        category: "Commercial".to_string(),
                        description: "Property ownership through Thai limited company".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: true,
                            requires_permit: true,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: Some(49.0),
                            typical_duration_years: None,
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Company Registration".to_string(),
                            "Title Deed".to_string(),
                            "BOI Permit (if applicable)".to_string(),
                            "Foreign Business License".to_string(),
                        ],
                        restrictions: vec![
                            "51% Thai ownership required".to_string(),
                            "Preferential voting rights for Thai shareholders".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.03,
                            transfer_tax_rate: 0.01,
                            special_tax_rules: vec![
                                "Corporate Income Tax 20%".to_string(),
                                "Business Registration Tax".to_string(),
                            ],
                        },
                        legal_references: vec![
                            "Civil and Commercial Code".to_string(),
                            "Foreign Business Act B.E. 2542".to_string(),
                        ],
                    },
                    OwnershipType {
                        name: "Leasehold".to_string(),
                        local_name: Some("สิทธิการเช่า".to_string()),
                        category: "Residential/Commercial".to_string(),
                        description: "Long-term lease rights up to 30 years with renewal options".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: false,
                            requires_permit: false,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: Some(30),
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Lease Agreement".to_string(),
                            "Registration of Lease".to_string(),
                            "Land Office Registration".to_string(),
                        ],
                        restrictions: vec![
                            "Maximum 30 years per term".to_string(),
                            "Renewal must be registered".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.02,
                            transfer_tax_rate: 0.01,
                            special_tax_rules: vec![
                                "Lease Registration Fee 1%".to_string(),
                                "Stamp Duty 0.1%".to_string(),
                            ],
                        },
                        legal_references: vec![
                            "Civil and Commercial Code Section 537-571".to_string(),
                        ],
                    },
                    OwnershipType {
                        name: "Usufruct".to_string(),
                        local_name: Some("สิทธิเก็บกิน".to_string()),
                        category: "Special Rights".to_string(),
                        description: "Right to use and enjoy property for lifetime or specified period".to_string(),
                        rights: OwnershipRights {
                            can_sell: false,
                            can_inherit: false,
                            can_lease: true,
                            can_mortgage: false,
                            requires_permit: false,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: Some(30),
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Usufruct Agreement".to_string(),
                            "Registration at Land Office".to_string(),
                        ],
                        restrictions: vec![
                            "Non-transferable".to_string(),
                            "Expires with death of usufructuary".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.02,
                            transfer_tax_rate: 0.01,
                            special_tax_rules: vec![
                                "Registration Fee 1%".to_string(),
                            ],
                        },
                        legal_references: vec![
                            "Civil and Commercial Code Section 1417-1428".to_string(),
                        ],
                    },
                    OwnershipType {
                        name: "Superficies".to_string(),
                        local_name: Some("สิทธิเหนือพื้นดิน".to_string()),
                        category: "Special Rights".to_string(),
                        description: "Right to own buildings on another's land".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: false,
                            requires_permit: false,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: Some(30),
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Superficies Agreement".to_string(),
                            "Building Permit".to_string(),
                            "Land Office Registration".to_string(),
                        ],
                        restrictions: vec![
                            "Limited to buildings only".to_string(),
                            "Land remains separate ownership".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.02,
                            transfer_tax_rate: 0.01,
                            special_tax_rules: vec![
                                "Building Tax".to_string(),
                            ],
                        },
                        legal_references: vec![
                            "Civil and Commercial Code Section 1410-1416".to_string(),
                        ],
                    },
                ],
                special_zones: Some(vec![
                    "BOI Investment Zones".to_string(),
                    "Special Economic Zones".to_string(),
                    "Eastern Economic Corridor".to_string(),
                ]),
                foreign_ownership_rules: "Foreigners cannot own land directly but can own condominiums within foreign quota and structures on land".to_string(),
                required_permits: vec![
                    "Foreign Exchange Transaction Form".to_string(),
                    "Work Permit (if applicable)".to_string(),
                    "BOI promotion (if applicable)".to_string(),
                ],
                legal_framework: vec![
                    "Land Code B.E. 2497".to_string(),
                    "Condominium Act B.E. 2522".to_string(),
                    "Foreign Business Act B.E. 2542".to_string(),
                    "Civil and Commercial Code".to_string(),
                ],
            },
        );
        // Denmark Example
        self.countries.insert(
            "Denmark".to_string(),
            Country {
                name: "Denmark".to_string(),
                local_name: Some("Danmark".to_string()),
                currency_code: "DKK".to_string(),
                ownership_types: vec![
                    OwnershipType {
                        name: "Full Private Ownership".to_string(),
                        local_name: Some("Fuld Privat Ejerskab".to_string()),
                        category: "Residential".to_string(),
                        description: "Complete ownership of property and land".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: true,
                            requires_permit: false,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: None,
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Skøde (Title Deed)".to_string(),
                            "BBR Extract".to_string(),
                        ],
                        restrictions: vec![],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.92,
                            transfer_tax_rate: 0.6,
                            special_tax_rules: vec![],
                        },
                        legal_references: vec!["Danish Property Law §1".to_string()],
                    },
                    OwnershipType {
                        name: "Andelsbolig".to_string(),
                        local_name: Some("Andelsbolig".to_string()),
                        category: "Cooperative".to_string(),
                        description: "Cooperative housing ownership".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: false,
                            can_mortgage: true,
                            requires_permit: true,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: None,
                            minimum_holding_period: Some(1),
                        },
                        required_documents: vec![
                            "Andelsbevis".to_string(),
                            "Vedtægter".to_string(),
                        ],
                        restrictions: vec![
                            "Must be approved by board".to_string(),
                            "Primary residence requirement".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.0,
                            transfer_tax_rate: 0.0,
                            special_tax_rules: vec!["Cooperative tax rules".to_string()],
                        },
                        legal_references: vec!["Andelsboligloven".to_string()],
                    },
                    OwnershipType {
                        name: "Ejerlejlighed".to_string(),
                        local_name: Some("Ejerlejlighed".to_string()),
                        category: "Condominium".to_string(),
                        description: "Owner-occupied apartment".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: true,
                            requires_permit: false,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: None,
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Ejerlejlighedsskøde".to_string(),
                            "Vedtægter".to_string(),
                        ],
                        restrictions: vec![],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.92,
                            transfer_tax_rate: 0.6,
                            special_tax_rules: vec![],
                        },
                        legal_references: vec!["Ejerlejlighedsloven".to_string()],
                    },
                    OwnershipType {
                        name: "Almene Boliger".to_string(),
                        local_name: Some("Almene Boliger".to_string()),
                        category: "Social Housing".to_string(),
                        description: "Non-profit social housing".to_string(),
                        rights: OwnershipRights {
                            can_sell: false,
                            can_inherit: false,
                            can_lease: false,
                            can_mortgage: false,
                            requires_permit: true,
                            foreign_ownership_allowed: true,
                            max_foreign_ownership_percentage: None,
                            typical_duration_years: None,
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Lejekontrakt".to_string(),
                            "Medlemsbevis".to_string(),
                        ],
                        restrictions: vec![
                            "Income limits may apply".to_string(),
                            "Waiting list system".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.0,
                            transfer_tax_rate: 0.0,
                            special_tax_rules: vec!["Social housing exemptions".to_string()],
                        },
                        legal_references: vec!["Almenboligloven".to_string()],
                    },
                    OwnershipType {
                        name: "Landbrug".to_string(),
                        local_name: Some("Landbrug".to_string()),
                        category: "Agricultural".to_string(),
                        description: "Agricultural property ownership".to_string(),
                        rights: OwnershipRights {
                            can_sell: true,
                            can_inherit: true,
                            can_lease: true,
                            can_mortgage: true,
                            requires_permit: true,
                            foreign_ownership_allowed: false,
                            max_foreign_ownership_percentage: Some(0.0),
                            typical_duration_years: None,
                            minimum_holding_period: None,
                        },
                        required_documents: vec![
                            "Landbrugsskøde".to_string(),
                            "Green Card (agricultural education)".to_string(),
                        ],
                        restrictions: vec![
                            "Residence obligation".to_string(),
                            "Agricultural education requirement".to_string(),
                        ],
                        tax_info: TaxInfo {
                            property_tax_rate: 0.74,
                            transfer_tax_rate: 0.6,
                            special_tax_rules: vec!["Agricultural tax benefits".to_string()],
                        },
                        legal_references: vec!["Landbrugsloven".to_string()],
                    },
                ],
                special_zones: Some(vec![
                    "Sommerhusområder".to_string(),
                    "Kystnære områder".to_string(),
                ]),
                foreign_ownership_rules: "EU citizens have same rights as Danish citizens except for agricultural land and summer houses".to_string(),
                required_permits: vec![
                    "CPR number for registration".to_string(),
                    "Special permit for summer houses".to_string(),
                ],
                legal_framework: vec![
                    "Tinglysningsloven".to_string(),
                    "Boligreguleringsloven".to_string(),
                    "Planloven".to_string(),
                ],
            },
        );
    }

    pub fn add_country(&mut self, name: String, country_data: Country) -> Result<(), String> {
        if self.countries.contains_key(&name) {
            return Err("Country already exists".to_string());
        }
        self.countries.insert(name, country_data);
        Ok(())
    }

    pub fn get_country(&self, name: &str) -> Option<&Country> {
        self.countries.get(name)
    }

    pub fn update_ownership_type(
        &mut self,
        country_name: &str,
        ownership_type_name: &str,
        new_data: OwnershipType,
    ) -> Result<(), String> {
        if let Some(country) = self.countries.get_mut(country_name) {
            if let Some(ownership_type) = country.ownership_types
                .iter_mut()
                .find(|ot| ot.name == ownership_type_name) {
                *ownership_type = new_data;
                Ok(())
            } else {
                Err("Ownership type not found".to_string())
            }
        } else {
            Err("Country not found".to_string())
        }
    }

    pub fn add_ownership_type(
        &mut self,
        country_name: &str,
        ownership_type: OwnershipType,
    ) -> Result<(), String> {
        if let Some(country) = self.countries.get_mut(country_name) {
            country.ownership_types.push(ownership_type);
            Ok(())
        } else {
            Err("Country not found".to_string())
        }
    }
}