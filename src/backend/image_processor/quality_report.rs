use super::image_utils::QualityAnalysis;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct QualityReport {
    pub overall_score: f32,
    pub issues: Vec<QualityIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct QualityIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub enum IssueSeverity {
    Critical,
    Major,
    Minor,
}

#[derive(Debug, Serialize)]
pub enum IssueCategory {
    Blur,
    Perspective,
    Lighting,
    Windows,
    Composition,
    ColorBalance,
    Detail,
}

impl QualityReport {
    pub fn from_analysis(analysis: &QualityAnalysis) -> Self {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Blur detection
        if analysis.is_blurry {
            issues.push(QualityIssue {
                severity: IssueSeverity::Critical,
                category: IssueCategory::Blur,
                description: "Image is blurry or out of focus".to_string(),
            });
            recommendations.push("Use a tripod or increase shutter speed".to_string());
            recommendations.push("Ensure proper focus on key architectural elements".to_string());
        }
        
        // Perspective issues
        if analysis.has_perspective_issues {
            issues.push(QualityIssue {
                severity: IssueSeverity::Major,
                category: IssueCategory::Perspective,
                description: "Vertical lines are not straight".to_string(),
            });
            recommendations.push("Position camera parallel to walls".to_string());
            recommendations.push("Use a tilt-shift lens or correct in post-processing".to_string());
        }
        
        // Lighting analysis
        if analysis.has_poor_lighting {
            issues.push(QualityIssue {
                severity: IssueSeverity::Major,
                category: IssueCategory::Lighting,
                description: "Uneven or poor lighting conditions".to_string(),
            });
            recommendations.push("Add supplementary lighting to dark areas".to_string());
            recommendations.push("Shoot during optimal daylight hours".to_string());
        }
        
        // Window exposure
        if analysis.window_overexposure {
            issues.push(QualityIssue {
                severity: IssueSeverity::Major,
                category: IssueCategory::Windows,
                description: "Windows are overexposed".to_string(),
            });
            recommendations.push("Use HDR techniques or flash to balance window exposure".to_string());
        }
        
        // Composition score
        if analysis.composition_score < 0.6 {
            issues.push(QualityIssue {
                severity: IssueSeverity::Minor,
                category: IssueCategory::Composition,
                description: "Suboptimal composition".to_string(),
            });
            recommendations.push("Follow rule of thirds for better composition".to_string());
            recommendations.push("Include leading lines to create depth".to_string());
        }
        
        // Color balance
        if analysis.color_balance < 0.7 {
            issues.push(QualityIssue {
                severity: IssueSeverity::Minor,
                category: IssueCategory::ColorBalance,
                description: "Color cast or incorrect white balance".to_string(),
            });
            recommendations.push("Use correct white balance setting".to_string());
            recommendations.push("Consider using color checker card".to_string());
        }
        
        let overall_score = calculate_overall_score(analysis);
        
        Self {
            overall_score,
            issues,
            recommendations,
        }
    }
}

fn calculate_overall_score(analysis: &QualityAnalysis) -> f32 {
    let mut score = 0.0;
    let mut weight_sum = 0.0;
    
    // Weight different factors
    let weights = [
        (analysis.composition_score, 0.3),
        (analysis.vertical_alignment, 0.2),
        (analysis.room_depth_score, 0.15),
        (analysis.lighting_uniformity, 0.15),
        (analysis.color_balance, 0.1),
        (analysis.detail_preservation, 0.1),
    ];
    
    for (value, weight) in weights.iter() {
        score += value * weight;
        weight_sum += weight;
    }
    
    // Penalize for critical issues
    if analysis.is_blurry {
        score *= 0.5;
    }
    if analysis.has_perspective_issues {
        score *= 0.7;
    }
    if analysis.has_poor_lighting {
        score *= 0.8;
    }
    
    score / weight_sum
} 