use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, Luma};
use imageproc::filter::gaussian_blur_f32;
use imageproc::{
    gradients::sobel_gradients,
    hough::{detect_lines, LineDetectionOptions, PolarLine}
};
use serde::{Deserialize, Serialize};
use crate::backend::common::error::error::Result;


pub fn unsharp_mask(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>, 
    sigma: f32, 
    amount: f32
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let blurred = gaussian_blur_f32(img, sigma);
    let mut output = img.clone();
    
    // Enhanced unsharp mask with edge preservation
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let blurred_pixel = blurred.get_pixel(x, y);
        
        // Process each channel with edge-aware sharpening
        for c in 0..3 {
            let original = pixel[c] as f32;
            let blurred = blurred_pixel[c] as f32;
            let diff = original - blurred;
            
            // Apply adaptive sharpening based on local contrast
            let sharpening = if diff.abs() > 30.0 {
                // Reduce sharpening for strong edges to prevent halos
                amount * 0.7
            } else {
                amount
            };
            
            let sharpened = original + sharpening * diff;
            pixel[c] = sharpened.max(0.0).min(255.0) as u8;
        }
        pixel[3] = img.get_pixel(x, y)[3]; // Preserve alpha
    }
    
    output
}

pub fn detect_edges(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let grayscale = img.to_luma8();
    let gradients = sobel_gradients(&grayscale);
    
    let (width, height) = grayscale.dimensions();
    let mut edge_image = ImageBuffer::new(width, height);
    
    // Enhanced edge detection with noise suppression
    for y in 0..height {
        for x in 0..width {
            let pixel = gradients.get_pixel(x, y);
            let magnitude = ((pixel[0] as f32).powi(2) + (pixel[1] as f32).powi(2)).sqrt();
            
            // Apply adaptive thresholding for better edge detection
            let threshold = if y < height / 3 {
                // Lower threshold for upper part (potential sky/ceiling)
                10.0
            } else {
                // Standard threshold for main content
                15.0
            };
            
            let edge_strength = if magnitude < threshold {
                0
            } else {
                (magnitude.min(255.0)) as u8
            };
            
            edge_image.put_pixel(x, y, Luma([edge_strength]));
        }
    }
    
    edge_image
}

pub fn detect_architectural_lines(img: &DynamicImage) -> Result<Vec<PolarLine>> {
    let edges = detect_edges(img);
    
    let lines = detect_lines(&edges, LineDetectionOptions {
        vote_threshold: 150,
        suppression_radius: 8,
    });
    
    // Filter and classify lines by orientation
    let vertical_lines: Vec<PolarLine> = lines.into_iter()
        .filter(|line| {
            let angle = line.angle_in_degrees as f32;
            let vertical_deviation = (angle - 90.0).abs();
            
            // Accept lines that are:
            // 1. Nearly vertical (within 10 degrees)
            // 2. Have sufficient strength (avoid noise)
            vertical_deviation < 10.0 && line.r > 50.0
        })
        .collect();

    Ok(vertical_lines)
}

pub fn detect_window_regions(img: &DynamicImage) -> Result<Vec<Rect>> {
    let _edges = detect_edges(img);
    let (width, height) = img.dimensions();
    
    // Use local contrast analysis to find potential window areas
    let mut window_regions = Vec::new();
    let block_size = 32;
    
    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let block_width = ((x + block_size) as u32).min(width) - x as u32;
            let block_height = ((y + block_size) as u32).min(height) - y as u32;
            
            // Analyze local contrast and brightness
            let stats = analyze_block(img, x as u32, y as u32, block_width, block_height);
            
            if is_potential_window(&stats) {
                window_regions.push(Rect {
                    x: x as u32,
                    y: y as u32,
                    width: block_width,
                    height: block_height,
                });
            }
        }
    }
    
    // Merge overlapping regions
    merge_overlapping_regions(&mut window_regions);
    Ok(window_regions)
}

