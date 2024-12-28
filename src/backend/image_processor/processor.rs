use std::sync::Arc;
use image::{DynamicImage, ImageFormat, GenericImageView};
use webp::Encoder;
use tracing::{info, warn, instrument};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use rexiv2::Metadata as XmpMetadata;
use photon_rs::{PhotonImage, ColourSpaces, Conv, Effects, Filters, Helpers, Multiple, Native, Text, Channels, Transform};

use crate::backend::common::{
    error::error::{Result, AppError, ImageError},
    types::id_types::{ListingId, ImageId},
    validation::image_validation::{MAX_WIDTH, MAX_HEIGHT},
};

pub struct ImageProcessor {
    file_manager: Arc<FileManager>,
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
    #[instrument(skip(self))]
    pub async fn process_image(
        &self,
        listing_id: &ListingId,
        image_data: Vec<u8>,
        content_type: &str
    ) -> Result<ProcessedImage> {
        let image_id = ImageId::generate();
        let filename = format!("{}-{}", listing_id.as_str(), image_id.as_str());
        
        let timer = self.metrics.image_processing_duration.start_timer();
        
        // Validate format and size
        self.validate_image(&image_data)?;
        
        // Load and process image
        let img = image::load_from_memory(&image_data)?;
        let processed = self.optimize_image(img)?;
        
        // Convert to WebP
        let webp_data = self.convert_to_webp(&processed)?;
        
        // Add XMP metadata
        let metadata = ImageMetadata {
            id: image_id.clone(),
            listing_id: listing_id.clone(),
            filename: filename.clone(),
            created_at: Utc::now(),
            // ... other fields ...
        };
        
        let final_data = self.add_xmp_metadata(&webp_data, &metadata)?;
        
        timer.observe_duration();
        Ok(ProcessedImage {
            id: image_id,
            listing_id: listing_id.clone(),
            filename,
            content_type: "image/webp".to_string(),
            size: final_data.len() as i64,
            data: final_data,
            width: processed.width(),
            height: processed.height(),
        })
    }

