use image::DynamicImage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistogramStats {
    pub mean: f32,
    pub median: u8,
    pub std_dev: f32,
    pub peaks: Vec<(usize, u32)>,
    pub total_pixels: u32,
    pub dark_fraction: f32,   // Fraction of pixels in dark regions (<64)
    pub light_fraction: f32,  // Fraction of pixels in bright regions (>192)
    pub mid_fraction: f32,    // Fraction of pixels in middle range (64-192)
    pub contrast_ratio: f32,  // Dynamic range ratio
    pub exposure_bias: f32,   // How far from ideal exposure (-1.0 to 1.0)
    pub highlight_clipping: f32, // Percentage of potentially clipped highlights
    pub shadow_detail: f32,   // Amount of detail in shadow areas
    pub window_probability: f32, // Likelihood of containing windows (bright spots)
}

pub fn get_histogram_statistics(histogram: &[u32]) -> HistogramStats {
    let total_pixels: u32 = histogram.iter().sum();
    
    // Basic statistics
    let mean = calculate_mean(histogram, total_pixels);
    let median = calculate_median(histogram, total_pixels);
    let std_dev = calculate_std_dev(histogram, mean, total_pixels);
    
    // Real estate specific metrics
    let dark_fraction = calculate_dark_fraction(histogram, total_pixels);
    let light_fraction = calculate_light_fraction(histogram, total_pixels);
    let mid_fraction = 1.0 - dark_fraction - light_fraction;
    
    // Calculate exposure bias (-1.0 to 1.0, where 0 is ideal)
    let ideal_mean = 127.0;
    let exposure_bias = (mean - ideal_mean) / ideal_mean;
    
    // Detect highlight clipping (percentage of pixels > 250)
    let highlight_clipping = histogram.iter()
        .skip(250)
        .sum::<u32>() as f32 / total_pixels as f32;
    
    // Calculate shadow detail (variance in dark regions)
    let shadow_detail = calculate_shadow_detail(histogram);
    
    // Estimate window probability based on bright regions pattern
    let window_probability = estimate_window_probability(histogram, total_pixels);

    HistogramStats {
        mean,
        median,
        std_dev,
        peaks: find_peaks(histogram),
        total_pixels,
        dark_fraction,
        light_fraction,
        mid_fraction,
        contrast_ratio: calculate_contrast_ratio(histogram),
        exposure_bias,
        highlight_clipping,
        shadow_detail,
        window_probability,
    }
}

fn calculate_mean(histogram: &[u32], total_pixels: u32) -> f32 {
    histogram.iter()
        .enumerate()
        .map(|(i, &count)| i as f32 * count as f32)
        .sum::<f32>() / total_pixels as f32
}

fn calculate_median(histogram: &[u32], total_pixels: u32) -> u8 {
    let mut cumsum = 0;
    let median_count = total_pixels / 2;
    histogram.iter()
        .enumerate()
        .find(|(_, &count)| {
            cumsum += count;
            cumsum >= median_count
        })
        .map_or(0, |(i, _)| i) as u8
}

fn calculate_std_dev(histogram: &[u32], mean: f32, total_pixels: u32) -> f32 {
    let variance = histogram.iter()
        .enumerate()
        .map(|(i, &count)| {
            let diff = i as f32 - mean;
            diff * diff * count as f32
        })
        .sum::<f32>() / total_pixels as f32;
    variance.sqrt()
}

fn calculate_dark_fraction(histogram: &[u32], total: u32) -> f32 {
    histogram.iter()
        .take(64)
        .sum::<u32>() as f32 / total as f32
}

fn calculate_light_fraction(histogram: &[u32], total: u32) -> f32 {
    histogram.iter()
        .skip(192)
        .sum::<u32>() as f32 / total as f32
}

