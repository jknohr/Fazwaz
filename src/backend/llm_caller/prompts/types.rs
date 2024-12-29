use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub scene_type: String,
    pub content_type: Option<String>,
    pub quality_score: f32,
    pub features: Vec<String>,
    pub condition: ConditionAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionAnalysis {
    pub overall: f32,
    pub issues: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub analysis: ImageAnalysis,
} 