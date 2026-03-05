use toy_ac::symbol_model::VectorCountSymbolModel;

pub struct RegionEncodingMap {
    symbol_models: Vec<VectorCountSymbolModel<i32>>, // each region has its own symbol model
    height: u32,                                     // height of the frame
    width: u32,                                      // width of the frame
    resolution: u32,                                 // frame is divided into resolution^2 regions
}

impl RegionEncodingMap {
    pub fn new(height: u32, width: u32, resolution: u32) -> Self {
        let symbol_models = (0..resolution.pow(2))
            .map(|_| VectorCountSymbolModel::new((0..=255).collect()))
            .collect::<Vec<_>>();

        Self {
            symbol_models,
            height,
            width,
            resolution,
        }
    }

    // maps pixels to regions; returns offset into self.symbol_models
    // the symbol model for this pixel is found at
    pub fn get_symbol_model_index(&self, pixel: (u32, u32)) -> usize {
        let (r, c) = pixel;
        if r > self.height || c > self.width {
            panic!("Pixel out of bounds")
        }

        let r_offset = (self.resolution * r) / (self.height + 1);
        let c_offset = (self.resolution * c) / (self.width + 1);
        let i = (self.resolution * r_offset + c_offset) as usize;

        return i;
    }

    // returns the symbol model for the region this pixel is apart of
    pub fn get_symbol_model(&self, pixel: (u32, u32)) -> &VectorCountSymbolModel<i32> {
        let i = self.get_symbol_model_index(pixel);
        return &self.symbol_models[i];
    }

    // increments the count of the symbol model for the region this pixel is apart of
    pub fn incr_count_regional(&mut self, pixel: (u32, u32), s: &i32) {
        let i = self.get_symbol_model_index(pixel);
        self.symbol_models[i].incr_count(s);
    }
}
