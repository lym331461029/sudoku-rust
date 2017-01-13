use rustc_serialize::*;
use std::fmt;

pub trait SudokuElemTrait {
    fn set_value(&mut self,value: u8);
    fn get_value(&self) -> u8;
    fn cache_num(&self) -> u8;
    fn push_cache_back(&mut self,num: u8);
    fn remove_from_cache(&mut self,num: u8);
    fn pop_cache_front(&mut self) -> u8; 
    fn remove_all_cache(&mut self);
    fn push_all_to_cache(&mut self);
    
    fn set_area(&mut self, area: u8);
    fn get_area(&self) -> u8;
}

#[derive(Debug,Copy,Clone)]
pub struct SudokuElem {
    internal_: u16,
    area_:     u8,
}

impl fmt::Display for SudokuElem {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Value {},Area {},CacheNum {}",self.get_value(), self.get_area(),self.cache_num())
    }
}

impl SudokuElemTrait for SudokuElem {
    fn set_value(&mut self,value: u8) {
        let _tp : u16 = (value as u16) << 12;
        self.internal_ |= _tp
    }
    
    fn get_value(&self) -> u8 {
        let _tp = self.internal_ >> 12;
        _tp as u8
    }
    
    fn cache_num(&self) -> u8 {
        let mut _ret : u8 = 0u8;
        for i in 0..9 {
            _ret += (self.internal_ >> i) as u8 & 0x01;
        }
        _ret
    }

    fn push_cache_back(&mut self,num: u8) {
        self.internal_ |= 0x01u16 << (num-1); 
    }

    fn remove_from_cache(&mut self,num: u8){
        let _tp : u16 = !(0x01u16 << (num-1));
        self.internal_ &= _tp;
    }
    
    fn pop_cache_front(&mut self) -> u8 {
        for num in 1..10 {
            if (0x01u16 << (num -1)) & self.internal_ > 0 {
                self.internal_ &= !(0x01u16 << (num - 1));
                return num as u8;
            }
        }
        0u8
    }
    
    fn remove_all_cache(&mut self) {
        self.internal_ &= !511u16;
    }
    
    fn push_all_to_cache(&mut self) {
        self.internal_ |= 511u16
    }
    
    fn set_area(&mut self, area: u8) {
        self.area_ = area;
    }
    
    fn get_area(&self) -> u8 {
        self.area_
    }
}

impl Encodable for SudokuElem {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_u8(self.get_value())
    }
}

impl Decodable for SudokuElem {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match d.read_u8() {
            Ok(value) => Ok(SudokuElem::new(value)),
            Err(e) => Err(e),
        }
    }
}


impl SudokuElem {
    pub fn new(value: u8) -> Self {
        let mut e = SudokuElem{internal_:0u16,area_:0u8};
        e.set_value(value);

        if value == 0 {
            e.push_all_to_cache();
        }
        e
    }
    
    pub fn new_with_area(value:u8,area: u8) -> Self {
        let mut e = SudokuElem{internal_:0u16,area_:0u8};
        e.set_value(value);
        e.set_area(area);
        if value == 0 {
            e.push_all_to_cache();
        }
        e
    }
}