fn analyze_block(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> BlockStats {
    let mut brightness_sum = 0.0;
    let mut min_brightness = f32::MAX;
    let mut max_brightness = f32::MIN;
    let mut pixel_count = 0;
    
    for cy in y..y+height {
        for cx in x..x+width {
            let pixel = img.get_pixel(cx, cy);
            let brightness = 0.299 * pixel[0] as f32 + 
                           0.587 * pixel[1] as f32 + 
                           0.114 * pixel[2] as f32;
            brightness_sum += brightness;
            min_brightness = min_brightness.min(brightness);
            max_brightness = max_brightness.max(brightness);
            pixel_count += 1;
        }
    }
    
    let avg_brightness = brightness_sum / pixel_count as f32;
    let local_contrast = max_brightness - min_brightness;
    
    BlockStats {
        avg_brightness,
        local_contrast,
    }
}

fn is_potential_window(stats: &BlockStats) -> bool {
    // Windows typically have high brightness and high local contrast
    stats.avg_brightness > 180.0 && stats.local_contrast > 30.0
}

fn merge_overlapping_regions(regions: &mut Vec<Rect>) {
    regions.sort_by_key(|r| (r.x, r.y));
    
    let mut i = 0;
    while i < regions.len() {
        let mut j = i + 1;
        while j < regions.len() {
            if regions[i].overlaps(&regions[j]) {
                regions[i] = regions[i].merge(&regions[j]);
                regions.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct BlockStats {
    avg_brightness: f32,
    local_contrast: f32,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
    
    fn merge(&self, other: &Rect) -> Rect {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }
}

pub fn detect_quality_issues(img: &DynamicImage) -> QualityAnalysis {
    let edges = detect_edges(img);
    let window_regions = detect_window_regions(img).unwrap_or_default();
    let lines = detect_architectural_lines(img).unwrap_or_default();
    
    QualityAnalysis {
        is_blurry: check_blur_level(&edges),
        has_perspective_issues: check_perspective(img),
        has_poor_lighting: check_lighting_issues(img),
        window_overexposure: check_window_exposure(&window_regions, img),
        noise_level: estimate_noise_level(img),
        composition_score: analyze_composition(img),
        vertical_alignment: analyze_vertical_alignment(&lines),
        room_depth_score: analyze_room_depth(img),
        lighting_uniformity: analyze_lighting_uniformity(img),
        color_balance: analyze_color_balance(img),
        detail_preservation: analyze_detail_preservation(img),
    }
}

fn check_blur_level(edges: &ImageBuffer<Luma<u8>, Vec<u8>>) -> bool {
    // Calculate edge strength distribution
    let total_edges: f32 = edges.pixels()
        .map(|p| p[0] as f32)
        .sum::<f32>();
    let avg_edge_strength = total_edges / (edges.width() * edges.height()) as f32;
    
    avg_edge_strength < 12.0 // Threshold for blur detection
}

fn check_perspective(img: &DynamicImage) -> bool {
    let lines = detect_architectural_lines(img).unwrap_or_default();
    
    // Check for vertical lines that deviate from 90 degrees
    lines.iter().any(|line| {
        let angle = line.angle_in_degrees as f32;
        (angle - 90.0).abs() > 2.0
    })
}

fn check_lighting_issues(img: &DynamicImage) -> bool {
    let (width, height) = img.dimensions();
    let block_size = 64u32;
    let mut dark_regions = 0;
    let mut bright_regions = 0;
    
    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let stats = analyze_block(img, 
                x as u32, y as u32,
                block_size.min(width - x as u32),
                block_size.min(height - y as u32));
            
            if stats.avg_brightness < 40.0 {
                dark_regions += 1;
            } else if stats.avg_brightness > 220.0 {
                bright_regions += 1;
            }
        }
    }
    
    let total_regions = ((width / block_size as u32) * (height / block_size as u32)) as f32;
    (dark_regions as f32 / total_regions > 0.2) || 
    (bright_regions as f32 / total_regions > 0.2)
}

fn check_window_exposure(windows: &[Rect], img: &DynamicImage) -> bool {
    windows.iter().any(|window| {
        let stats = analyze_block(img, window.x, window.y, window.width, window.height);
        stats.avg_brightness > 240.0 && stats.local_contrast < 10.0
    })
}

fn estimate_noise_level(img: &DynamicImage) -> f32 {
    // Analyze local variance in smooth regions
    let edges = detect_edges(img);
    let (width, height) = img.dimensions();
    let mut noise_sum = 0.0;
    let mut smooth_regions = 0;
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            if edges.get_pixel(x, y)[0] < 10 {
                let local_var = calculate_local_variance(img, x, y);
                noise_sum += local_var;
                smooth_regions += 1;
            }
        }
    }
    
    if smooth_regions > 0 {
        noise_sum / smooth_regions as f32
    } else {
        0.0
    }
}

fn analyze_composition(img: &DynamicImage) -> f32 {
    let (width, height) = img.dimensions();
    let mut score = 0.0;
    
    // Rule of thirds points (30% of total score)
    let thirds_score = analyze_thirds_composition(img);
    score += thirds_score * 0.3;
    
    // Architectural alignment (25% of score)
    if let Ok(lines) = detect_architectural_lines(img) {
        let arch_score = analyze_architectural_composition(&lines, width, height);
        score += arch_score * 0.25;
    }
    
    // Room depth and perspective (25% of score)
    let depth_score = analyze_room_depth(img);
    score += depth_score * 0.25;
    
    // Visual balance and window placement (20% of score)
    let balance_score = analyze_visual_balance(img);
    if let Ok(windows) = detect_window_regions(img) {
        let window_score = analyze_window_placement(&windows, width, height);
        score += balance_score * 0.1 + window_score * 0.1;
    } else {
        score += balance_score * 0.2;
    }
    
    score.min(1.0)
}

fn analyze_thirds_composition(img: &DynamicImage) -> f32 {
    let (width, height) = img.dimensions();
    let edges = detect_edges(img);
    let mut score = 0.0;
    
    // Check both intersection points and lines
    let third_w = width as f32 / 3.0;
    let third_h = height as f32 / 3.0;
    
    // Intersection points
    let points = [
        (third_w, third_h),
        (third_w * 2.0, third_h),
        (third_w, third_h * 2.0),
        (third_w * 2.0, third_h * 2.0),
    ];
    
    for point in points.iter() {
        score += analyze_region_interest(&edges, point.0 as u32, point.1 as u32, 50) * 0.15;
    }
    
    // Vertical and horizontal third lines
    for i in 1..=2 {
        let x = (width as f32 * (i as f32 / 3.0)) as u32;
        let y = (height as f32 * (i as f32 / 3.0)) as u32;
        
        // Vertical lines
        score += analyze_vertical_line(&edges, x) * 0.1;
        // Horizontal lines
        score += analyze_horizontal_line(&edges, y) * 0.1;
    }
    
    score
}

fn analyze_architectural_composition(lines: &[PolarLine], width: u32, height: u32) -> f32 {
    let mut score = 0.0;
    
    // Check for strong vertical lines near edges (framing)
    let edge_verticals = lines.iter()
        .filter(|line| {
            let angle = line.angle_in_degrees as f32;
            let x = line.r * angle.to_radians().cos();
            (angle - 90.0).abs() < 2.0 && 
            (x < width as f32 * 0.2 || x > width as f32 * 0.8)
        })
        .count();
    
    score += (edge_verticals as f32 * 0.2).min(0.4); // Max 0.4 for framing
    
    // Check for converging lines (perspective)
    let converging = analyze_converging_lines(lines, height);
    score += converging * 0.3;
    
    // Check for symmetry in architectural elements
    let symmetry = analyze_architectural_symmetry(lines, width);
    score += symmetry * 0.3;
    
    score
}

fn analyze_room_depth(img: &DynamicImage) -> f32 {
    let edges = detect_edges(img);
    let (_width, height) = edges.dimensions();
    
    // Analyze gradient of edge density from foreground to background
    let mut depth_score = 0.0;
    let sections = 5;
    
    for i in 0..sections {
        let y_start = height * i / sections;
        let y_end = height * (i + 1) / sections;
        let section_strength = calculate_section_edge_strength(&edges, y_start, y_end);
        
        // Weight closer sections more heavily
        let weight = 1.0 - (i as f32 / sections as f32);
        depth_score += section_strength * weight;
    }
    
    depth_score / sections as f32
}

fn analyze_window_placement(windows: &[Rect], width: u32, height: u32) -> f32 {
    let mut score = 0.0;
    
    for window in windows {
        // Check if window is well-placed relative to thirds
        let center_x = window.x + window.width / 2;
        let center_y = window.y + window.height / 2;
        
        let third_w = width / 3;
        let third_h = height / 3;
        
        // Higher score for windows near thirds lines
        let x_score = (center_x as f32 / third_w as f32 % 1.0 - 0.5).abs();
        let y_score = (center_y as f32 / third_h as f32 % 1.0 - 0.5).abs();
        
        score += (1.0 - (x_score + y_score) / 2.0) / windows.len() as f32;
    }
    
    score
}

fn analyze_region_interest(edges: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32, radius: u32) -> f32 {
    let mut total_edge_strength = 0.0;
    let mut pixels_checked = 0;
    
    for dy in -(radius as i32)..=radius as i32 {
        for dx in -(radius as i32)..=radius as i32 {
            let px = (x as i32 + dx) as u32;
            let py = (y as i32 + dy) as u32;
            
            if px < edges.width() && py < edges.height() {
                total_edge_strength += edges.get_pixel(px, py)[0] as f32;
                pixels_checked += 1;
            }
        }
    }
    
    if pixels_checked > 0 {
        (total_edge_strength / pixels_checked as f32) / 255.0
    } else {
        0.0
    }
}

fn analyze_leading_lines(lines: &[PolarLine], width: u32, height: u32) -> f32 {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let mut score = 0.0;
    
    for line in lines {
        // Convert polar coordinates to endpoints
        let (x1, y1, x2, y2) = polar_to_cartesian(line, width, height);
        
        // Check if line leads to center region
        let distance_to_center = point_line_distance(
            center_x, center_y,
            x1, y1, x2, y2
        );
        
        if distance_to_center < (width.min(height) as f32 * 0.2) {
            score += 1.0;
        }
    }
    
    (score / lines.len() as f32).min(1.0)
}

fn analyze_visual_balance(img: &DynamicImage) -> f32 {
    let (width, height) = img.dimensions();
    let center_x = width / 2;
    let mut left_weight = 0.0;
    let mut right_weight = 0.0;
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let intensity = 0.299 * pixel[0] as f32 + 
                          0.587 * pixel[1] as f32 + 
                          0.114 * pixel[2] as f32;
            
            if x < center_x {
                left_weight += intensity;
            } else {
                right_weight += intensity;
            }
        }
    }
    
    let total_weight = left_weight + right_weight;
    if total_weight > 0.0 {
        1.0 - (left_weight - right_weight).abs() / total_weight
    } else {
        0.5
    }
}

