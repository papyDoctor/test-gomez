use std::collections::{HashMap, HashSet};

use crate::math::*;
use gomez::nalgebra::{Dyn, IsContiguous};
use gomez::{Domain, Problem, SolverDriver, System};

pub struct Eq2DConstraints<'a> {
    lut: Vec<(VertexId, f64)>,
    inv_lut: HashMap<VertexId, usize>,
    bindings_pool: &'a BindingsPool,
}

impl<'a> Eq2DConstraints<'a> {
    pub fn new(bindings_pool: &'a mut BindingsPool, v_pool: &VerticesPool) -> Eq2DConstraints<'a> {
        // Store the two values of each vertex from the bindings_pool
        // linearly on a vec for the solving, along with the vertex id for the bindings
        let mut lut = vec![];
        let mut inv_lut = HashMap::new();
        {
            // Get all vertices ids that are binded, NO DUPLICATE
            let mut v_ids = HashSet::new();
            bindings_pool
                .values()
                .for_each(|bind| bind.get_v_ids(&mut v_ids));

            v_ids.iter().for_each(|v_id| {
                lut.push((*v_id, v_pool[v_id].pt.x));
                lut.push((*v_id, v_pool[v_id].pt.y));
            });

            println!("lut: {:?}", lut);
        }

        lut.iter()
            .enumerate()
            .step_by(2)
            .for_each(|(idx, (v_id, _))| _ = inv_lut.insert(*v_id, idx));

        println!("inv_lut: {:?}", inv_lut);

        Eq2DConstraints {
            lut,
            inv_lut,
            bindings_pool,
        }
    }

    pub fn solve(&mut self, v_pool: &mut VerticesPool) -> Result<(), String> {
        let mut init = vec![];
        for (_, value) in self.lut.iter() {
            init.push(*value);
        }
        println!("init: {:?}", init);
        let mut solver = SolverDriver::builder(self).with_initial(init).build();
        let tolerance = 1e-6;
        let (vals, norm) = solver
            .find(|state| {
                println!(
                    "iter = {}\t||r(x)|| = {}\tx = {:?}",
                    state.iter(),
                    state.norm(),
                    state.x()
                );
                // println!("iter = {}", state.iter(),);
                state.norm() <= tolerance || state.iter() >= 100
            })
            .map_err(|error| format!("{error}"))?;

        println!("vals: {:?} ", vals);

        self.inv_lut.iter().for_each(|(v_id, idx)| {
            let v = v_pool.get_mut(v_id).unwrap();
            v.pt.x = vals[*idx];
            v.pt.y = vals[*idx + 1];
        });

        if norm <= tolerance {
            Ok(())
        } else {
            Err("did not converge".to_string())
        }
    }
}

impl<'a> Problem for Eq2DConstraints<'a> {
    type Field = f64;
    fn domain(&self) -> Domain<Self::Field> {
        Domain::unconstrained(self.lut.len())
    }
}

impl<'a> System for Eq2DConstraints<'a> {
    fn eval<Sx, Srx>(
        &self,
        x: &gomez::nalgebra::Vector<Self::Field, Dyn, Sx>,
        rx: &mut gomez::nalgebra::Vector<Self::Field, Dyn, Srx>,
    ) where
        Sx: gomez::nalgebra::storage::Storage<Self::Field, Dyn> + IsContiguous,
        Srx: gomez::nalgebra::storage::StorageMut<Self::Field, Dyn>,
    {
        let mut idx_rx = 0;
        self.bindings_pool.iter().for_each(|(_, bind)| match bind {
            Binding::Fixed(b) => {
                let bind = b.bind(&[x[self.inv_lut[&b.v_id]], x[self.inv_lut[&b.v_id] + 1]]);
                rx[idx_rx] = bind[0];
                idx_rx += 1;
                rx[idx_rx] = bind[1];
                idx_rx += 1;
            }
            Binding::FixedX(b) => {
                let bind = b.bind(&[x[self.inv_lut[&b.v_id]], x[self.inv_lut[&b.v_id] + 1]]);
                rx[idx_rx] = bind;
                idx_rx += 1;
            }
            Binding::FixedY(b) => {
                let bind = b.bind(&[x[self.inv_lut[&b.v_id]], x[self.inv_lut[&b.v_id] + 1]]);
                rx[idx_rx] = bind;
                idx_rx += 1;
            }
            Binding::Vertical(b) => {
                rx[idx_rx] = b.bind(&[
                    x[self.inv_lut[&b.va_id]],
                    x[self.inv_lut[&b.va_id] + 1],
                    x[self.inv_lut[&b.vb_id]],
                    x[self.inv_lut[&b.vb_id] + 1],
                ]);
                idx_rx += 1;
            }
            Binding::Horizontal(b) => {
                rx[idx_rx] = b.bind(&[
                    x[self.inv_lut[&b.va_id]],
                    x[self.inv_lut[&b.va_id] + 1],
                    x[self.inv_lut[&b.vb_id]],
                    x[self.inv_lut[&b.vb_id] + 1],
                ]);
                idx_rx += 1;
            }
            Binding::Parallel(b) => {
                rx[idx_rx] = b.bind(&[
                    x[self.inv_lut[&b.l1va_id]],
                    x[self.inv_lut[&b.l1va_id] + 1],
                    x[self.inv_lut[&b.l1vb_id]],
                    x[self.inv_lut[&b.l1vb_id] + 1],
                    x[self.inv_lut[&b.l2va_id]],
                    x[self.inv_lut[&b.l2va_id] + 1],
                    x[self.inv_lut[&b.l2vb_id]],
                    x[self.inv_lut[&b.l2vb_id] + 1],
                ]);
                idx_rx += 1;
            }
            Binding::Error(_) => (),
        });
    }
}
