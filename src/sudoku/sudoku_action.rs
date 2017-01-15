pub use rustc_serialize::json;
use std::fmt;
use sudoku_elem::*;
use std::boxed::Box;
use std::collections::HashMap;
use std::borrow::BorrowMut;
use std::borrow::Borrow;

const Start1 : usize = 1;
const End1: usize = 3;
const Start2: usize = 5;
const End2: usize = 7;

#[derive(Copy,Clone,RustcDecodable,RustcEncodable,Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone,RustcDecodable,RustcEncodable,Debug)]
pub struct Sudoku {
    data :[[SudokuElem;9];9],
    st:  char,
    area_map: Box<HashMap<u8,Vec<Point>>>,
}

impl Sudoku {
    pub fn from_json_new(jsonstr: &str) -> Self {

        #[derive(Copy,Clone,RustcDecodable)]
        struct Intermediate {
            data : [[u8;9];9],
            st: char,
        }

        let intermediate_ : Intermediate = json::decode(jsonstr).unwrap();

        let mut rel = Sudoku{
            data:[[SudokuElem::new(0u8);9];9],
            st: intermediate_.st,
            area_map: Box::new(HashMap::<u8,Vec<Point>>::new()),
        };
        rel.st = intermediate_.st;

        if intermediate_.st != 'A' {
            for i in 0..9 {
                for j in 0..9 {
                    let value = intermediate_.data[i][j];
                    if value == 0 {
                        rel.data[i][j].push_all_to_cache();
                    } else {
                        rel.data[i][j].set_value(value);
                    }
                }
            }
        } else {
            for i in 0..9 {
                for j in 0..9 {
                    let area_no = intermediate_.data[i][j] / 10;
                    rel.data[i][j].set_area(area_no);

                    let value = intermediate_.data[i][j] % 10;
                    if value == 0 {
                        rel.data[i][j].push_all_to_cache();
                    } else {
                        rel.data[i][j].set_value(value);
                    }
                    {
                        let mut_ref: &mut HashMap<u8,Vec<Point>> = rel.area_map.as_mut();
                        if let Some(vec_ref) = mut_ref.get_mut(&area_no) {
                            vec_ref.push(Point{x:i,y:j,});
                        }
                    }
                    {
                        let mut_ref: &mut HashMap<u8,Vec<Point>> = rel.area_map.as_mut();
                        if let None = mut_ref.get_mut(&area_no) {
                            mut_ref.insert(area_no,vec![Point{x:i,y:j}]);
                        }
                    }
                }
            }
        }
        rel
    }

    fn nine_restrict_impl(&mut self, xs :usize, xe:usize, ys:usize, ye: usize, x:usize,y:usize) {
        if x >= xs && x <= xe && y >= ys && y <= ye {
            for i in xs..xe+1 {
                for j in ys..ye+1 {
                    if x == i && y == j {
                        continue;
                    }

                    let _tp = self.data[i][j].get_value();
                    if _tp > 0 {
                        self.data[x][y].remove_from_cache(_tp);
                    }
                }
            }
        }
    }

    fn nine_restrict(&mut self, x:usize,y:usize) -> u8 {
        let _xs = x - x % 3;
        let _ys = y - y % 3;
        self.nine_restrict_impl(_xs,_xs+2,_ys,_ys+2,x,y);
        self.data[x][y].cache_num()
    }

    fn row_restrict(&mut self, x:usize,y:usize) -> u8 {
        self.nine_restrict_impl(x,x,0,8,x,y);
        self.data[x][y].cache_num()
    }

    fn col_restrict(&mut self, x:usize,y:usize) -> u8 {
        self.nine_restrict_impl(0,8,y,y,x,y);
        self.data[x][y].cache_num()
    }

    fn x_restrict(&mut self, x:usize,y:usize) -> u8 {
        if x == y {
            for i in 0..9 {
                if x == i {
                    continue;
                }
                let value = self.data[i][i].get_value();
                self.data[x][y].remove_from_cache(value);
            }
        }

        if x + y == 8 {
            for i in 0..9 {
                if x == i {
                    continue;
                }

                let value = self.data[i][8-i].get_value();
                self.data[x][y].remove_from_cache(value);
            }
        }
        self.data[x][y].cache_num()
    }

    fn percent_restrict(&mut self, x:usize,y:usize) -> u8 {
        self.nine_restrict_impl(Start1,End1,Start1,End1,x,y);
        self.nine_restrict_impl(Start2,End2,Start2,End2,x,y);

        if x + y == 8 {
            for i in 0..9 {
                if x == i {
                    continue;
                }

                let value =  self.data[i][8-i].get_value();
                self.data[x][y].remove_from_cache(value);
            }
        }
        self.data[x][y].cache_num()
    }

