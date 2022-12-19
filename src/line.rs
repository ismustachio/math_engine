use crate::prelude::*;

pub struct Line {
    direction: Vector3,
    moment: Vector3,
}

impl Line {
    pub fn new(vx: f32, vy: f32, vz: f32, mx: f32, my: f32, mz: f32) -> Line {
        Line {
            direction: Vector3::new(vx, vy, vz),
            moment: Vector3::new(mx, my, mz),
        }
    }

    pub fn new_with_vecs(direction: Vector3, moment: Vector3) -> Line {
        Line { direction, moment }
    }

    // Transforms this line with a transformation matrix
    pub fn transform(&self, h: &Transform4) -> Line {
        let v1 = h.vec_at(1).cross(&h.vec_at(2));
        let v2 = h.vec_at(2).cross(&h.vec_at(0));
        let v3 = h.vec_at(0).cross(&h.vec_at(1));
        // the transpose of the adjugate of the upper-left 3x3
        // portion of h because of the column-major order
        let adj = Matrix3::new_with_vecs(&v1, &v2, &v3);
        let t = h.get_translation();
        let direction = *h * self.direction;
        // TODO: fix this
        let moment = adj * self.moment + Vector3::from(t).cross(&direction);
        let moment = self.moment;
        Line { direction, moment }
    }

    pub fn transform_mut(&mut self, h: &Transform4) {
        let l = self.transform(h);
        self.direction = l.direction;
        self.moment = l.moment;
    }
}

impl Default for Line {
    fn default() -> Line {
        Line {
            direction: Default::default(),
            moment: Default::default(),
        }
    }
}
