# Image Processor Module

Core image processing engine for the Neural Reef Registration System, specialized for real estate photography in Southeast Asian and Middle Eastern markets.

## System Integration

### Module Dependencies
```
processor.rs
├── color.rs
├── histogram.rs
├── image_utils.rs
├── quality_report.rs
└── batch.rs
```

### Service Chain
```
listing.rs → processor.rs → image_service.rs → batch_analysis_service.rs
```

### Validation Chain
```
request_validation.rs → image_validation.rs → id_validation.rs → processor.rs
```

## Core Components

### ID Management
- Uses `id_types.rs` for:
  - `ListingId` (FL_prefix)
  - `BatchId` (FB_prefix)
  - `ImageId` (FI_prefix)

### Quality Control
- Integrates with `quality_report.rs`
- Validates through `image_validation.rs`
- Reports to `batch_analysis_service.rs`

### Authentication
- Secured via `key_logic_auth/auth.rs`
- API keys managed by `key_service.rs`
- Endpoints protected in `api/key.rs`

## Processing Pipeline

### 1. Input Validation
```rust
// Validate dimensions (1080p-4K)
let img = image::load_from_memory(&image_data)?;
let (width, height) = img.dimensions();
if width < 1920 || height < 1080 || width > 3840 || height > 2160 {
    return Err(AppError::InvalidInput("..."));
}
```

### 2. Enhancement Chain
1. Color correction (`color.rs`)
2. Histogram analysis (`histogram.rs`)
3. Architectural detection (`image_utils.rs`)
4. Quality assessment (`quality_report.rs`)

### 3. Database Integration
- Schema defined in `schema.surql`
- Storage handled by `image_service.rs`
- Batch processing via `batch.rs`

## Feature Modules

### Search Capabilities
- Endpoint: `api/search.rs`
- Integration: `f_ai_database/image_service.rs`
- Validation: `request_validation.rs`

### Batch Processing
- Service: `batch_analysis_service.rs`
- Core: `f_ai_core/batch.rs`
- Metrics: `quality_report.rs`

### Email Notifications
- Service: `email_service.rs`
- Triggers: Quality issues, batch completion
- Templates: Processing reports

## Market-Specific Processing

### Configuration Chain
```
listing.rs → processor.rs → image_validation.rs → quality_report.rs
```

### Regional Settings
- Thailand: `ThailandDetails`
- UAE: `UAEDetails`
- Vietnam: `VietnamDetails`
- Malaysia: `MalaysiaDetails`
- Cambodia: `CambodiaDetails`

## Error Handling

### Validation Layer
```rust
pub enum ValidationError {
    RequestError(String),
    ImageError(String),
    IdError(String),
    AuthError(String)
}
```

### Processing Layer
```rust
pub enum ProcessingError {
    QualityError(String),
    EnhancementError(String),
    BatchError(String),
    StorageError(String)
}
```

## Performance Optimization

### Caching Strategy
1. Input validation results
2. Intermediate processing results
3. Quality analysis metrics
4. Batch processing status

### Resource Management
1. Memory-efficient processing
2. Parallel batch execution
3. Async I/O operations
4. Connection pooling

## API Integration

### Endpoints
- POST `/api/listing/{id}/images`
- POST `/api/batch/process`
- GET `/api/quality/report`
- GET `/api/search/images`

### Authentication
```rust
async fn validate_api_key(key: &str) -> Result<()> {
    key_service::validate(key).await
}
```

## Monitoring & Metrics

### Quality Metrics
- Processing success rate
- Image quality scores
- Enhancement effectiveness
- Regional compliance

### Performance Metrics
- Processing duration
- Memory utilization
- Cache hit rates
- Error frequencies

## Future Roadmap

### 1. AI Integration
- Style transfer models
- Automated staging
- Defect detection
- Quality prediction

### 2. Performance
- GPU acceleration
- Distributed processing
- Advanced caching
- Memory optimization

### 3. Features
- HDR processing
- Focus stacking
- Panorama stitching
- Virtual staging

## Documentation Index
- [Color Processing](README_color.md)
- [Image Utils](README_image_utils.md)
- [Histogram Analysis](README_histogram.md)
- [Quality Reports](quality_report.rs)

## Image Enhancement Capabilities

### Color Corrections
1. White Balance
   - Temperature adjustment (warm/cool)
   - Tint correction (green/magenta)
   - Auto white balance for mixed lighting
   - Regional light temperature compensation

2. Color Management
   - Color cast removal
   - Saturation optimization
   - Color space conversion
   - Regional color preferences

### Exposure Corrections
1. Global Adjustments
   - Brightness normalization
   - Contrast enhancement
   - Dynamic range optimization
   - Exposure compensation

2. Local Adjustments
   - Shadow recovery
   - Highlight protection
   - Window exposure balancing
   - Local contrast enhancement

### Architectural Corrections
1. Perspective
   - Vertical line alignment
   - Horizontal level correction
   - Keystone distortion removal
   - Lens distortion compensation

2. Spatial Features
   - Room depth enhancement
   - Ceiling/floor alignment
   - Wall convergence correction
   - Corner alignment

### Detail Enhancement
1. Sharpening
   - Edge-aware sharpening
   - Detail recovery
   - Texture preservation
   - Unsharp masking

2. Noise Management
   - Luminance noise reduction
   - Color noise suppression
   - Pattern noise removal
   - Detail-preserving smoothing