    fn add_xmp_metadata(&self, data: &[u8], metadata: &ImageMetadata) -> Result<Vec<u8>> {
        let mut xmp = XmpMetadata::new_from_buffer(data)?;
        
        // Add basic metadata
        xmp.set_tag_string("Xmp.dc.identifier", &metadata.id.to_string())?;
        xmp.set_tag_string("Xmp.dc.title", &metadata.filename)?;
        xmp.set_tag_string("Xmp.dc.created", &metadata.created_at.to_rfc3339())?;
        
        // Add custom namespace for our application
        xmp.set_tag_string("Xmp.neural-reef.listingId", &metadata.listing_id.to_string())?;
        xmp.set_tag_string("Xmp.neural-reef.processingVersion", "1.0")?;
        
        // Save buffer with metadata
        Ok(xmp.save_to_buffer()?)
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

    fn convert_to_webp(&self, img: &DynamicImage) -> Result<Vec<u8>> {
        let encoder = Encoder::from_image(img)
            .map_err(|e| AppError::ImageError(ImageError::ConversionError(e.to_string())))?;
            
        let memory = encoder.encode(90.0); // Quality parameter 0-100
        Ok(memory.to_vec())
    }

    fn enhance_real_estate_photo(&self, img: DynamicImage) -> Result<DynamicImage> {
        // Convert to RGB for processing
        let rgb_img = img.to_rgb8();

        // Use photon_rs for the actual processing, using our config values
        let mut photo = PhotonImage::from_dynamic(img);
        
        // Apply settings from config
        photon_rs::filters::sharpen(&mut photo, self.enhancement_config.sharpening_sigma);
        photon_rs::adjust::brightness(&mut photo, self.enhancement_config.brightness_adjustment);
        photon_rs::adjust::correct_contrast(&mut photo, self.enhancement_config.contrast_boost);
        
        // Handle shadow recovery
        if self.enhancement_config.shadow_recovery > 0.0 {
            photon_rs::adjust::adjust_channel_intensity(
                &mut photo,
                photon_rs::channels::ChannelIndex::Blue, 
                self.enhancement_config.shadow_recovery
            );
        }

        // And so on for other parameters...

        Ok(photo.to_dynamic())
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
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedImage {
    pub id: ImageId,
    pub listing_id: ListingId,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
struct ImageMetadata {
    id: ImageId,
    listing_id: ListingId,
    filename: String,
    created_at: DateTime<Utc>,
    // Add other metadata fields as needed
} 

#[derive(Debug, Clone)]
pub struct ImageEnhancementConfig {
    // Sharpening parameters
    pub sharpening_sigma: f32,      // Controls the radius of sharpening
    pub sharpening_threshold: i32,   // Minimum brightness change to sharpen
    
    // Tone adjustment
    pub contrast_boost: f32,        // Overall contrast enhancement
    pub brightness_adjustment: i32,  // Global brightness adjustment
    pub shadow_recovery: f32,       // Recovery of shadow details
    pub highlight_protection: f32,   // Prevent highlight clipping
    
    // Color enhancement
    pub color_enhancement_strength: f32,  // Vibrance adjustment
    pub saturation_limit: f32,           // Prevent oversaturation
    pub white_balance_temp: f32,         // Color temperature adjustment
    pub white_balance_tint: f32,         // Green-magenta balance
    
    // Geometric corrections
    pub perspective_correction_threshold: f32,  // Vertical/horizontal line detection sensitivity
    pub perspective_correction_strength: f32,   // How aggressively to correct perspective
    
    // Room-specific adjustments
    pub interior_shadow_boost: f32,      // Extra shadow recovery for interiors
    pub window_recovery_strength: f32,    // HDR-like recovery for bright windows
    pub exterior_sky_enhancement: f32,    // Subtle sky enhancement for exterior shots
    
}





impl Default for ImageEnhancementConfig {
    fn default() -> Self {
        Self {
            // Conservative sharpening that enhances architectural details
            // without creating halos or noise
            sharpening_sigma: 0.8,        // Slightly reduced from 1.0 for more natural look
            sharpening_threshold: 7,       // Increased to prevent noise enhancement
            
            // Balanced tone adjustments that maintain realism
            contrast_boost: 1.15,          // Subtle contrast increase
            brightness_adjustment: 5,       // Reduced from 10 for more natural look
            shadow_recovery: 0.3,          // Moderate shadow recovery
            highlight_protection: 0.85,     // Strong highlight protection
            
            // Natural color enhancements
            color_enhancement_strength: 1.08,  // Very subtle color boost
            saturation_limit: 1.2,            // Prevent oversaturation
            white_balance_temp: 0.0,          // Neutral temperature adjustment
            white_balance_tint: 0.0,          // Neutral tint adjustment
            
            // Perspective corrections
            perspective_correction_threshold: 0.015,  // More sensitive detection
            perspective_correction_strength: 0.75,    // Conservative correction
            
            // Room-specific adjustments
            interior_shadow_boost: 0.25,       // Moderate interior shadow recovery
            window_recovery_strength: 0.6,     // Balanced window detail recovery
            exterior_sky_enhancement: 0.15,    // Very subtle sky enhancement
        }
    }
}
impl Interiordefault for ImageEnhancementConfig {
    fn default() -> Self {
        Self {
            // Conservative sharpening that enhances architectural details
            // without creating halos or noise
                    shadow_recovery: 0.4,          // Stronger shadow recovery
                    interior_shadow_boost: 0.35,    // Stronger interior lighting
                    window_recovery_strength: 0.7,  // Stronger window recovery
                    brightness_adjustment: 8,       // Brighter overall
                    ..Self::default()
        }
    }
}
impl Exteriordefault for ImageEnhancementConfig {
    fn default() -> Self {
        Self {
            // Conservative sharpening that enhances architectural details
            // without creating halos or noise
            exterior_sky_enhancement: 0.25,  // Stronger sky enhancement
            contrast_boost: 1.2,            // Stronger contrast
            shadow_recovery: 0.25,          // Less shadow recovery
            highlight_protection: 0.9,      // Less highlight protection
                    ..Self::default()
        }
    }
}
impl Twilightdefault for ImageEnhancementConfig {
    fn default() -> Self {
        Self {
            shadow_recovery: 0.5,           // Strong shadow recovery
            brightness_adjustment: 12,       // Brighter image
            color_enhancement_strength: 1.15, // Stronger colors
            window_recovery_strength: 0.8,   // Strong window glow
            ..Self::default()
        }
    }
}







use image::{DynamicImage, ImageBuffer, Rgb, imageops};
use imageproc::{geometric_transformations, filter};

impl ImageProcessor {
    fn optimize_image(&self, img: DynamicImage) -> Result<DynamicImage> {
        let (width, height) = img.dimensions();
        
        // 1. Resize if needed (existing code)
        let resized = if width > MAX_WIDTH || height > MAX_HEIGHT {
            let ratio = f64::min(
                MAX_WIDTH as f64 / width as f64,
                MAX_HEIGHT as f64 / height as f64
            );
            let new_width = (width as f64 * ratio) as u32;
            let new_height = (height as f64 * ratio) as u32;
            
            img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
        } else {
            img
        };

        // 2. Auto-enhance image
        let enhanced = self.enhance_real_estate_photo(resized)?;
        
        Ok(enhanced)
    }

    fn enhance_real_estate_photo(&self, img: DynamicImage) -> Result<DynamicImage> {
        // Convert to RGB for processing
        let rgb_img = img.to_rgb8();

        // Auto-level exposure and contrast
        let balanced = self.auto_level_exposure(&rgb_img)?;
        
        // Enhance details while preserving natural look
        let sharpened = self.enhance_details(&balanced)?;

        // Automatic perspective correction
        let corrected = self.correct_perspective(&sharpened)?;

        // Color enhancement
        let color_enhanced = self.enhance_colors(&corrected)?;

        // Convert back to DynamicImage
        Ok(DynamicImage::ImageRgb8(color_enhanced))
    }

    fn auto_level_exposure(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        // Implement automatic exposure correction
        // 1. Calculate histogram
        // 2. Find optimal black and white points
        // 3. Apply contrast stretch
        // 4. Adjust gamma if needed
        
        // Placeholder for actual implementation
        Ok(img.clone())
    }

    fn enhance_details(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        // Controlled sharpening for architectural details
        // 1. Apply unsharp mask with conservative parameters
        // 2. Reduce noise in shadows
        // 3. Enhance edge contrast
        
        // Example parameters (adjust based on testing)
        let sigma = 1.0;
        let threshold = 5;
        
        // Apply unsharp mask
        let sharpened = filter::unsharp_mask(img, sigma, threshold);
        
        Ok(sharpened)
    }

    fn correct_perspective(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        // Automatic perspective correction
        // 1. Detect strong vertical/horizontal lines
        // 2. Calculate correction matrix
        // 3. Apply transform
        
        // Placeholder for actual implementation
        Ok(img.clone())
    }

    fn enhance_colors(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        // Subtle color enhancement
        // 1. Auto white balance
        // 2. Slight vibrance boost
        // 3. Sky detection and enhancement
        // 4. Preserve natural colors for vegetation
        
        // Example implementation (adjust parameters based on testing)
        let enhanced = imageops::colorops::brighten(img, 10);
        let enhanced = imageops::colorops::contrast(&enhanced, 1.1);
        
        Ok(enhanced)
    }
}