```rust
pub struct Rgb {
pub r: u8, // 0-255
pub g: u8, // 0-255
pub b: u8, // 0-255
}
```

#### HSL (Hue, Saturation, Lightness)
```rust
pub struct Hsl {
pub h: f32, // 0-360
pub s: f32, // 0-100
pub l: f32, // 0-100
}
```

### Key Features

1. **Color Space Conversion**
   - RGB to HSL conversion
   - HSL to RGB conversion
   - Precise color space transformations

2. **White Balance Adjustment**
   - Temperature control (warm/cool)
   - Tint adjustment (green/magenta)
   - Non-destructive adjustments

3. **Image Enhancement Traits**
   - Brightness adjustment
   - Contrast enhancement
   - Saturation control
   - Shadow recovery
   - Highlight protection

## Usage Examples

### Basic Color Adjustments
```rust
rust
let mut color = Rgb { r: 128, g: 128, b: 128 };
// Adjust white balance
color.adjust_white_balance(0.2, 0.0); // Warm up the color
// Enhance image properties
color.adjust_brightness(10.0);
color.adjust_contrast(1.2);
color.adjust_saturation(1.1);
```


### Advanced Processing
```rust
rust
let mut color = Rgb { r: 200, g: 150, b: 100 };
// Shadow and highlight recovery
color.adjust_shadows(15.0); // Lift shadows
color.adjust_highlights(-10.0); // Recover highlights
```

## Implementation Details

- All color adjustments are performed in the HSL color space for better perceptual results
- Adjustments are clamped to valid ranges to prevent overflow
- Efficient conversion algorithms minimize processing overhead
- Thread-safe implementations with `Copy` trait support

## Best Practices

1. Use HSL for perceptual adjustments (brightness, contrast)
2. Apply white balance before other adjustments
3. Make subtle adjustments for natural results
4. Consider batch processing for performance

## Performance Considerations

- In-place modifications for efficiency
- Minimal allocations in color space conversions
- Optimized mathematical operations
- Suitable for real-time processing