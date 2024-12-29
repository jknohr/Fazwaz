use std::sync::Arc;
use image::{DynamicImage, ImageFormat, GenericImageView, ImageBuffer, Rgba, Luma};
use webp::Encoder;
use tracing::{info, instrument};

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use rexiv2::Metadata as XmpMetadata;
use crate::backend::image_processor::color::{Rgb as ColorRgb, ImageEnhancement};
use crate::backend::image_processor::image_utils::{ 
    detect_edges, 
    detect_architectural_lines,
    detect_quality_issues,
    QualityAnalysis
};
use crate::backend::image_processor::histogram::{get_histogram_statistics, analyze_histogram};
use imageproc::{
    gradients::sobel_gradients,
    filter::gaussian_blur_f32,
    geometric_transformations::{Projection, Interpolation, warp},
    hough::PolarLine,
};
use prometheus::{HistogramVec, register_histogram_vec};
use std::hash::Hash;
use uuid7::Uuid as Uuid7;
use futures::future::try_join_all;

use crate::backend::common::{
    error::error::{Result, AppError, ImageError, ImageValidationError},
    types::id_types::{ListingId, ImageId, BatchId},
    validation::image_validation::{MAX_WIDTH, MAX_HEIGHT},
};

// Add this struct if not defined in b2_storage.rs
pub struct ImageMetrics {
    pub image_processing_duration: HistogramVec,
    pub batch_processing_duration: HistogramVec,
    pub image_size_bytes: HistogramVec,
    pub image_dimensions: HistogramVec,
}

