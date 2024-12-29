use super::quality_report::{QualityReport, IssueSeverity, IssueCategory};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct BatchQualityReport {
    pub overall_batch_score: f32,
    pub total_images: usize,
    pub issue_summary: IssueSummary,
    pub quality_distribution: QualityDistribution,
    pub content_type_analysis: HashMap<String, ContentTypeStats>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct IssueSummary {
    pub critical_issues: usize,
    pub major_issues: usize,
    pub minor_issues: usize,
    pub issues_by_category: HashMap<IssueCategory, usize>,
}

#[derive(Debug, Serialize)]
pub struct QualityDistribution {
    pub excellent: usize,  // Score > 0.8
    pub good: usize,      // Score 0.6-0.8
    pub fair: usize,      // Score 0.4-0.6
    pub poor: usize,      // Score < 0.4
}

#[derive(Debug, Serialize)]
pub struct ContentTypeStats {
    pub count: usize,
    pub avg_score: f32,
    pub common_issues: Vec<IssueCategory>,
}

impl BatchQualityReport {
    pub fn new() -> Self {
        Self {
            overall_batch_score: 0.0,
            total_images: 0,
            issue_summary: IssueSummary::new(),
            quality_distribution: QualityDistribution::new(),
            content_type_analysis: HashMap::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn add_report(&mut self, report: &QualityReport, content_type: &str) {
        self.total_images += 1;
        
        // Update quality distribution
        match report.overall_score {
            score if score > 0.8 => self.quality_distribution.excellent += 1,
            score if score > 0.6 => self.quality_distribution.good += 1,
            score if score > 0.4 => self.quality_distribution.fair += 1,
            _ => self.quality_distribution.poor += 1,
        }
        
        // Update issue summary
        for issue in &report.issues {
            match issue.severity {
                IssueSeverity::Critical => self.issue_summary.critical_issues += 1,
                IssueSeverity::Major => self.issue_summary.major_issues += 1,
                IssueSeverity::Minor => self.issue_summary.minor_issues += 1,
            }
            *self.issue_summary.issues_by_category
                .entry(issue.category.clone())
                .or_insert(0) += 1;
        }
        
        // Update room type statistics
        let stats = self.content_type_analysis
            .entry(content_type.to_string())
            .or_insert_with(ContentTypeStats::new);
        stats.update(report);
        
        // Recalculate overall batch score
        self.update_overall_score();
    }

    fn update_overall_score(&mut self) {
        if self.total_images == 0 {
            return;
        }
        
        let quality_score = (self.quality_distribution.excellent * 4 +
                           self.quality_distribution.good * 3 +
                           self.quality_distribution.fair * 2 +
                           self.quality_distribution.poor) as f32;
                           
        self.overall_batch_score = quality_score / (self.total_images * 4) as f32;
    }
}

impl IssueSummary {
    fn new() -> Self {
        Self {
            critical_issues: 0,
            major_issues: 0,
            minor_issues: 0,
            issues_by_category: HashMap::new(),
        }
    }
}

impl QualityDistribution {
    fn new() -> Self {
        Self {
            excellent: 0,
            good: 0,
            fair: 0,
            poor: 0,
        }
    }
}

impl ContentTypeStats {
    fn new() -> Self {
        Self {
            count: 0,
            avg_score: 0.0,
            common_issues: Vec::new(),
        }
    }

    fn update(&mut self, report: &QualityReport) {
        self.count += 1;
        self.avg_score = (self.avg_score * (self.count - 1) as f32 + 
                         report.overall_score) / self.count as f32;
        
        // Update common issues
        let mut issue_counts = HashMap::new();
        for issue in &report.issues {
            *issue_counts.entry(issue.category.clone()).or_insert(0) += 1;
        }
        
        // Keep top 3 most common issues
        self.common_issues = issue_counts.into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .take(3)
            .map(|(category, _)| category)
            .collect();
    }
} 