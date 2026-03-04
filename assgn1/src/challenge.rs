use toy_ac::symbol_model::VectorCountSymbolModel;
use rangemap::RangeMap;

pub struct RegionEncodingMap<T: std::cmp::Eq>{
    map: RangeMap<u32, VectorCountSymbolModel<T>>,
    height: u32,
    width:u32,
    subdivisions: u32
}

impl<T: std::cmp::Eq> RegionEncodingMap<T> {
    pub fn get_symbol_model(&self, key: u32) -> &VectorCountSymbolModel<T> {
        return self.map.get(&key).unwrap();
    }
}