    fn super_restrict(&mut self, x:usize,y:usize) -> u8 {
        self.nine_restrict_impl(Start1,End1,Start1,End1,x,y);
        self.nine_restrict_impl(Start2,End2,Start2,End2,x,y);
        self.nine_restrict_impl(Start1,End1,Start2,End2,x,y);
        self.nine_restrict_impl(Start2,End2,Start1,End1,x,y);
        self.data[x][y].cache_num()
    }

    fn color_restrict(&mut self, x:usize,y:usize) -> u8 {
        let (tp_x,tp_y) = (x%3,y%3);
        let mut i = 0usize;
        let mut j = 0usize;

        while i < 9 {
            while j < 9 {
                if i + tp_x == x && j + tp_y == y {
                    continue;
                }

                let value = self.data[i+tp_x][j+tp_y].get_value();
                self.data[x][y].remove_from_cache(value);
                j += 3;
            }
            i += 3;
        }
        self.data[x][y].cache_num()
    }

    fn area_restrict(&mut self, x:usize,y:usize) -> u8 {
        let area_no = self.data[x][y].get_area();
        if let Some(area_points) = self.area_map.as_ref().get(&area_no) {
            for p in area_points.iter() {
                let value = self.data[p.x][p.y].get_value();
                if value > 0 {
                    self.data[x][y].remove_from_cache(value);
                }
            }
        }
        self.data[x][y].cache_num()
    }

    fn get_candidate_num(&self,x:usize,y:usize) -> u8 {
        if self.data[x][y].get_value() > 0 {
            return 1u8;
        }
        self.data[x][y].cache_num()
    }

    pub fn generate_sudoku(&mut self,rel_array : &mut Vec<Self>) -> bool {
        let mut MinX : usize = 0;
        let mut MinY : usize = 0;
        let mut MinC : u8 = 0u8;
        let mut MaxC : u8 = 0u8;
        let mut next : bool = true;
        let mut tpCand : u8 = 0u8;
        while next {
            next = false;
            MinC = 9;
            MaxC = 1;
            for i in 0..9 {
                for j in 0..9 {
                    let candidate_num = self.get_candidate_num(i,j);
                    if candidate_num > 1 {
                        let mut tp_num = self.nine_restrict(i,j);
                        if tp_num > 0 {
                            tp_num = self.row_restrict(i,j);
                        }
                        if tp_num > 0 {
                            tp_num = self.col_restrict(i,j);
                        }

                        if tp_num > 0 && self.st == 'X' {
                            tp_num = self.x_restrict(i,j);
                        }
                        if tp_num > 0 && self.st == 'U' {
                            tp_num = self.super_restrict(i,j);
                        }
                        if tp_num > 0 && self.st == 'P' {
                            tp_num = self.percent_restrict(i,j);
                        }
                        if tp_num > 0 && self.st == 'C' {
                            tp_num = self.color_restrict(i,j);
                        }

                        if tp_num > 0 && self.st == 'A' {
                            tp_num = self.area_restrict(i,j);
                        }

                        if tp_num == 0 {
                            return false;
                        }

                        if tp_num == 1 {
                            let _tp = self.data[i][j].pop_cache_front();
                            self.data[i][j].set_value(_tp);
                        }

                        tpCand = self.get_candidate_num(i,j);
                        if tpCand < candidate_num {
                            next = true;
                        }

                        if tpCand < MinC && tpCand > 1 {
                            MinC = tpCand;
                            MinX = i;
                            MinY = j;
                        }

                        if tpCand > MaxC {
                            MaxC = tpCand;
                        }
                    }
                }
            }
        }
        
        if MaxC > 1 {
            while self.data[MinX][MinY].cache_num() > 0 {
                let mut tp_sudoku = self.clone();
                let value = tp_sudoku.data[MinX][MinY].pop_cache_front();
                tp_sudoku.data[MinX][MinY].set_value(value);
                tp_sudoku.data[MinX][MinY].remove_all_cache();
                tp_sudoku.generate_sudoku(rel_array);
                self.data[MinX][MinY].pop_cache_front();
            }
        } else if MaxC == 1 && MinC == 9 {
            //println!("{}",*self);
            rel_array.push(self.clone());
            return true
        }
        return false
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut rsl = "\n".to_string();
        for i in 0..9 {
            let mut row : String = "".to_string();
            for j in 0..9 {
                row += &format!("{},",self.data[i][j].get_value());
            }  
            rsl += &row;
            rsl += "\n";
        }
        write!(f,"{}",rsl)
    }
}


