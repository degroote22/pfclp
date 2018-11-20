#[cfg(test)]
use instance::*;

#[cfg(test)]
pub fn generate_simple_instance() -> ParsedInstance {
    let point_a_face_0 = vec![InstanceFace::new(1, 0)];
    let point_a_face_1 = vec![InstanceFace::new(1, 1)];
    let point_a = vec![point_a_face_0, point_a_face_1];

    let point_b_face_0 = vec![InstanceFace::new(0, 0)];
    let point_b_face_1 = vec![InstanceFace::new(0, 1)];
    let point_b = vec![point_b_face_0, point_b_face_1];

    let instance = ParsedInstance::new(4, 2, vec![point_a, point_b]);

    instance
}
