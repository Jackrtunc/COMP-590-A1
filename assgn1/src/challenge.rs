use toy_ac::symbol_model::VectorCountSymbolModel;

pub struct RegionEncodingMap {
    symbol_models: Vec<VectorCountSymbolModel<u32>>,
    height: u32,
    width: u32,
    resolution: u32,
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
            resolution
        }
    }
    pub fn get_symbol_model(&self, pixel: (u32, u32)) -> &VectorCountSymbolModel<u32> {
        let (r, c) = pixel;

        let r_offset = (self.resolution * r) / (self.height + 1);
        let c_offset = (self.resolution * c) / (self.width + 1);
        let i = (self.resolution * r_offset + c_offset) as usize;

        return &self.symbol_models[i];
    }
}