impl ImageMetrics {
    pub fn new() -> Result<Self> {
        Ok(Self {
            image_processing_duration: register_histogram_vec!(
                "image_processing_duration_seconds",
                "Time spent processing individual images",
                &["content_type", "operation"]
            )?,
            batch_processing_duration: register_histogram_vec!(
                "batch_processing_duration_seconds",
                "Time spent processing image batches",
                &["listing_id"]
            )?,
            image_size_bytes: register_histogram_vec!(
                "image_size_bytes",
                "Size of processed images in bytes",
                &["content_type"]
            )?,
            image_dimensions: register_histogram_vec!(
                "image_dimensions_pixels",
                "Image dimensions after processing",
                &["dimension"]
            )?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    LivingRoom,
    Bedroom,
    Kitchen,
    Bathroom,
    OtherInterior,
    Exterior,
    View,
    FloorPlan,
    TitlePaper,
    SPAContract,
    Reservation,
    RentalAgreement,
    ListingAgreement,   
}

pub struct ImageProcessor {
    metrics: Arc<ImageMetrics>,
    max_size: usize,
    supported_formats: Vec<ImageFormat>,
    enhancement_config: ImageEnhancementConfig 
}
// Add this enum to select presets
pub enum ImagePreset {
    Interior,
    Exterior,
    Twilight,
    Default,
}

impl ImageProcessor {
    #[instrument(skip(self, image_data))]
    pub async fn process_image(
        &self,
        listing_id: &ListingId,
        image_data: Vec<u8>,
        content_type: ContentType,
    ) -> Result<ProcessedImage> {
        let image_id = ImageId::generate();
        let filename = format!("{}-{}.webp", listing_id.as_str(), image_id.as_str());
        
        // Validate dimensions (1080p-4K)
        let img = image::load_from_memory(&image_data)?;
        let (width, height) = img.dimensions();
        if width < 1920 || height < 1080 || width > 3840 || height > 2160 {
            return Err(AppError::InvalidInput("Image dimensions must be between 1080p and 4K".into()));
        }

        // Clone img before first use
        let enhanced = self.enhance_image(img.clone(), content_type)?;
        
        // Convert to WebP with 0.9 quality
        let webp_data = self.convert_to_webp(&enhanced, 0.9)?;
        
        // Add XMP metadata
        let metadata = self.create_metadata(listing_id, &image_id, &filename, content_type.clone())?;
        let final_data = self.add_xmp_metadata(&webp_data, &metadata)?;

        // Use original img for quality analysis
        let quality_analysis = detect_quality_issues(&img);
        
        Ok(ProcessedImage {
            id: image_id,
            listing_id: listing_id.clone(),
            filename,
            size: final_data.len() as i64,
            data: final_data,
            width,
            height,
            content_type,
            quality_analysis,
        })
    }

    fn enhance_image(&self, img: DynamicImage, content_type: ContentType) -> Result<DynamicImage> {
        // Quick analysis of the image
        let analysis = self.analyze_image(&img)?;
        let config = self.get_room_specific_config(&content_type, &analysis);
        
        let mut img_buffer = img.to_rgba8();
        
        // Apply lens distortion correction if needed
        if analysis.needs_perspective_correction {
            img_buffer = self.correct_perspective(img_buffer)?;
        }

        // Apply local contrast enhancement for architectural details
        if content_type == ContentType::Exterior || content_type == ContentType::FloorPlan {
            img_buffer = self.enhance_architectural_details(img_buffer)?;
        }

        // Process each pixel with enhanced color management
        for pixel in img_buffer.pixels_mut() {
            let mut rgb = ColorRgb {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            };

            // Apply room-specific color enhancements
            match content_type {
                ContentType::Kitchen | ContentType::Bathroom => {
                    // Enhance whites and reduce yellow cast
                    rgb.adjust_white_balance(-0.1, 0.0);
                },
                ContentType::LivingRoom | ContentType::Bedroom => {
                    // Warmer, more inviting tones
                    rgb.adjust_white_balance(0.05, 0.0);
                },
                ContentType::Exterior if analysis.is_twilight => {
                    // Enhance blue hour colors
                    rgb.adjust_white_balance(-0.15, 0.0);
                    rgb.adjust_saturation(1.2);
                },
                _ => {}
            }

            // Apply global enhancements
            rgb.adjust_contrast(config.contrast_boost);
            rgb.adjust_saturation(config.color_enhancement_strength);
            rgb.adjust_shadows(config.shadow_recovery);
            rgb.adjust_highlights(-config.highlight_protection);

            pixel[0] = rgb.r;
            pixel[1] = rgb.g;
            pixel[2] = rgb.b;
        }

        // Final pass for global adjustments
        if analysis.needs_sharpening {
            img_buffer = self.apply_smart_sharpening(img_buffer, &config)?;
        }

        Ok(DynamicImage::ImageRgba8(img_buffer))
    }

    fn add_xmp_metadata(&self, data: &[u8], metadata: &ImageMetadata) -> Result<Vec<u8>> {
        let temp_path = std::env::temp_dir().join(format!("temp_{}.webp", metadata.image_id));
        std::fs::write(&temp_path, data)?;
        
        let xmp = XmpMetadata::new_from_path(&temp_path)?;
        xmp.set_tag_string("Xmp.dc.identifier", &metadata.image_id.to_string())?;
        xmp.set_tag_string("Xmp.dc.title", &metadata.filename)?;
        xmp.set_tag_string("Xmp.dc.created", &metadata.created_at.to_rfc3339())?;
        xmp.set_tag_string("Xmp.neural-reef.listingId", &metadata.listing_id.to_string())?;
        xmp.set_tag_string("Xmp.neural-reef.processingVersion", &metadata.processing_version)?;
        xmp.save_to_file(&temp_path)?;
        
        let final_data = std::fs::read(&temp_path)?;
        std::fs::remove_file(temp_path)?;
        
        Ok(final_data)
    }

    fn optimize_image(&self, img: DynamicImage) -> Result<DynamicImage> {
        let (width, height) = img.dimensions();
        
        // Check if resizing needed
        if width > MAX_WIDTH || height > MAX_HEIGHT {
            let ratio = f64::min(
                MAX_WIDTH as f64 / width as f64,
                MAX_HEIGHT as f64 / height as f64
            );
            let new_width = (width as f64 * ratio) as u32;
            let new_height = (height as f64 * ratio) as u32;
            
            Ok(img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3))
        } else {
            Ok(img)
        }
    }

    fn convert_to_webp(&self, img: &DynamicImage, quality: f32) -> Result<Vec<u8>> {
        let encoder = Encoder::from_image(img)
            .map_err(|e| AppError::ImageError(ImageError::ConversionError(e.to_string())))?;
            
        let memory = encoder.encode(quality);
        Ok(memory.to_vec())
    }

    // Add method to switch presets based on scene type
    pub fn set_enhancement_preset(&mut self, preset: ImagePreset) {
        self.enhancement_config = match preset {
            ImagePreset::Interior => ImageEnhancementConfig::interior_preset(),
            ImagePreset::Exterior => ImageEnhancementConfig::exterior_preset(),
            ImagePreset::Twilight => ImageEnhancementConfig::twilight_preset(),
            ImagePreset::Default => ImageEnhancementConfig::default(),
        };
    }

    fn create_metadata(&self, listing_id: &ListingId, image_id: &ImageId, filename: &str, content_type: ContentType) -> Result<ImageMetadata> {
        Ok(ImageMetadata {
            image_id: image_id.to_uuid7()?,
            listing_id: listing_id.clone(),
            filename: filename.to_string(),
            content_type: content_type.clone(),
            dimensions: (0, 0),
            file_size: 0,
            image_data: Vec::new(),
            processing_version: "2.0".to_string(),
            enhancement_preset: match content_type {
                ContentType::LivingRoom | ContentType::Bedroom | ContentType::Kitchen | ContentType::Bathroom => "interior",
                ContentType::Exterior | ContentType::View => "exterior",
                _ => "default",
            }.to_string(),
            gps_coordinates: None, // Will be added later in the pipeline
            processing_status: ProcessingStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    fn get_enhancement_config(&self, content_type: &ContentType, time_of_day: Option<TimeOfDay>) -> ImageEnhancementConfig {
        match (content_type, time_of_day) {
            (ContentType::Exterior | ContentType::View, Some(TimeOfDay::Twilight)) => {
                ImageEnhancementConfig::twilight_preset()
            },
            (ContentType::Exterior | ContentType::View, _) => {
                ImageEnhancementConfig::exterior_preset()
            },
            (ContentType::LivingRoom | ContentType::Bedroom | ContentType::Kitchen | ContentType::Bathroom, _) => {
                ImageEnhancementConfig::interior_preset()
            },
            _ => ImageEnhancementConfig::default(),
        }
    }

    async fn validate_and_extract_metadata(&self, data: &[u8]) -> Result<Option<XmpMetadata>> {
        if let Ok(xmp) = XmpMetadata::new_from_buffer(data) {
            // Validate existing metadata
            if xmp.has_tag("Xmp.dc.identifier") {
                return Ok(Some(xmp));
            }
        }
        Ok(None)
    }

    #[instrument(skip(self, data))]
    pub async fn process_batch(
        &self,
        config: BatchProcessingConfig,
        data: Vec<(Vec<u8>, ContentType)>,
    ) -> Result<Vec<ProcessedImage>> {
        info!(
            listing_id = %config.listing_id,
            batch_id = %config.batch_id,
            "Processing image batch"
        );

        // Validate dimensions
        for (image_data, _) in &data {
            let img = image::load_from_memory(image_data)?;
            let (width, height) = img.dimensions();
            if width < 1920 || height < 1080 || width > 3840 || height > 2160 {
                return Err(AppError::ImageValidation(ImageValidationError::InvalidDimensions { 
                    width, 
                    height 
                }));
            }
        }

        // Process images with proper async handling
        let futures = data.into_iter()
            .map(|(image_data, content_type)| self.process_image(
                &config.listing_id,
                image_data,
                content_type
            ));
        
        let processed = try_join_all(futures).await?;
        self.update_batch_status(&config.batch_id, &processed)?;
        Ok(processed)
    }

    fn update_batch_status(&self, batch_id: &BatchId, processed: &[ProcessedImage]) -> Result<()> {
        // Group images by room type
        let mut room_groups = std::collections::HashMap::new();
        
        for image in processed {
            room_groups
                .entry(image.content_type.clone())
                .or_insert_with(Vec::new)
                .push(image.id.clone());
        }

        // Update batch processing status
        for (content_type, images) in room_groups {
            let group = RoomGroup {
                content_type,
                images,
                status: ProcessingStatus::Completed,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            // Update batch status in database (this would be handled by the model layer)
            info!(
                batch_id = %batch_id,
                content_type = ?group.content_type,
                image_count = group.images.len(),
                "Updated batch group status"
            );
        }

        Ok(())
    }

    // Add specialized real estate presets
    pub fn get_room_specific_config(&self, content_type: &ContentType, analysis: &ImageAnalysis) -> ImageEnhancementConfig {
        let mut config = match content_type {
            ContentType::LivingRoom | ContentType::Bedroom => {
                let mut config = ImageEnhancementConfig::interior_preset();
                
                // Fine-tune based on histogram analysis
                if analysis.has_window {
                    // Reduce window recovery if not severely overexposed
                    config.window_recovery_strength = if analysis.is_overexposed { 1.5 } else { 1.2 };
                    config.highlight_protection = if analysis.is_overexposed { 0.95 } else { 0.85 };
                }
                
                if analysis.is_underexposed {
                    // Progressive shadow recovery based on severity
                    config.shadow_recovery = if analysis.is_yellow_cast { 0.4 } else { 0.6 };
                    config.brightness_adjustment = if analysis.needs_sharpening { 12.0 } else { 15.0 };
                }
                
                config
            },
            ContentType::Kitchen | ContentType::Bathroom => {
                let mut config = ImageEnhancementConfig::interior_preset();
                // Enhance reflective surfaces
                config.sharpening_threshold = 0.4;
                config.highlight_protection = 0.92;
                config.white_balance_temp = -0.1; // Cooler for cleaner look
                config
            },
            ContentType::Exterior => {
                let mut config = ImageEnhancementConfig::exterior_preset();
                if analysis.has_sky {
                    config.exterior_sky_enhancement = 1.4;
                    config.highlight_protection = 0.85;
                }
                if analysis.is_twilight {
                    config = ImageEnhancementConfig::twilight_preset();
                }
                config
            },
            _ => ImageEnhancementConfig::default(),
        };

        // Apply global fixes for common real estate photo issues
        if analysis.is_underexposed {
            config.brightness_adjustment += 10.0;
            config.shadow_recovery += 0.2;
        }
        if analysis.is_overexposed {
            config.highlight_protection += 0.1;
            config.brightness_adjustment -= 5.0;
        }
        if analysis.is_yellow_cast {
            config.white_balance_temp -= 0.15;
        }

        config
    }

    // Add these helper methods
    fn analyze_image(&self, img: &DynamicImage) -> Result<ImageAnalysis> {
        // Get detailed histogram stats
        let histogram = analyze_histogram(img);
        let stats = get_histogram_statistics(&histogram);
        
        // Determine exposure issues using histogram data
        let is_underexposed = stats.dark_fraction > 0.3 && stats.mean < 80.0;
        let is_overexposed = stats.highlight_clipping > 0.1 || stats.light_fraction > 0.4;
        
        // Check for color cast using the exposure bias
        let is_yellow_cast = stats.exposure_bias > 0.2 && self.detect_color_cast(img)? > 0.15;
        
        // Enhanced window detection using histogram data
        let has_window = stats.window_probability > 0.2 && 
                        stats.light_fraction > 0.15 && 
                        stats.contrast_ratio > 4.0;
        
        // Improved sky detection combining histogram and spatial analysis
        let has_sky = self.detect_sky_region(img)? > 0.15 && 
                     stats.light_fraction > 0.2;
        
        // More sophisticated twilight detection
        let is_twilight = stats.mean < 100.0 && 
                         stats.window_probability > 0.15 &&
                         stats.shadow_detail < 0.3 &&
                         self.detect_twilight_conditions(img)?;

        // Enhanced sharpening detection using edge analysis
        let edges = detect_edges(img);
        let avg_edge_strength = edges.pixels()
            .map(|p| p[0] as u32)
            .sum::<u32>() as f32 / (edges.width() * edges.height()) as f32;
        let needs_sharpening = avg_edge_strength < 30.0 || stats.std_dev < 20.0;

        // Perspective correction check
        let needs_perspective_correction = if let Ok(lines) = detect_architectural_lines(img) {
            !lines.is_empty() && lines.iter().any(|l| (l.angle_degrees() - 90.0).abs() > 2.0)
        } else {
            false
        };

        Ok(ImageAnalysis {
            is_underexposed,
            is_overexposed,
            is_yellow_cast,
            has_window,
            has_sky,
            is_twilight,
            needs_perspective_correction,
            needs_sharpening,
        })
    }

    fn calculate_histogram(&self, img: &DynamicImage) -> Result<Vec<u32>> {
        let gray = img.to_luma8();
        let mut histogram = vec![0; 256];
        
        for pixel in gray.pixels() {
            histogram[pixel[0] as usize] += 1;
        }
        
        Ok(histogram)
    }

    fn detect_edges(&self, img: &DynamicImage) -> Result<DynamicImage> {
        let gray = img.to_luma8();
        let sobel = imageproc::gradients::sobel_gradients(&gray);
        
        // Convert 16-bit to 8-bit by scaling
        let converted = ImageBuffer::from_fn(sobel.width(), sobel.height(), |x, y| {
            let pixel = sobel.get_pixel(x, y)[0];
            Luma([((pixel as f32) / 256.0) as u8])
        });
        
        Ok(DynamicImage::ImageLuma8(converted))
    }

    fn find_vertical_lines(&self, edges: &DynamicImage) -> Result<Vec<Line>> {
        let gray = edges.to_luma8();
        let lines = imageproc::hough::detect_lines(
            &gray,
            imageproc::hough::LineDetectionOptions {
                vote_threshold: 150,
                suppression_radius: 8,
            }
        ).into_iter()
        .filter(|line| {
            let angle = (line.angle_in_degrees as f32).to_radians().abs();
            angle < 10.0 || angle > 170.0
        })
        .map(|line| Line {
            rho: line.r,
            theta: (line.angle_in_degrees as f32).to_radians(),
        })
        .collect();

        Ok(lines)
    }

    fn calculate_line_convergence(&self, lines: &[Line]) -> Result<f32> {
        if lines.len() < 2 {
            return Ok(0.0);
        }

        let mut max_convergence = 0.0f32;
        for i in 0..lines.len() {
            for j in i+1..lines.len() {
                let angle_diff = (lines[i].theta - lines[j].theta).abs();
                max_convergence = f32::max(max_convergence, angle_diff);
            }
        }

        Ok(max_convergence / std::f32::consts::PI)
    }

    fn apply_smart_sharpening(&self, img: ImageBuffer<Rgba<u8>, Vec<u8>>, config: &ImageEnhancementConfig) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let kernel = [
            [-1.0, -1.0, -1.0],
            [-1.0,  9.0, -1.0],
            [-1.0, -1.0, -1.0],
        ];

        let mut output = img.clone();
        let (width, height) = img.dimensions();

        for y in 1..height-1 {
            for x in 1..width-1 {
                let mut new_pixel = [0.0; 4];
                
                // Apply convolution with edge detection
                for (ky, krow) in kernel.iter().enumerate() {
                    for (kx, &k) in krow.iter().enumerate() {
                        let px = img.get_pixel(x + kx as u32 - 1, y + ky as u32 - 1);
                        for c in 0..3 {
                            new_pixel[c] += k * px[c] as f32;
                        }
                    }
                }

                // Apply sharpening threshold
                let pixel = output.get_pixel_mut(x, y);
                for c in 0..3 {
                    let mut val = new_pixel[c].max(0.0).min(255.0) as u8;
                    let diff = (val as i32 - pixel[c] as i32).abs();
                    if diff < config.sharpening_threshold as i32 {
                        val = pixel[c];
                    }
                    pixel[c] = val;
                }
                pixel[3] = 255; // Preserve alpha
            }
        }

        Ok(output)
    }

    fn enhance_architectural_details(&self, img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let edges = self.detect_edges(&DynamicImage::ImageRgba8(img.clone()))?;
        let mut enhanced = img.clone();

        // Enhance edges while preserving architectural details
        for (x, y, pixel) in enhanced.enumerate_pixels_mut() {
            let edge_strength = edges.get_pixel(x, y)[0] as f32 / 255.0;
            if edge_strength > 0.1 {
                // Enhance contrast along edges
                for c in 0..3 {
                    let val = pixel[c] as f32;
                    pixel[c] = (val * (1.0 + edge_strength * 0.3)).min(255.0) as u8;
                }
            }
        }

        Ok(enhanced)
    }

    fn correct_perspective(&self, img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let edges = self.detect_edges(&DynamicImage::ImageRgba8(img.clone()))?;
        let lines = self.find_vertical_lines(&edges)?;
        
        if lines.is_empty() {
            return Ok(img);
        }

        let avg_angle = lines.iter()
            .map(|line| line.theta)
            .sum::<f32>() / lines.len() as f32;

        // Create affine transformation matrix
        let transform = Projection::from_matrix([
            avg_angle.cos(), -avg_angle.sin(), 0.0,
            avg_angle.sin(), avg_angle.cos(), 0.0,
            0.0, 0.0, 1.0
        ]).ok_or_else(|| AppError::ImageProcessing("Failed to create projection matrix".into()))?;

        // Apply transformation
        Ok(warp(
            &img,
            &transform,
            Interpolation::Bilinear,
            Rgba([0, 0, 0, 0])
        ))
    }

    // Add these new methods
    fn analyze_channels(&self, img: &DynamicImage) -> Result<ChannelAnalysis> {
        let rgb = img.to_rgb8();
        let mut r_hist = vec![0u32; 256];
        let mut g_hist = vec![0u32; 256];
        let mut b_hist = vec![0u32; 256];
        
        // Build histograms for each channel
        for pixel in rgb.pixels() {
            r_hist[pixel[0] as usize] += 1;
            g_hist[pixel[1] as usize] += 1;
            b_hist[pixel[2] as usize] += 1;
        }

        let total_pixels = (img.width() * img.height()) as f32;
        
        Ok(ChannelAnalysis {
            r_mean: self.calculate_mean(&r_hist, total_pixels),
            g_mean: self.calculate_mean(&g_hist, total_pixels),
            b_mean: self.calculate_mean(&b_hist, total_pixels),
            r_std_dev: self.calculate_std_dev(&r_hist, total_pixels),
            g_std_dev: self.calculate_std_dev(&g_hist, total_pixels),
            b_std_dev: self.calculate_std_dev(&b_hist, total_pixels),
            dark_threshold: self.find_dark_threshold(&r_hist, &g_hist, &b_hist),
            bright_threshold: self.find_bright_threshold(&r_hist, &g_hist, &b_hist),
        })
    }

    fn normalize_channels(&self, img: &mut DynamicImage, analysis: &ChannelAnalysis) -> Result<()> {
        let mut rgb = img.to_rgb8();
        
        // Calculate adjustment factors
        let target_mean = 128.0;
        let r_factor = target_mean / analysis.r_mean;
        let g_factor = target_mean / analysis.g_mean;
        let b_factor = target_mean / analysis.b_mean;

        // Apply selective normalization
        for pixel in rgb.pixels_mut() {
            // Normalize each channel while preserving relative relationships
            if pixel[0] as f32 <= analysis.dark_threshold {
                pixel[0] = (pixel[0] as f32 * r_factor * 1.2).min(255.0) as u8;
            } else if pixel[0] as f32 >= analysis.bright_threshold {
                pixel[0] = (pixel[0] as f32 * r_factor * 0.8).max(0.0) as u8;
            }

            // Similar adjustments for G and B channels
            if pixel[1] as f32 <= analysis.dark_threshold {
                pixel[1] = (pixel[1] as f32 * g_factor * 1.2).min(255.0) as u8;
            } else if pixel[1] as f32 >= analysis.bright_threshold {
                pixel[1] = (pixel[1] as f32 * g_factor * 0.8).max(0.0) as u8;
            }

            if pixel[2] as f32 <= analysis.dark_threshold {
                pixel[2] = (pixel[2] as f32 * b_factor * 1.2).min(255.0) as u8;
            } else if pixel[2] as f32 >= analysis.bright_threshold {
                pixel[2] = (pixel[2] as f32 * b_factor * 0.8).max(0.0) as u8;
            }
        }

        *img = DynamicImage::ImageRgb8(rgb);
        Ok(())
    }

    // Add helper methods
    fn calculate_mean(&self, hist: &[u32], total: f32) -> f32 {
        let mut sum = 0.0;
        for (value, &count) in hist.iter().enumerate() {
            sum += (value as f32) * (count as f32);
        }
        sum / total
    }

    fn calculate_std_dev(&self, hist: &[u32], total: f32) -> f32 {
        let mean = self.calculate_mean(hist, total);
        let mut sum_sq = 0.0;
        for (value, &count) in hist.iter().enumerate() {
            let diff = value as f32 - mean;
            sum_sq += diff * diff * count as f32;
        }
        (sum_sq / total).sqrt()
    }

    fn find_dark_threshold(&self, r: &[u32], g: &[u32], b: &[u32]) -> f32 {
        // Find the point where cumulative histogram reaches 10%
        let total = r.iter().sum::<u32>();
        let target = total / 10;
        let mut sum = 0;
        
        for i in 0..256 {
            sum += r[i].max(g[i]).max(b[i]);
            if sum >= target {
                return i as f32;
            }
        }
        0.0
    }

    fn find_bright_threshold(&self, r: &[u32], g: &[u32], b: &[u32]) -> f32 {
        // Find the point where cumulative histogram from the top reaches 90%
        let total = r.iter().sum::<u32>();
        let target = total * 9 / 10;
        let mut sum = 0;
        
        for i in (0..256).rev() {
            sum += r[i].max(g[i]).max(b[i]);
            if sum >= target {
                return i as f32;
            }
        }
        255.0
    }

    // Add this new method for real estate specific analysis
    fn analyze_real_estate_image(&self, img: &DynamicImage) -> Result<RealEstateAnalysis> {
        let channel_analysis = self.analyze_channels(img)?;
        
        // Analyze specific real estate photo characteristics
        Ok(RealEstateAnalysis {
            channel_stats: channel_analysis,
            window_regions: self.detect_window_regions(img)?,
            interior_lighting: self.analyze_interior_lighting(img)?,
            color_cast: self.detect_color_temperature(img)?,
            vertical_lines: self.detect_vertical_lines(img)?,
            room_brightness: self.calculate_room_brightness(img)?,
        })
    }

    fn detect_window_regions(&self, img: &DynamicImage) -> Result<Vec<WindowRegion>> {
        let rgb = img.to_rgb8();
        let mut windows = Vec::new();
        let (width, height) = rgb.dimensions();
        let block_size = 32;  // Analysis block size
        
        // Scan image in blocks to detect potential window regions
        for y in (0..height).step_by(block_size as usize) {
            for x in (0..width).step_by(block_size as usize) {
                let mut block_brightness = 0.0;
                let mut pixel_count = 0;
                
                // Calculate average brightness for block
                for by in y..std::cmp::min(y + block_size, height) {
                    for bx in x..std::cmp::min(x + block_size, width) {
                        let pixel = rgb.get_pixel(bx, by);
                        block_brightness += (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);
                        pixel_count += 1;
                    }
                }
                
                let avg_brightness = block_brightness / pixel_count as f32;
                
                // Detect window characteristics
                if avg_brightness > 0.85 {  // Very bright region
                    windows.push(WindowRegion {
                        x,
                        y,
                        width: std::cmp::min(block_size, width - x),
                        height: std::cmp::min(block_size, height - y),
                        brightness: avg_brightness,
                        is_overexposed: avg_brightness > 0.95
                    });
                }
            }
        }
        
        // Merge adjacent window regions
        self.merge_window_regions(&mut windows);
        
        Ok(windows)
    }

    fn merge_window_regions(&self, regions: &mut Vec<WindowRegion>) {
        regions.sort_by_key(|r| (r.y, r.x));
        let mut i = 0;
        while i < regions.len() {
            let mut j = i + 1;
            while j < regions.len() {
                if self.regions_overlap(&regions[i], &regions[j]) {
                    // Merge overlapping regions
                    regions[i].width = std::cmp::max(
                        regions[i].x + regions[i].width,
                        regions[j].x + regions[j].width
                    ) - regions[i].x;
                    regions[i].height = std::cmp::max(
                        regions[i].y + regions[i].height,
                        regions[j].y + regions[j].height
                    ) - regions[i].y;
                    regions[i].brightness = (regions[i].brightness + regions[j].brightness) / 2.0;
                    regions[i].is_overexposed = regions[i].is_overexposed || regions[j].is_overexposed;
                    regions.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    fn regions_overlap(&self, r1: &WindowRegion, r2: &WindowRegion) -> bool {
        r1.x < r2.x + r2.width && 
        r1.x + r1.width > r2.x &&
        r1.y < r2.y + r2.height && 
        r1.y + r1.height > r2.y
    }

    fn analyze_interior_lighting(&self, img: &DynamicImage) -> Result<InteriorLighting> {
        let rgb = img.to_rgb8();
        let light_sources = Vec::new();
        let mut ambient_level = 0.0;
        let mut samples = 0;
        
        // Sample grid points for ambient light
        let step = 20; // Sample every 20 pixels
        for y in (0..rgb.height()).step_by(step) {
            for x in (0..rgb.width()).step_by(step) {
                let pixel = rgb.get_pixel(x, y);
                let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);
                ambient_level += brightness;
                samples += 1;
            }
        }
        
        ambient_level /= samples as f32;
        
        Ok(InteriorLighting {
            ambient_level,
            light_sources,
            lighting_uniformity: self.calculate_lighting_uniformity(&rgb)?,
            shadow_depth: self.calculate_shadow_depth(&rgb)?,
        })
    }

    fn enhance_real_estate_photo(&self, img: &mut DynamicImage, analysis: &RealEstateAnalysis) -> Result<()> {
        // Apply targeted enhancements based on analysis
        if analysis.channel_stats.r_std_dev > 50.0 || 
           analysis.channel_stats.g_std_dev > 50.0 || 
           analysis.channel_stats.b_std_dev > 50.0 {
            self.normalize_channels(img, &analysis.channel_stats)?;
        }

        // Handle window exposure
        for window in &analysis.window_regions {
            self.balance_window_exposure(img, window)?;
        }

        // Improve interior lighting
        if analysis.interior_lighting.ambient_level < 0.4 {
            self.enhance_interior_lighting(img, &analysis.interior_lighting)?;
        }

        // Fix color temperature issues
        if analysis.color_cast.temperature_offset.abs() > 0.1 {
            self.correct_color_temperature(img, analysis.color_cast.temperature_offset)?;
        }

        Ok(())
    }

    fn detect_color_temperature(&self, img: &DynamicImage) -> Result<ColorTemperature> {
        let rgb = img.to_rgb8();
        let mut r_sum = 0.0;
        let mut g_sum = 0.0;
        let mut b_sum = 0.0;
        let total_pixels = (rgb.width() * rgb.height()) as f32;

        for pixel in rgb.pixels() {
            r_sum += pixel[0] as f32;
            g_sum += pixel[1] as f32;
            b_sum += pixel[2] as f32;
        }

        r_sum /= total_pixels * 255.0;
        g_sum /= total_pixels * 255.0;
        b_sum /= total_pixels * 255.0;

        Ok(ColorTemperature {
            temperature_offset: (r_sum - b_sum) * 2.0, // Rough estimate of color temperature
            tint_offset: g_sum - (r_sum + b_sum) / 2.0, // Green-magenta balance
        })
    }

    fn calculate_room_brightness(&self, img: &DynamicImage) -> Result<RoomBrightness> {
        let gray = img.to_luma8();
        let mut histogram = vec![0u32; 256];
        let total_pixels = (gray.width() * gray.height()) as f32;

        for pixel in gray.pixels() {
            histogram[pixel[0] as usize] += 1;
        }

        let dark_count = histogram.iter().take(64).sum::<u32>() as f32;
        let bright_count = histogram.iter().skip(192).sum::<u32>() as f32;
        let mean_brightness = self.calculate_mean(&histogram, total_pixels);
        let std_dev = self.calculate_std_dev(&histogram, total_pixels);

        Ok(RoomBrightness {
            overall_brightness: mean_brightness / 255.0,
            uniformity: 1.0 - (std_dev / 128.0),
            dark_areas_percentage: dark_count / total_pixels * 100.0,
            bright_areas_percentage: bright_count / total_pixels * 100.0,
        })
    }

    fn check_exposure_histogram(&self, img: &DynamicImage) -> Result<f32> {
        let gray = img.to_luma8();
        let mut histogram = vec![0u32; 256];
        let total_pixels = (gray.width() * gray.height()) as f32;
        
        for pixel in gray.pixels() {
            histogram[pixel[0] as usize] += 1;
        }
        
        // Calculate weighted average brightness
        let mut weighted_sum = 0.0;
        for (value, count) in histogram.iter().enumerate() {
            weighted_sum += (value as f32 / 255.0) * (*count as f32 / total_pixels);
        }
        
        Ok(weighted_sum)
    }

    fn detect_bright_regions(&self, img: &DynamicImage) -> Result<f32> {
        let gray = img.to_luma8();
        let mut bright_pixels = 0;
        let total_pixels = (gray.width() * gray.height()) as f32;
        
        for pixel in gray.pixels() {
            if pixel[0] > 220 {
                bright_pixels += 1;
            }
        }
        
        Ok(bright_pixels as f32 / total_pixels)
    }

    fn check_sharpness(&self, img: &DynamicImage) -> Result<f32> {
        let gray = img.to_luma8();
        let edges = sobel_gradients(&gray);
        let mut sharpness = 0.0;
        
        for pixel in edges.pixels() {
            sharpness += pixel[0] as f32;
        }
        
        Ok(sharpness / (img.width() * img.height()) as f32 / 255.0)
    }

    fn detect_sky_region(&self, img: &DynamicImage) -> Result<f32> {
        let rgb = img.to_rgb8();
        let height = img.height() as f32;
        let width = img.width() as f32;
        let mut sky_pixels = 0;
        
        // Check top third of image for sky-like colors
        for y in 0..(height as u32 / 3) {
            for x in 0..img.width() {
                let pixel = rgb.get_pixel(x, y);
                if self.is_sky_color(pixel[0], pixel[1], pixel[2]) {
                    sky_pixels += 1;
                }
            }
        }
        
        Ok(sky_pixels as f32 / (width * height / 3.0))
    }

    fn apply_unsharp_mask(&self, img: &image::RgbImage, sigma: f32, amount: f32) -> Result<image::RgbImage> {
        let blurred = gaussian_blur_f32(img, sigma);
        let mut output = img.clone();
        
        for (x, y, pixel) in output.enumerate_pixels_mut() {
            let original = img.get_pixel(x, y);
            let blur = blurred.get_pixel(x, y);
            
            // Apply unsharp mask formula: original + amount * (original - blur)
            for c in 0..3 {
                let val = original[c] as f32 + amount * (original[c] as f32 - blur[c] as f32);
                pixel[c] = val.clamp(0.0, 255.0) as u8;
            }
        }
        
        Ok(output)
    }

    fn calculate_lighting_uniformity(&self, img: &image::RgbImage) -> Result<f32> {
        let mut brightness_values = Vec::new();
        let step = 20; // Sample every 20 pixels
        
        for y in (0..img.height()).step_by(step) {
            for x in (0..img.width()).step_by(step) {
                let pixel = img.get_pixel(x, y);
                let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);
                brightness_values.push(brightness);
            }
        }
        
        // Calculate standard deviation of brightness values
        let mean = brightness_values.iter().sum::<f32>() / brightness_values.len() as f32;
        let variance = brightness_values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / brightness_values.len() as f32;
            
        Ok(1.0 - variance.sqrt())
    }

    fn calculate_shadow_depth(&self, img: &image::RgbImage) -> Result<f32> {
        let mut darkest: f32 = 255.0;
        let step = 20; // Sample every 20 pixels
        
        for y in (0..img.height()).step_by(step) {
            for x in (0..img.width()).step_by(step) {
                let pixel = img.get_pixel(x, y);
                let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
                darkest = f32::min(darkest, brightness);
            }
        }
        
        Ok(darkest / 255.0)
    }

    fn detect_vertical_lines(&self, img: &DynamicImage) -> Result<Vec<VerticalLine>> {
        let edges = self.detect_edges(img)?;
        let gray = edges.to_luma8();
        
        let hough_lines = imageproc::hough::detect_lines(
            &gray,
            imageproc::hough::LineDetectionOptions {
                vote_threshold: 150,
                suppression_radius: 8,
            }
        ).into_iter()
        .filter(|line| {
            let angle = line.angle_in_degrees as f32 + 90.0;
            angle.abs() < 10.0 || angle.abs() > 170.0
        })
        .map(|line| VerticalLine {
            start: (
                ((line.r * (line.angle_in_degrees as f32).to_radians().cos()) as f32) as u32,
                0
            ),
            end: (
                ((line.r * (line.angle_in_degrees as f32).to_radians().cos()) as f32) as u32,
                img.height()
            ),
            strength: (line.angle_in_degrees as f32).to_radians().cos().abs(),
        })
        .collect();

        Ok(hough_lines)
    }

    fn detect_color_cast(&self, img: &DynamicImage) -> Result<f32> {
        let rgb = img.to_rgb8();
        let mut r_sum = 0.0;
        let mut g_sum = 0.0;
        let mut b_sum = 0.0;
        let total_pixels = (rgb.width() * rgb.height()) as f32;

        for pixel in rgb.pixels() {
            r_sum += pixel[0] as f32;
            g_sum += pixel[1] as f32;
            b_sum += pixel[2] as f32;
        }

        // Calculate average channel values
        r_sum /= total_pixels * 255.0;
        g_sum /= total_pixels * 255.0;
        b_sum /= total_pixels * 255.0;

        // Return yellow cast measure (high R+G compared to B)
        Ok(((r_sum + g_sum) / 2.0 - b_sum).max(0.0))
    }

    fn detect_twilight_conditions(&self, img: &DynamicImage) -> Result<bool> {
        let rgb = img.to_rgb8();
        let mut blue_dominance = 0.0;
        let total_pixels = (rgb.width() * rgb.height()) as f32;

        for pixel in rgb.pixels() {
            // Check for blue-hour characteristics
            if pixel[2] > pixel[0] && pixel[2] > pixel[1] {
                blue_dominance += (pixel[2] as f32 - (pixel[0].max(pixel[1]) as f32)) / 255.0;
            }
        }

        blue_dominance /= total_pixels;
        Ok(blue_dominance > 0.15) // Threshold for twilight detection
    }

    fn check_perspective(&self, img: &DynamicImage) -> Result<bool> {
        let edges = self.detect_edges(img)?;
        let lines = self.find_vertical_lines(&edges)?;
        
        if lines.is_empty() {
            return Ok(false);
        }

        // Calculate line convergence
        let convergence = self.calculate_line_convergence(&lines)?;
        
        // If convergence is above threshold, perspective correction is needed
        Ok(convergence > 0.05)  // 5% threshold for perspective distortion
    }

    fn is_sky_color(&self, r: u8, g: u8, b: u8) -> bool {
        let (r, g, b) = (r as f32, g as f32, b as f32);
        
        // Check for blue sky
        let is_blue = b > r && b > g && b > 100.0;
        
        // Check for white/gray clouds
        let is_cloud = (r + g + b) / 3.0 > 200.0 && 
            (r - g).abs() < 20.0 && 
            (r - b).abs() < 20.0;
            
        // Check for sunset colors
        let is_sunset = r > 180.0 && g > 100.0 && b < 150.0;
        
        is_blue || is_cloud || is_sunset
    }

    fn balance_window_exposure(&self, img: &mut DynamicImage, window: &WindowRegion) -> Result<()> {
        let mut buffer = img.to_rgba8();
        
        // Apply local tone mapping to window region
        for y in window.y..window.y + window.height {
            for x in window.x..window.x + window.width {
                if x < buffer.width() && y < buffer.height() {
                    let pixel = buffer.get_pixel_mut(x, y);
                    if window.is_overexposed {
                        // Recover overexposed windows
                        for c in 0..3 {
                            let val = pixel[c] as f32;
                            pixel[c] = (val * 0.85).min(255.0) as u8;
                        }
                    } else {
                        // Enhance window detail
                        for c in 0..3 {
                            let val = pixel[c] as f32;
                            pixel[c] = (val * 1.15).min(255.0) as u8;
                        }
                    }
                }
            }
        }
        
        *img = DynamicImage::ImageRgba8(buffer);
        Ok(())
    }

    fn enhance_interior_lighting(&self, img: &mut DynamicImage, lighting: &InteriorLighting) -> Result<()> {
        let mut buffer = img.to_rgba8();
        
        // Apply global ambient light boost
        let boost_factor = (0.5 / lighting.ambient_level).min(1.5);
        
        for pixel in buffer.pixels_mut() {
            // Boost darker areas more than already-bright areas
            for c in 0..3 {
                let val = pixel[c] as f32;
                let normalized = val / 255.0;
                // Apply non-linear boost that affects shadows more than highlights
                let boosted = ((1.0 - normalized).powf(0.7) * boost_factor + normalized) * 255.0;
                pixel[c] = boosted.min(255.0) as u8;
            }
        }
        
        *img = DynamicImage::ImageRgba8(buffer);
        Ok(())
    }

    fn correct_color_temperature(&self, img: &mut DynamicImage, offset: f32) -> Result<()> {
        let mut rgb = img.to_rgb8();
        
        for pixel in rgb.pixels_mut() {
            if offset < 0.0 {
                // Cool down: reduce red, increase blue
                pixel[0] = ((pixel[0] as f32) * (1.0 + offset)).min(255.0) as u8;
                pixel[2] = ((pixel[2] as f32) * (1.0 - offset)).min(255.0) as u8;
            } else {
                // Warm up: increase red, reduce blue
                pixel[0] = ((pixel[0] as f32) * (1.0 + offset)).min(255.0) as u8;
                pixel[2] = ((pixel[2] as f32) * (1.0 - offset)).min(255.0) as u8;
            }
        }
        
        *img = DynamicImage::ImageRgb8(rgb);
        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedImage {
    pub id: ImageId,
    pub listing_id: ListingId,
    pub filename: String,
    pub content_type: ContentType,
    pub size: i64,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub quality_analysis: QualityAnalysis,
}

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub image_id: Uuid7,
    pub listing_id: ListingId,
    pub filename: String,
    pub content_type: ContentType,
    pub dimensions: (u32, u32),
    pub file_size: usize,
    pub image_data: Vec<u8>,
    pub processing_version: String,
    pub enhancement_preset: String,
    pub gps_coordinates: Option<(f64, f64)>,
    pub processing_status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum TimeOfDay {
    Day,
    Twilight,
    Night,
}

#[derive(Debug, Clone)]
pub struct BatchProcessingConfig {
    pub listing_id: ListingId,
    pub batch_id: BatchId,
    pub room_groups: Vec<RoomGroup>,
    pub quality: f32,  // WebP quality (0.9)
    pub processing_version: String,
}

#[derive(Debug, Clone)]
pub struct RoomGroup {
    pub content_type: ContentType,
    pub images: Vec<ImageId>,
    pub status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

#[derive(Debug)]
struct ImageAnalysis {
    is_underexposed: bool,
    is_overexposed: bool,
    is_yellow_cast: bool,
    has_window: bool,
    has_sky: bool,
    is_twilight: bool,
    needs_perspective_correction: bool,
    needs_sharpening: bool,
}

#[derive(Debug, Clone)]
struct Line {
    rho: f32,
    theta: f32,
}

impl Line {
    fn angle(&self) -> f32 {
        self.theta
    }
}

// Add this struct to store channel analysis results
#[derive(Debug)]
struct ChannelAnalysis {
    r_mean: f32,
    g_mean: f32,
    b_mean: f32,
    r_std_dev: f32,
    g_std_dev: f32,
    b_std_dev: f32,
    dark_threshold: f32,
    bright_threshold: f32,
}

// Add these new types
#[derive(Debug)]
struct RealEstateAnalysis {
    channel_stats: ChannelAnalysis,
    window_regions: Vec<WindowRegion>,
    interior_lighting: InteriorLighting,
    color_cast: ColorTemperature,
    vertical_lines: Vec<VerticalLine>,
    room_brightness: RoomBrightness,
}

#[derive(Debug)]
struct WindowRegion {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    brightness: f32,
    is_overexposed: bool,
}

#[derive(Debug)]
struct InteriorLighting {
    ambient_level: f32,
    light_sources: Vec<LightSource>,
    lighting_uniformity: f32,
    shadow_depth: f32,
}

#[derive(Debug)]
struct ColorTemperature {
    temperature_offset: f32,  // Negative = too cool, Positive = too warm
    tint_offset: f32,        // Green-magenta balance
}

#[derive(Debug)]
struct LightSource {
    x: u32,
    y: u32,
    intensity: f32,
    color_temp: f32,
}

#[derive(Debug)]
struct VerticalLine {
    start: (u32, u32),
    end: (u32, u32),
    strength: f32,
}

#[derive(Debug)]
struct RoomBrightness {
    overall_brightness: f32,
    uniformity: f32,
    dark_areas_percentage: f32,
    bright_areas_percentage: f32,
}

trait PolarLineExt {
    fn angle_degrees(&self) -> f32;
    fn angle_radians(&self) -> f32;
}

impl PolarLineExt for PolarLine {
    fn angle_degrees(&self) -> f32 {
        self.angle_in_degrees as f32
    }

    fn angle_radians(&self) -> f32 {
        (self.angle_in_degrees as f32).to_radians()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BatchProcessingStatus {
    Uploading,
    ValidatingImages,
    EnhancingQuality,
    AnalyzingContent,
    AddingMetadata,
    IndexingVectors,
    UploadingToStorage,
    Completed,
    Failed(String),
}

impl ImageProcessor {
    // Add this method to handle the complete batch processing pipeline
    pub async fn process_listing_batch(
        &self,
        batch_metadata: BatchMetadata,
        images: Vec<(Vec<u8>, ContentType)>
    ) -> Result<Vec<ProcessedImage>> {
        info!(
            listing_id = %batch_metadata.listing_id,
            "Starting batch processing pipeline"
        );

        // Validate dimensions
        for (image_data, _) in &images {
            let img = image::load_from_memory(image_data)?;
            let (width, height) = img.dimensions();
            if width < 1920 || height < 1080 || width > 3840 || height > 2160 {
                return Err(AppError::ImageValidation(ImageValidationError::InvalidDimensions { 
                    width, 
                    height 
                }));
            }
        }

        // Process images with proper async handling
        let futures = images.into_iter()
            .map(|(image_data, content_type)| self.process_image(
                &batch_metadata.listing_id,
                image_data,
                content_type
            ));
        
        let processed = try_join_all(futures).await?;
        Ok(processed)
    }
}

#[derive(Debug, Clone)]
pub struct ImageEnhancementConfig {
    pub contrast_boost: f32,
    pub color_enhancement_strength: f32,
    pub shadow_recovery: f32,
    pub highlight_protection: f32,
    pub sharpening_threshold: f32,
    pub brightness_adjustment: f32,
    pub window_recovery_strength: f32,
    pub white_balance_temp: f32,
    pub exterior_sky_enhancement: f32,
}

impl ImageEnhancementConfig {
    pub fn default() -> Self {
        Self {
            contrast_boost: 1.0,
            color_enhancement_strength: 1.0,
            shadow_recovery: 0.3,
            highlight_protection: 0.2,
            sharpening_threshold: 10.0,
            brightness_adjustment: 0.0,
            window_recovery_strength: 1.0,
            white_balance_temp: 0.0,
            exterior_sky_enhancement: 1.0,
        }
    }

    pub fn interior_preset() -> Self {
        Self {
            contrast_boost: 1.1,
            color_enhancement_strength: 1.1,
            shadow_recovery: 0.4,
            highlight_protection: 0.3,
            sharpening_threshold: 10.0,
            brightness_adjustment: 0.0,
            window_recovery_strength: 1.5,
            white_balance_temp: -0.05,
            exterior_sky_enhancement: 1.0,
        }
    }

    pub fn exterior_preset() -> Self {
        Self {
            contrast_boost: 1.2,
            color_enhancement_strength: 1.2,
            shadow_recovery: 0.2,
            highlight_protection: 0.4,
            sharpening_threshold: 10.0,
            brightness_adjustment: 0.0,
            window_recovery_strength: 0.8,
            white_balance_temp: 0.0,
            exterior_sky_enhancement: 1.4,
        }
    }

    pub fn twilight_preset() -> Self {
        Self {
            contrast_boost: 1.3,
            color_enhancement_strength: 1.3,
            shadow_recovery: 0.5,
            highlight_protection: 0.2,
            sharpening_threshold: 10.0,
            brightness_adjustment: 0.0,
            window_recovery_strength: 1.2,
            white_balance_temp: -0.1,
            exterior_sky_enhancement: 1.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchMetadata {
    pub listing_id: ListingId,
    pub batch_id: BatchId,
    pub room_groups: Vec<RoomGroup>,
    pub quality: f32,  // WebP quality (0.9)
    pub processing_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