fn point_line_distance(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let numerator = ((y2 - y1) * px - (x2 - x1) * py + x2 * y1 - y2 * x1).abs();
    let denominator = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
    
    if denominator > 0.0 {
        numerator / denominator
    } else {
        f32::MAX
    }
}

fn polar_to_cartesian(line: &PolarLine, _width: u32, _height: u32) -> (f32, f32, f32, f32) {
    let rho = line.r;
    let theta = (line.angle_in_degrees as f32).to_radians();
    
    let x0 = rho * theta.cos();
    let y0 = rho * theta.sin();
    
    let alpha = 1000.0; // Line length
    
    let x1 = x0 + alpha * (-theta.sin());
    let y1 = y0 + alpha * theta.cos();
    let x2 = x0 - alpha * (-theta.sin());
    let y2 = y0 - alpha * theta.cos();
    
    (x1, y1, x2, y2)
}

fn calculate_local_variance(img: &DynamicImage, x: u32, y: u32) -> f32 {
    let mut values = Vec::with_capacity(9);
    
    // Sample 3x3 neighborhood
    for dy in -1..=1 {
        for dx in -1..=1 {
            let nx = (x as i32 + dx) as u32;
            let ny = (y as i32 + dy) as u32;
            let p = img.get_pixel(nx, ny);
            let val = 0.299 * p[0] as f32 + 
                     0.587 * p[1] as f32 + 
                     0.114 * p[2] as f32;
            values.push(val);
        }
    }
    
    // Calculate variance
    let mean = values.iter().sum::<f32>() / 9.0;
    values.iter()
        .map(|&v| (v - mean).powi(2))
        .sum::<f32>() / 9.0
}

