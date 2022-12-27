use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Vector<const DIM: usize, DType = f64> {
    pub data: [DType; DIM]
}

// ADD NTYPE TO ALL VALUES
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

// MULTIPLY NTYPE BY ALL VALUES
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
