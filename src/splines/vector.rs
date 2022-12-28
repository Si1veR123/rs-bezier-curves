use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Vector<const DIM: usize, DType = f64> {
    pub data: [DType; DIM]
}

pub trait VectorLength<const DIM: usize> {
    fn length(&self) -> f64;
}

pub trait VectorNormalise<const DIM: usize>: VectorLength<DIM> {
    fn normalise(&self) -> Option<Vector<DIM>>;
}

// ADD DTYPE TO ALL VALUES
impl<const DIM: usize, DType> Add<DType> for Vector<DIM, DType>
    where DType: From<u8> + Copy + Add<DType, Output = DType>,
    {
    type Output = Self;

    fn add(self, rhs: DType) -> Self::Output {
        let mut new_arr: [DType; DIM] = [0u8.into(); DIM];
        for (i, val) in self.data.iter().enumerate() {
            new_arr[i] = rhs.add(val.clone());
        }
        Self { data: new_arr }
    }
}

// MULTIPLY DTYPE BY ALL VALUES
impl<const DIM: usize, DType> Mul<DType> for Vector<DIM, DType>
    where DType: From<u8> + Copy + Mul<DType, Output = DType>,
    {
    type Output = Self;

    fn mul(self, rhs: DType) -> Self::Output {
        let mut new_arr: [DType; DIM] = [0u8.into(); DIM];
        for (i, val) in self.data.iter().enumerate() {
            new_arr[i] = rhs.mul(val.clone());
        }
        Self { data: new_arr }
    }
}


impl<const DIM: usize, DType> VectorLength<DIM> for Vector<DIM, DType>
    where DType: Mul<DType, Output = DType> + Copy + Into<f64>
    {
    fn length(&self) -> f64 {
        let mut square_sum = 0f64;
        for d in &self.data {
            let d_f64: f64 = d.clone().into();
            square_sum += d_f64*d_f64
        }
        square_sum.sqrt()
    }
}


impl<const DIM: usize, DType> VectorNormalise<DIM> for Vector<DIM, DType> 
    where DType: Mul<DType, Output = DType> + Copy + Into<f64> + PartialOrd
    {
    fn normalise(&self) -> Option<Vector<DIM, f64>> {
        let length = self.length();
        if length == 0.0 {
            return None
        }
        let mut new_arr: [f64; DIM] = [0.0; DIM];
        for (i, old_val) in self.data.iter().cloned().enumerate() {
            let old_val_f: f64 = old_val.into();
            new_arr[i] = (old_val_f/length).into();
        }
        Some( Vector { data: new_arr } )
    }
}


#[allow(dead_code)]
impl<const DIM: usize, DType> Vector<DIM, DType> 
    where DType: From<u8> + Copy + Add<DType, Output = DType> + Mul<DType, Output = DType>
    {
    pub fn add_vector(&self, rhs: &Vector<DIM, DType>) -> Vector<DIM, DType> {
        let mut new_arr: [DType; DIM] = [0u8.into(); DIM];
        for (i, (l_val, r_val)) in self.data.iter().zip(rhs.data.iter()).enumerate() {
            new_arr[i] = l_val.clone() + r_val.clone();
        }
        Vector { data: new_arr }
    }

    pub fn multiply_vector(&self, rhs: &Vector<DIM, DType>) -> Vector<DIM, DType> {
        let mut new_arr: [DType; DIM] = [0u8.into(); DIM];
        for (i, (l_val, r_val)) in self.data.iter().zip(rhs.data.iter()).enumerate() {
            new_arr[i] = l_val.clone() * r_val.clone();
        }
        Vector { data: new_arr }
    }
}