fn calculate_contrast_ratio(histogram: &[u32]) -> f32 {
    let first_nonzero = histogram.iter()
        .enumerate()
        .find(|(_, &count)| count > 0)
        .map(|(i, _)| i)
        .unwrap_or(0);
        
    let last_nonzero = histogram.iter()
        .enumerate()
        .rev()
        .find(|(_, &count)| count > 0)
        .map(|(i, _)| i)
        .unwrap_or(255);
        
    (last_nonzero as f32 + 1.0) / (first_nonzero as f32 + 1.0)
}

fn find_peaks(histogram: &[u32]) -> Vec<(usize, u32)> {
    histogram.iter()
        .enumerate()
        .filter(|&(i, &count)| {
            if i == 0 || i == 255 { return false; }
            count > histogram[i-1] && count > histogram[i+1]
        })
        .map(|(i, &count)| (i, count))
        .collect()
}

fn estimate_window_probability(histogram: &[u32], total: u32) -> f32 {
    // Windows typically create bright spots with sharp transitions
    let bright_region_size = histogram.iter()
        .skip(200)
        .sum::<u32>() as f32 / total as f32;
    
    let has_sharp_transition = histogram.windows(2)
        .skip(180) // Look for transitions in bright areas
        .any(|w| (w[1] as f32 - w[0] as f32).abs() > (total as f32 * 0.01));
    
    if has_sharp_transition && bright_region_size > 0.05 {
        (bright_region_size * 2.0).min(1.0)
    } else {
        bright_region_size
    }
}

fn calculate_shadow_detail(histogram: &[u32]) -> f32 {
    // Calculate variance in shadow regions (0-64)
    let shadow_values: Vec<(f32, f32)> = histogram.iter()
        .take(64)
        .enumerate()
        .map(|(i, &count)| (i as f32, count as f32))
        .collect();
    
    if shadow_values.is_empty() {
        return 0.0;
    }
    
    let mean = shadow_values.iter()
        .map(|(i, count)| i * count)
        .sum::<f32>() / shadow_values.iter()
        .map(|(_, count)| count)
        .sum::<f32>();
        
    shadow_values.iter()
        .map(|(i, count)| {
            let diff = i - mean;
            diff * diff * count
        })
        .sum::<f32>().sqrt() / 64.0
}

pub fn analyze_histogram(img: &DynamicImage) -> Vec<u32> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let mut histogram = vec![0u32; 256];
    
    // Optimized weights for real estate photography
    const R_WEIGHT: f32 = 0.299; // Emphasize red for warm interior tones
    const G_WEIGHT: f32 = 0.587; // Standard green weight for natural perception
    const B_WEIGHT: f32 = 0.114; // Reduce blue influence to prevent window/sky bias
    
    // Split image into regions for parallel processing
    let chunk_size = (height / 4).max(1); // Process in 4 chunks minimum
    let chunks: Vec<_> = (0..height)
        .step_by(chunk_size as usize)
        .map(|y| {
            let mut chunk_hist = vec![0u32; 256];
            let chunk_end = (y + chunk_size).min(height);
            
            // Process each chunk
            for cy in y..chunk_end {
                for x in 0..width {
                    let pixel = rgb_img.get_pixel(x, cy);
                    
                    // Apply perceptual color weights and gamma correction
                    let intensity = (
                        R_WEIGHT * (pixel[0] as f32).powf(2.2) + 
                        G_WEIGHT * (pixel[1] as f32).powf(2.2) + 
                        B_WEIGHT * (pixel[2] as f32).powf(2.2)
                    ).powf(1.0/2.2);
                    
                    // HDR-aware intensity mapping
                    let mapped_intensity = if intensity > 255.0 {
                        // Soft clipping for HDR content
                        255.0 - (255.0 / (1.0 + (intensity - 255.0) * 0.1))
                    } else {
                        intensity
                    };
                    
                    // Convert to 8-bit with proper rounding
                    let index = (mapped_intensity + 0.5) as usize;
                    chunk_hist[index.min(255)] += 1;
                }
            }
            chunk_hist
        })
        .collect();
    
    // Combine chunk histograms
    for chunk in chunks {
        for (i, &count) in chunk.iter().enumerate() {
            histogram[i] += count;
        }
    }
    
    histogram
}