fn analyze_vertical_line(edges: &ImageBuffer<Luma<u8>, Vec<u8>>, x: u32) -> f32 {
    let mut strength = 0.0;
    for y in 0..edges.height() {
        strength += edges.get_pixel(x, y)[0] as f32;
    }
    strength / (edges.height() as f32 * 255.0)
}

fn analyze_horizontal_line(edges: &ImageBuffer<Luma<u8>, Vec<u8>>, y: u32) -> f32 {
    let mut strength = 0.0;
    for x in 0..edges.width() {
        strength += edges.get_pixel(x, y)[0] as f32;
    }
    strength / (edges.width() as f32 * 255.0)
}

fn analyze_converging_lines(lines: &[PolarLine], _height: u32) -> f32 {
    let mut converging_pairs = 0;
    let mut total_pairs = 0;
    
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines.iter().skip(i + 1) {
            let angle1 = line1.angle_in_degrees as f32;
            let angle2 = line2.angle_in_degrees as f32;
            
            // Check if lines are roughly vertical and converging
            if (angle1 - 90.0).abs() < 20.0 && (angle2 - 90.0).abs() < 20.0 {
                if (angle1 - angle2).abs() > 1.0 {
                    converging_pairs += 1;
                }
                total_pairs += 1;
            }
        }
    }
    
    if total_pairs > 0 {
        converging_pairs as f32 / total_pairs as f32
    } else {
        0.0
    }
}

