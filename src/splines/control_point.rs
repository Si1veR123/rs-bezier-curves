use super::vector::Vector;

pub trait ControlPoint<const DIM: usize> {
    fn get_control_point_position(&self) -> Vector<DIM>;
    fn get_first_handle(&self) -> Vector<DIM>;
    fn get_second_handle(&self) -> Vector<DIM>;
}


pub struct CustomControlPoint<const DIM: usize> {
    position: Vector<DIM>,
    first_handle: Vector<DIM>,
    second_handle: Vector<DIM>
}

impl<const DIM: usize> ControlPoint<DIM> for CustomControlPoint<DIM> {
    fn get_control_point_position(&self) -> Vector<DIM> {
        self.position.clone()
    }
    fn get_first_handle(&self) -> Vector<DIM> {
        self.first_handle.clone()
    }
    fn get_second_handle(&self) -> Vector<DIM> {
        self.second_handle.clone()
    }
}

impl<const DIM: usize> CustomControlPoint<DIM> {
    pub fn new(position: Vector<DIM>, first_handle: Vector<DIM>, second_handle: Vector<DIM>) -> Self {
        Self { position, first_handle, second_handle }
    }
}


pub struct AlignedControlPoint<const DIM: usize> {
    position: Vector<DIM>,
    first_handle_unit_vector: Vector<DIM>,
    first_handle_length: f64,
    second_handle_length: f64,
}

impl<const DIM: usize> ControlPoint<DIM> for AlignedControlPoint<DIM> {
    fn get_control_point_position(&self) -> Vector<DIM> {
        self.position.clone()
    }
    fn get_first_handle(&self) -> Vector<DIM> {
        let relative = self.first_handle_unit_vector.clone() * self.first_handle_length;
        relative.add_vector(&self.position)
    }
    fn get_second_handle(&self) -> Vector<DIM> {
        let relative = self.first_handle_unit_vector.clone() * self.second_handle_length * -1.0;
        relative.add_vector(&self.position)
    }
}

impl<const DIM: usize> AlignedControlPoint<DIM> {
    pub fn new(position: Vector<DIM>, first_handle_unit_vector: Vector<DIM>, first_handle_length: f64, second_handle_length: f64) -> Self {
        Self { position, first_handle_unit_vector, first_handle_length, second_handle_length }
    }
}


pub struct MirroredControlPoint<const DIM: usize> {
    position: Vector<DIM>,
    first_handle_vector: Vector<DIM>
}

impl<const DIM: usize> ControlPoint<DIM> for MirroredControlPoint<DIM> {
    fn get_control_point_position(&self) -> Vector<DIM> {
        self.position.clone()
    }
    fn get_first_handle(&self) -> Vector<DIM> {
        self.first_handle_vector.add_vector(&self.position)
    }
    fn get_second_handle(&self) -> Vector<DIM> {
        (self.first_handle_vector.clone()*-1.0).add_vector(&self.position)
    }
}

impl<const DIM: usize> MirroredControlPoint<DIM> {
    pub fn new(position: Vector<DIM>, first_handle_vector: Vector<DIM>) -> Self {
        Self { position, first_handle_vector }
    }
}
