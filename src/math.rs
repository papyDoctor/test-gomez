use kurbo::Point;
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Binding {
    Fixed(BindFixed),
    FixedX(BindFixedX),
    FixedY(BindFixedY),
    Vertical(BindVertical),
    Horizontal(BindHorizontal),
    Parallel(BindParallel),
    Distance(BindDistance),
    Error(BindError),
}
#[allow(dead_code)]
impl Binding {
    pub fn get_id(&self) -> BindingId {
        match self {
            Binding::Fixed(b) => b.id,
            Binding::FixedX(b) => b.id,
            Binding::FixedY(b) => b.id,
            Binding::Vertical(b) => b.id,
            Binding::Horizontal(b) => b.id,
            Binding::Parallel(b) => b.id,
            Binding::Distance(b) => b.id,
            Binding::Error(b) => b.id,
        }
    }
    pub fn get_v_ids(&self, v_ids: &mut HashSet<VertexId>) {
        match self {
            Binding::Fixed(b) => {
                v_ids.insert(b.v_id);
            }
            Binding::FixedX(b) => {
                v_ids.insert(b.v_id);
            }
            Binding::FixedY(b) => {
                v_ids.insert(b.v_id);
            }
            Binding::Vertical(b) => {
                v_ids.insert(b.va_id);
                v_ids.insert(b.vb_id);
            }
            Binding::Horizontal(b) => {
                v_ids.insert(b.va_id);
                v_ids.insert(b.vb_id);
            }
            Binding::Parallel(b) => {
                v_ids.insert(b.l1va_id);
                v_ids.insert(b.l1vb_id);
                v_ids.insert(b.l2va_id);
                v_ids.insert(b.l2vb_id);
            }
            Binding::Distance(b) => {
                v_ids.insert(b.va_id);
                v_ids.insert(b.vb_id);
            }
            Binding::Error(_) => (),
        };
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct BindError {
    pub id: BindingId,
}
#[allow(dead_code)]
impl BindError {
    pub fn new() -> Binding {
        return Binding::Error(BindError {
            id: BindingId::new_id(),
        });
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindFixed {
    pub id: BindingId,
    pub fixed_value: Point,
    pub v_id: VertexId,
}
impl BindFixed {
    pub fn bind(&self, vals: &[f64; 2]) -> [f64; 2] {
        [vals[0] - self.fixed_value.x, vals[1] - self.fixed_value.y]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindFixedX {
    pub id: BindingId,
    pub fixed_value: f64,
    pub v_id: VertexId,
}
impl BindFixedX {
    pub fn bind(&self, vals: &[f64; 2]) -> f64 {
        vals[0] - self.fixed_value
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindFixedY {
    pub id: BindingId,
    pub fixed_value: f64,
    pub v_id: VertexId,
}
impl BindFixedY {
    pub fn bind(&self, vals: &[f64; 2]) -> f64 {
        vals[1] - self.fixed_value
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindVertical {
    pub id: BindingId,
    pub va_id: VertexId,
    pub vb_id: VertexId,
}
impl BindVertical {
    pub fn bind(&self, vals: &[f64; 4]) -> f64 {
        vals[0] - vals[2]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindHorizontal {
    pub id: BindingId,
    pub va_id: VertexId,
    pub vb_id: VertexId,
}
impl BindHorizontal {
    pub fn bind(&self, vals: &[f64; 4]) -> f64 {
        vals[1] - vals[3]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindParallel {
    pub id: BindingId,
    pub l1va_id: VertexId,
    pub l1vb_id: VertexId,
    pub l2va_id: VertexId,
    pub l2vb_id: VertexId,
}
impl BindParallel {
    pub fn bind(&self, vals: &[f64; 8]) -> f64 {
        (vals[6] - vals[4]) * (vals[3] - vals[1]) - (vals[7] - vals[5]) * (vals[2] - vals[0])
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindDistance {
    pub id: BindingId,
    pub sq_distance_value: f64,
    pub va_id: VertexId,
    pub vb_id: VertexId,
}
impl BindDistance {
    pub fn bind(&self, vals: &[f64; 4]) -> f64 {
        ((vals[3] - vals[1]).powi(2) + ((vals[2] - vals[0]).powi(2)) - self.sq_distance_value).abs()
    }
}

// pub trait ApiShapes {
//     fn get_id(&self) -> ShapeTypeId;
//     fn is_selected(&self) -> bool;
//     fn set_selected(&mut self, selection: bool);
//     fn set_all_vertices(&mut self, selection: bool, vertices_pool: &mut HashMap<VertexId, Vertex>);
//     fn get_vertex_selected(&self, vertices_pool: &HashMap<VertexId, Vertex>) -> Option<VertexId>;
//     fn get_vertex_under_pos(
//         &self,
//         pt: &Point,
//         grab_handle_precision: f64,
//         vertices_pool: &HashMap<VertexId, Vertex>,
//     ) -> Option<VertexId>;
//     fn select_vertex_under_pos(
//         &mut self,
//         pt: &Point,
//         grab_handle_precision: f64,
//         vertices_pool: &mut HashMap<VertexId, Vertex>,
//     );
//     fn move_selection(&mut self, dpt: &Point, vertices_pool: &mut HashMap<VertexId, Vertex>);
//     fn move_selection_end(&mut self, vertices_pool: &mut HashMap<VertexId, Vertex>);
//     fn get_handles_vertices(&self, vertices_pool: &HashMap<VertexId, Vertex>) -> Vec<Vertex>;
//     fn get_path(&self, tol: f64, vertices_pool: &HashMap<VertexId, Vertex>) -> BezPath;
// }
// #[derive(Clone, Debug)]
// pub struct LineShape {
//     // id: ShapeTypeId,
//     selected: bool,
//     va_id: VertexId,
//     vb_id: VertexId,
// }

// impl ApiShapes for LineShape {
//     fn get_id(&self) -> ShapeTypeId {
//         self.id
//     }
//     fn is_selected(&self) -> bool {
//         self.selected
//     }
//     fn set_selected(&mut self, selection: bool) {
//         self.selected = selection
//     }
//     fn set_all_vertices(&mut self, selection: bool, vertices_pool: &mut HashMap<VertexId, Vertex>) {
//         let va = vertices_pool.get_mut(&self.va_id).unwrap();
//         va.selected = selection;
//         let vb = vertices_pool.get_mut(&self.vb_id).unwrap();
//         vb.selected = selection;
//         self.move_selection_end(vertices_pool);
//     }
//     fn get_vertex_selected(&self, vertices_pool: &HashMap<VertexId, Vertex>) -> Option<VertexId> {
//         let va = vertices_pool.get(&self.va_id).unwrap();
//         let vb = vertices_pool.get(&self.vb_id).unwrap();
//         match (va.selected, vb.selected) {
//             (true, false) => Some(self.va_id),
//             (false, true) => Some(self.vb_id),
//             _ => None,
//         }
//     }
//     fn get_vertex_under_pos(
//         &self,
//         pt: &Point,
//         grab_handle_precision: f64,
//         vertices_pool: &HashMap<VertexId, Vertex>,
//     ) -> Option<VertexId> {
//         let va = vertices_pool.get(&self.va_id).unwrap();
//         let vb = vertices_pool.get(&self.vb_id).unwrap();
//         if va.pt.distance(*pt) < grab_handle_precision {
//             return Some(self.va_id);
//         }
//         if vb.pt.distance(*pt) < grab_handle_precision {
//             return Some(self.vb_id);
//         }
//         None
//     }
//     fn select_vertex_under_pos(
//         &mut self,
//         pt: &Point,
//         grab_handle_precision: f64,
//         vertices_pool: &mut HashMap<VertexId, Vertex>,
//     ) {
//         let va = vertices_pool.get_mut(&self.va_id).unwrap();
//         if va.pt.distance(*pt) < grab_handle_precision {
//             va.saved_pt = va.pt;
//             va.selected = true;
//             return;
//         }
//         let vb = vertices_pool.get_mut(&self.vb_id).unwrap();
//         if vb.pt.distance(*pt) < grab_handle_precision {
//             vb.saved_pt = vb.pt;
//             vb.selected = true;
//             return;
//         }
//     }
//     fn move_selection(&mut self, dpt: &Point, vertices_pool: &mut HashMap<VertexId, Vertex>) {
//         if self.selected {
//             let va_sel = vertices_pool.get(&self.va_id).unwrap().selected;
//             let vb_sel = vertices_pool.get(&self.vb_id).unwrap().selected;
//             match (va_sel, vb_sel) {
//                 (false, false) => {
//                     let va = vertices_pool.get_mut(&self.va_id).unwrap();
//                     va.pt = va.saved_pt + (dpt.x, dpt.y);

//                     let vb = vertices_pool.get_mut(&self.vb_id).unwrap();
//                     vb.pt = vb.saved_pt + (dpt.x, dpt.y);
//                 }
//                 (true, false) => {
//                     let va = vertices_pool.get_mut(&self.va_id).unwrap();
//                     va.pt = va.saved_pt + (dpt.x, dpt.y);
//                 }
//                 (false, true) => {
//                     let vb = vertices_pool.get_mut(&self.vb_id).unwrap();
//                     vb.pt = vb.saved_pt + (dpt.x, dpt.y);
//                 }
//                 _ => (),
//             }
//         }
//     }
//     fn move_selection_end(&mut self, vertices_pool: &mut HashMap<VertexId, Vertex>) {
//         let va = vertices_pool.get_mut(&self.va_id).unwrap();
//         va.saved_pt = va.pt;
//         let vb = vertices_pool.get_mut(&self.vb_id).unwrap();
//         vb.saved_pt = vb.pt;
//     }
//     fn get_handles_vertices(&self, vertices_pool: &HashMap<VertexId, Vertex>) -> Vec<Vertex> {
//         let va = vertices_pool.get(&self.va_id).unwrap();
//         let vb = vertices_pool.get(&self.vb_id).unwrap();
//         vec![*va, *vb]
//     }
//     fn get_path(&self, tol: f64, vertices_pool: &HashMap<VertexId, Vertex>) -> BezPath {
//         let va = vertices_pool.get(&self.va_id).unwrap();
//         let vb = vertices_pool.get(&self.vb_id).unwrap();
//         Line::new(va.pt, vb.pt).into_path(tol)
//     }
// }

// #[derive(Clone, Debug)]
// pub enum ShapeType {
//     STLine(LineShape),
// }
// impl ShapeType {
//     pub fn new_line(line: LineShape) -> ShapeType {
//         ShapeType::STLine(line)
//     }
//     pub fn get_bez_path(&self, tol: f64, vertices_pool: &HashMap<VertexId, Vertex>) -> BezPath {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.get_path(tol, vertices_pool),
//         }
//     }
//     pub fn get_id(&self) -> ShapeTypeId {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.id,
//         }
//     }
//     pub fn is_selected(&self) -> bool {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.is_selected(),
//         }
//     }
//     pub fn set_selected(&mut self, selection: bool) {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.set_selected(selection),
//         };
//     }
//     pub fn set_vertices_selection(
//         &mut self,
//         selection: bool,
//         vertices_pool: &mut HashMap<VertexId, Vertex>,
//     ) {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.set_all_vertices(selection, vertices_pool),
//         };
//     }
//     pub fn get_vertex_selected(
//         &self,
//         vertices_pool: &HashMap<VertexId, Vertex>,
//     ) -> Option<VertexId> {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.get_vertex_selected(vertices_pool),
//         };
//         None
//     }
//     pub fn get_bounded_rectangle(&self) -> [Point; 2] {
//         // TODO
//         [Point::ZERO, Point::ZERO]
//     }
//     pub fn get_vertex_under_pos(
//         &self,
//         pick_pos: &Point,
//         grab_handle_precision: f64,
//         vertices_pool: &HashMap<VertexId, Vertex>,
//     ) -> Option<VertexId> {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => {
//                 line_shape.get_vertex_under_pos(&pick_pos, grab_handle_precision, vertices_pool)
//             }
//         }
//     }
//     pub fn select_vertex_under_pos(
//         &mut self,
//         pick_pos: &Point,
//         grab_handle_precision: f64,
//         vertices_pool: &mut HashMap<VertexId, Vertex>,
//     ) {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => {
//                 line_shape.select_vertex_under_pos(&pick_pos, grab_handle_precision, vertices_pool)
//             }
//         }
//     }
//     pub fn move_selection(&mut self, dpos: &Point, vertices_pool: &mut HashMap<VertexId, Vertex>) {
//         use ShapeType::*;
//         match self {
//             STLine(line_shape) => line_shape.move_selection(dpos, vertices_pool),
//         };
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub id: VertexId,
    pub pt: Point,
    pub saved_pt: Point,
    pub magnetic: bool,
    pub draggable: bool,
    pub selected: bool,
}
impl Vertex {
    // pub fn magnetic(mut self, magnetic: bool) -> Self {
    //     self.magnetic = magnetic;
    //     self
    // }
    // pub fn draggable(mut self, draggable: bool) -> Self {
    //     self.draggable = draggable;
    //     self
    // }
    // pub fn selected(mut self, selected: bool) -> Self {
    //     self.selected = selected;
    //     self
    // }
    pub fn dist_sq(&self, v: &Vertex) -> f64 {
        (v.pt.y - self.pt.y).powi(2) + (v.pt.x - self.pt.x).powi(2)
    }
}

#[derive(Clone, Debug)]
pub struct BindingsPool(HashMap<BindingId, Binding>);
impl Deref for BindingsPool {
    type Target = HashMap<BindingId, Binding>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BindingsPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[allow(dead_code)]
impl BindingsPool {
    pub fn new() -> BindingsPool {
        BindingsPool(HashMap::new())
    }
    pub fn add_bind_fixed(&mut self, v: &Vertex) -> BindFixed {
        let id = BindingId::new_id();
        let bind = BindFixed {
            id,
            fixed_value: v.pt,
            v_id: v.id,
        };
        self.insert(id, Binding::Fixed(bind.clone()));
        bind
    }
    pub fn add_bind_fixed_x(&mut self, v: &Vertex) -> BindFixedX {
        let id = BindingId::new_id();
        let bind = BindFixedX {
            id,
            fixed_value: v.pt.x,
            v_id: v.id,
        };
        self.insert(id, Binding::FixedX(bind.clone()));
        bind
    }
    pub fn add_bind_fixed_y(&mut self, v: &Vertex) -> BindFixedY {
        let id = BindingId::new_id();
        let bind = BindFixedY {
            id,
            fixed_value: v.pt.y,
            v_id: v.id,
        };
        self.insert(id, Binding::FixedY(bind.clone()));
        bind
    }
    pub fn add_bind_vertical(&mut self, seg: (&Vertex, &Vertex)) -> BindVertical {
        let id = BindingId::new_id();
        let bind = BindVertical {
            id,
            va_id: seg.0.id,
            vb_id: seg.1.id,
        };
        self.insert(id, Binding::Vertical(bind.clone()));
        bind
    }
    pub fn add_bind_horizontal(&mut self, seg: (&Vertex, &Vertex)) -> BindHorizontal {
        let id = BindingId::new_id();
        let bind = BindHorizontal {
            id,
            va_id: seg.0.id,
            vb_id: seg.1.id,
        };
        self.insert(id, Binding::Horizontal(bind.clone()));
        bind
    }
    pub fn add_bind_parallel(
        &mut self,
        seg1: (&Vertex, &Vertex),
        seg2: (&Vertex, &Vertex),
    ) -> BindParallel {
        let id = BindingId::new_id();
        let bind = BindParallel {
            id,
            l1va_id: seg1.0.id,
            l1vb_id: seg1.1.id,
            l2va_id: seg2.0.id,
            l2vb_id: seg2.1.id,
        };
        self.insert(id, Binding::Parallel(bind.clone()));
        bind
    }
    pub fn add_bind_distance(&mut self, seg: (&Vertex, &Vertex)) -> BindDistance {
        let id = BindingId::new_id();
        let bind = BindDistance {
            id,
            sq_distance_value: seg.0.dist_sq(seg.1),
            va_id: seg.0.id,
            vb_id: seg.1.id,
        };
        self.insert(id, Binding::Distance(bind.clone()));
        bind
    }
}

// pub struct ShapesPool(HashMap<ShapeTypeId, ShapeType>);
// impl Deref for ShapesPool {
//     type Target = HashMap<ShapeTypeId, ShapeType>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for ShapesPool {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
// impl ShapesPool {
//     pub fn new() -> ShapesPool {
//         ShapesPool(HashMap::new())
//     }
//     pub fn add_line(&mut self, va: &Vertex, vb: &Vertex) -> LineShape {
//         let id: ShapeTypeId = ShapeTypeId::new_id();
//         let line = LineShape {
//             id,
//             selected: false,
//             va_id: va.id,
//             vb_id: vb.id,
//         };
//         self.insert(id, ShapeType::STLine(line.clone()));
//         line
//     }
// }

pub struct VerticesPool(HashMap<VertexId, Vertex>);
impl Deref for VerticesPool {
    type Target = HashMap<VertexId, Vertex>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VerticesPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl VerticesPool {
    pub fn new() -> VerticesPool {
        VerticesPool(HashMap::new())
    }
    pub fn add(&mut self, pt: Point) -> Vertex {
        let id = VertexId::new_id();
        let v = Vertex {
            id,
            pt,
            saved_pt: pt,
            magnetic: true,
            draggable: true,
            selected: false,
        };
        self.insert(id, v);
        v
    }
}

static COUNTER_BINDINGS: AtomicUsize = AtomicUsize::new(0);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct BindingId(usize);
impl BindingId {
    pub fn new_id() -> BindingId {
        BindingId(COUNTER_BINDINGS.fetch_add(1, Ordering::Relaxed))
    }
}
impl Deref for BindingId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BindingId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// static COUNTER_SHAPES: AtomicUsize = AtomicUsize::new(0);
// #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
// pub struct ShapeTypeId(usize);
// impl ShapeTypeId {
//     pub fn new_id() -> ShapeTypeId {
//         ShapeTypeId(COUNTER_SHAPES.fetch_add(1, Ordering::Relaxed))
//     }
// }
// impl Deref for ShapeTypeId {
//     type Target = usize;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for ShapeTypeId {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

static COUNTER_VERTICES: AtomicUsize = AtomicUsize::new(0);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct VertexId(usize);
impl VertexId {
    pub fn new_id() -> VertexId {
        VertexId(COUNTER_VERTICES.fetch_add(1, Ordering::Relaxed))
    }
}
impl Deref for VertexId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VertexId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
