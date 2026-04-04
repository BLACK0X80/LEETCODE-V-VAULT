use std::collections::BTreeMap;

struct MKAverage {
    black_m: usize,
    black_k: usize,
    black_sz: usize,
    black_pos: usize,
    black_sum: i64,
    black_buf: Vec<i32>,
    black_lo: BTreeMap<i32, usize>,
    black_mid: BTreeMap<i32, usize>,
    black_hi: BTreeMap<i32, usize>,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        let (m, k) = (m as usize, k as usize);
        MKAverage {
            black_m: m, black_k: k, black_sz: m - 2 * k,
            black_pos: 0, black_sum: 0,
            black_buf: vec![0; m],
            black_lo: BTreeMap::new(),
            black_mid: BTreeMap::new(),
            black_hi: BTreeMap::new(),
        }
    }

    fn black_erase(map: &mut BTreeMap<i32, usize>, v: i32) {
        let c = map.get_mut(&v).unwrap();
        if *c == 1 { map.remove(&v); } else { *c -= 1; }
    }

    fn black_insert(map: &mut BTreeMap<i32, usize>, v: i32) {
        *map.entry(v).or_insert(0) += 1;
    }

    fn black_max(map: &BTreeMap<i32, usize>) -> i32 { *map.keys().next_back().unwrap() }
    fn black_min(map: &BTreeMap<i32, usize>) -> i32 { *map.keys().next().unwrap() }

    fn black_remove(&mut self, n: i32) {
        let lo_max = Self::black_max(&self.black_lo);
        let mid_max = if self.black_mid.is_empty() { i32::MIN } else { Self::black_max(&self.black_mid) };

        if n <= lo_max {
            Self::black_erase(&mut self.black_lo, n);
        } else if n <= mid_max {
            Self::black_erase(&mut self.black_mid, n);
            self.black_sum -= n as i64;
        } else {
            Self::black_erase(&mut self.black_hi, n);
        }

        if self.black_lo.values().sum::<usize>() < self.black_k && !self.black_mid.is_empty() {
            let v = Self::black_min(&self.black_mid);
            Self::black_erase(&mut self.black_mid, v);
            self.black_sum -= v as i64;
            Self::black_insert(&mut self.black_lo, v);
        }

        if self.black_mid.values().sum::<usize>() < self.black_sz && !self.black_hi.is_empty() {
            let v = Self::black_min(&self.black_hi);
            Self::black_erase(&mut self.black_hi, v);
            Self::black_insert(&mut self.black_mid, v);
            self.black_sum += v as i64;
        }
    }

    fn black_add(&mut self, n: i32) {
        Self::black_insert(&mut self.black_lo, n);

        if self.black_lo.values().sum::<usize>() > self.black_k {
            let v = Self::black_max(&self.black_lo);
            Self::black_erase(&mut self.black_lo, v);
            Self::black_insert(&mut self.black_mid, v);
            self.black_sum += v as i64;
        }

        if self.black_mid.values().sum::<usize>() > self.black_sz {
            let v = Self::black_max(&self.black_mid);
            Self::black_erase(&mut self.black_mid, v);
            self.black_sum -= v as i64;
            Self::black_insert(&mut self.black_hi, v);
        }
    }

    fn add_element(&mut self, num: i32) {
        if self.black_pos >= self.black_m {
            let old = self.black_buf[self.black_pos % self.black_m];
            self.black_remove(old);
        }
        self.black_add(num);
        self.black_buf[self.black_pos % self.black_m] = num;
        self.black_pos += 1;
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.black_pos < self.black_m { return -1; }
        (self.black_sum / self.black_sz as i64) as i32
    }
}