### Scene-Specific Optimizations
1. Interior Scenes
   - Mixed lighting correction
   - Window exposure balancing
   - Room depth enhancement
   - Interior reflection management

2. Exterior Scenes
   - Sky enhancement
   - Facade detail preservation
   - Vegetation optimization
   - Shadow/highlight balance

3. Twilight Scenes
   - Light source management
   - Long exposure simulation
   - Window glow optimization
   - Ambient light enhancement

### Regional Adaptations
1. Thailand Properties
   - Tropical light compensation
   - Pool reflection handling
   - Temple view enhancement
   - Tropical vegetation optimization

2. UAE Properties
   - Desert haze removal
   - Glass facade optimization
   - Harsh sunlight compensation
   - Sand color calibration

3. Vietnam Properties
   - Urban density optimization
   - Heritage detail preservation
   - Street view enhancement
   - Tropical storm light handling

### Quality Improvements
1. Technical Quality
   - Resolution optimization
   - Compression artifact removal
   - Moire pattern reduction
   - Chromatic aberration correction

2. Aesthetic Quality
   - Composition enhancement
   - Visual balance optimization
   - Leading line enhancement
   - Depth perception improvement

### Special Features
1. HDR-Like Effects
   - Dynamic range expansion
   - Tone mapping
   - Local exposure optimization
   - Light source balancing

2. Focus Enhancement
   - Depth of field optimization
   - Focus stacking simulation
   - Selective sharpening
   - Motion blur removal

3. Advanced Corrections
   - Lens flare reduction
   - Reflection management
   - Glass surface optimization
   - Weather condition compensation

### Automated Analysis
1. Quality Detection
   - Blur detection
   - Noise level assessment
   - Exposure evaluation
   - Color cast detection

2. Feature Detection
   - Window detection
   - Architectural line detection
   - Room type classification
   - Light source identification

### Output Optimization
1. Format Optimization
   - WebP conversion
   - Quality-size balancing
   - Progressive loading support
   - Metadata preservation

2. Delivery Optimization
   - Resolution variants
   - Progressive enhancement
   - Thumbnail generation
   - Preview optimization

## Implementation Details

### Enhancement Implementation Map

1. Color Processing Chain
```rust
fn enhance_color(&self, img: &mut DynamicImage) -> Result<()> {
    // White Balance
    self.correct_white_balance(img)?;
    
    // Color Management
    self.remove_color_cast(img)?;
    self.optimize_saturation(img)?;
    
    // Regional Adaptations
    match self.region {
        Region::Thailand => self.apply_tropical_compensation(img)?,
        Region::UAE => self.apply_desert_compensation(img)?,
        // ...
    }
    
    Ok(())
}
```

2. Exposure Enhancement Chain
```rust
fn enhance_exposure(&self, img: &mut DynamicImage) -> Result<()> {
    // Global Adjustments
    self.normalize_brightness(img)?;
    self.enhance_contrast(img)?;
    
    // Local Adjustments
    if self.has_windows {
        self.balance_window_exposure(img)?;
    }
    
    self.recover_shadows(img)?;
    self.protect_highlights(img)?;
    
    Ok(())
}
```

3. Architectural Enhancement Chain
```rust
fn enhance_architecture(&self, img: &mut DynamicImage) -> Result<()> {
    // Detect and correct lines
    let lines = detect_architectural_lines(img)?;
    if needs_perspective_correction(&lines) {
        self.correct_perspective(img)?;
    }
    
    // Enhance spatial features
    self.enhance_room_depth(img)?;
    self.align_verticals(img)?;
    
    Ok(())
}
```

4. Scene-Specific Processing
```rust
fn apply_scene_optimizations(&self, img: &mut DynamicImage) -> Result<()> {
    match self.scene_type {
        SceneType::Interior => {
            self.balance_mixed_lighting(img)?;
            self.enhance_room_depth(img)?;
        },
        SceneType::Exterior => {
            self.enhance_sky(img)?;
            self.preserve_facade_details(img)?;
        },
        SceneType::Twilight => {
            self.optimize_window_glow(img)?;
            self.enhance_ambient_light(img)?;
        }
    }
    Ok(())
}
```

5. Quality Enhancement Chain
```rust
fn enhance_quality(&self, img: &mut DynamicImage) -> Result<()> {
    // Technical improvements
    self.remove_noise(img)?;
    self.sharpen_details(img)?;
    self.correct_chromatic_aberration(img)?;
    
    // Aesthetic improvements
    self.optimize_composition(img)?;
    self.enhance_depth_perception(img)?;
    
    Ok(())
}
```

### Processing Parameters

```rust
pub struct EnhancementParams {
    // Color Correction
    white_balance_temp: f32,     // -100 to +100
    white_balance_tint: f32,     // -100 to +100
    saturation: f32,             // 0.0 to 2.0
    
    // Exposure
    exposure: f32,               // -2.0 to +2.0
    contrast: f32,               // 0.0 to 100.0
    highlights: f32,             // -100 to +100
    shadows: f32,                // -100 to +100
    
    // Detail
    sharpness: f32,             // 0.0 to 100.0
    noise_reduction: f32,        // 0.0 to 100.0
    
    // Scene-Specific
    window_protection: f32,      // 0.0 to 1.0
    sky_enhancement: f32,        // 0.0 to 1.0
    architecture_strength: f32,   // 0.0 to 1.0
}
```