fn analyze_architectural_symmetry(lines: &[PolarLine], width: u32) -> f32 {
    let center_x = width as f32 / 2.0;
    let mut symmetry_score = 0.0;
    let mut pairs_checked = 0;
    
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines.iter().skip(i + 1) {
            let x1 = line1.r * (line1.angle_in_degrees as f32).to_radians().cos();
            let x2 = line2.r * (line2.angle_in_degrees as f32).to_radians().cos();
            
            // Check if lines are equidistant from center
            let dist1 = (x1 - center_x).abs();
            let dist2 = (x2 - center_x).abs();
            
            if (dist1 - dist2).abs() < 10.0 {
                symmetry_score += 1.0;
            }
            pairs_checked += 1;
        }
    }
    
    if pairs_checked > 0 {
        symmetry_score / pairs_checked as f32
    } else {
        0.0
    }
}

fn calculate_section_edge_strength(edges: &ImageBuffer<Luma<u8>, Vec<u8>>, y_start: u32, y_end: u32) -> f32 {
    let mut total_strength = 0.0;
    let mut pixels_checked = 0;
    
    for y in y_start..y_end {
        for x in 0..edges.width() {
            total_strength += edges.get_pixel(x, y)[0] as f32;
            pixels_checked += 1;
        }
    }
    
    if pixels_checked > 0 {
        total_strength / (pixels_checked as f32 * 255.0)
    } else {
        0.0
    }
}

fn analyze_vertical_alignment(lines: &[PolarLine]) -> f32 {
    if lines.is_empty() {
        return 0.0;
    }
    
    // Calculate how well vertical lines align with true vertical (90 degrees)
    let deviations: Vec<f32> = lines.iter()
        .map(|line| (line.angle_in_degrees as f32 - 90.0).abs())
        .collect();
        
    1.0 - (deviations.iter().sum::<f32>() / (deviations.len() as f32 * 10.0)).min(1.0)
}

fn analyze_lighting_uniformity(img: &DynamicImage) -> f32 {
    let (width, height) = img.dimensions();
    let block_size = 64u32;
    let mut brightnesses = Vec::new();
    
    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let stats = analyze_block(img, 
                x as u32, y as u32,
                block_size.min(width - x as u32),
                block_size.min(height - y as u32));
            brightnesses.push(stats.avg_brightness);
        }
    }
    
    if brightnesses.is_empty() {
        return 0.0;
    }
    
    let mean = brightnesses.iter().sum::<f32>() / brightnesses.len() as f32;
    let variance = brightnesses.iter()
        .map(|&b| (b - mean).powi(2))
        .sum::<f32>() / brightnesses.len() as f32;
        
    1.0 - (variance / 10000.0).min(1.0)
}

fn analyze_color_balance(img: &DynamicImage) -> f32 {
    let (width, height) = img.dimensions();
    let mut r_sum = 0.0;
    let mut g_sum = 0.0;
    let mut b_sum = 0.0;
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            r_sum += pixel[0] as f32;
            g_sum += pixel[1] as f32;
            b_sum += pixel[2] as f32;
        }
    }
    
    let total = (width * height) as f32;
    let r_mean = r_sum / total;
    let g_mean = g_sum / total;
    let b_mean = b_sum / total;
    
    // Check for color cast
    let max_diff = (r_mean - g_mean).abs().max((g_mean - b_mean).abs())
        .max((b_mean - r_mean).abs());
        
    1.0 - (max_diff / 128.0).min(1.0)
}

fn analyze_detail_preservation(img: &DynamicImage) -> f32 {
    let edges = detect_edges(img);
    let histogram = calculate_edge_histogram(&edges);
    
    // Calculate detail preservation score based on edge distribution
    let strong_edges = histogram.iter().skip(128).sum::<u32>() as f32;
    let total_edges = histogram.iter().sum::<u32>() as f32;
    
    if total_edges > 0.0 {
        strong_edges / total_edges
    } else {
        0.0
    }
}

fn calculate_edge_histogram(edges: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u32> {
    let mut histogram = vec![0u32; 256];
    for pixel in edges.pixels() {
        histogram[pixel[0] as usize] += 1;
    }
    histogram
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAnalysis {
    pub is_blurry: bool,
    pub has_perspective_issues: bool,
    pub has_poor_lighting: bool,
    pub window_overexposure: bool,
    pub noise_level: f32,
    pub composition_score: f32,
    pub vertical_alignment: f32,
    pub room_depth_score: f32,
    pub lighting_uniformity: f32,
    pub color_balance: f32,
    pub detail_preservation: f32,
} 