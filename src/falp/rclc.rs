use falp;
use instance;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct RestrictedCandidateListCardinal {
    rcl: HashMap<instance::InstanceFace, u32>,
    worst: Option<(instance::InstanceFace, u32)>,
    size: u32,
}

impl RestrictedCandidateListCardinal {
    fn new(cl_len: u32, alpha: &super::Alpha) -> RestrictedCandidateListCardinal {
        let min: u32 = 1;
        let size = min.max((alpha.get() * cl_len as f64) as u32);

        RestrictedCandidateListCardinal {
            rcl: HashMap::new(),
            worst: None,
            size,
        }
    }

    pub fn take(
        l_star: &HashMap<instance::InstanceFace, u32>,
        config: &falp::Config,
    ) -> instance::InstanceFace {
        let mut rcl =
            falp::rclc::RestrictedCandidateListCardinal::new(l_star.len() as u32, &config.alpha);
        for (instance_face, degree) in l_star.iter() {
            rcl.try_insert(instance_face, *degree);
        }
        rcl.get()
    }

    fn update_worst(&mut self) {
        let mut w: Option<(instance::InstanceFace, u32)> = None;

        for (face, degree) in self.rcl.iter() {
            let nw = Some((*face, *degree));
            match w {
                None => w = nw,
                Some((old_face, old_degree)) => {
                    if *degree > old_degree {
                        // eh definitivamente o pior
                        w = nw;
                    } else if old_degree == *degree {
                        // o de menor index eh o pior
                        if face.index < old_face.index {
                            w = nw;
                        } else if face.index == old_face.index {
                            // se somos do mesmo index
                            // o de menor face eh o pior
                            if face.face < old_face.face {
                                w = nw;
                            }
                        }
                    }
                }
            }
        }
        self.worst = w;
    }
    fn try_insert(&mut self, face: &instance::InstanceFace, degree: u32) {
        if self.rcl.len() == self.size as usize {
            let (old_face, old_degree) = self.worst.unwrap();

            let mut update = || {
                self.rcl.remove(&old_face);
                self.rcl.insert(*face, degree);
                self.update_worst();
            };
            // se eu sou melhor que o pior, entro
            if degree < old_degree {
                update();
            } else if degree == old_degree {
                // se somos iguais
                // dar preferência pro de maior index
                if old_face.index < face.index {
                    update();
                } else if old_face.index == face.index {
                    // se temos o mesmo index
                    // dar preferencia pros de maior face
                    if old_face.face < face.face {
                        update();
                    }
                }
            }
        } else {
            // ainda tem espaço
            self.rcl.insert(*face, degree);
            self.update_worst();
        }
    }

    pub fn get(&self) -> instance::InstanceFace {
        let max = (self.size - 1) as usize;
        let nth: usize = {
            match max {
                0 => 0,
                _ => thread_rng().gen_range(0, max),
            }
        };

        *self.rcl.iter().nth(nth).unwrap().0
    }
}
