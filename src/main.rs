use kurbo::Point;
mod bindings;
mod math;
use bindings::Eq2DConstraints;
use math::*;

fn main() -> Result<(), String> {
    let mut bind_pool = BindingsPool::new();
    let mut v_pool = VerticesPool::new();

    let va = v_pool.add(Point::new(-10., -30.));
    let vb = v_pool.add(Point::new(4., 20.));
    let vc = v_pool.add(Point::new(-10., 0.));
    let vd = v_pool.add(Point::new(15., 5.));

    println!("va: ({:.2},{:.2}) ", va.pt.x, va.pt.y);
    println!("vb: ({:.2},{:.2})", vb.pt.x, vb.pt.y);
    println!("vc: ({:.2},{:.2}) ", vc.pt.x, vc.pt.y);
    println!("vd: ({:.2},{:.2})", vd.pt.x, vd.pt.y);

    println!("m1: {:.4} ", (vb.pt.y - va.pt.y) / (vb.pt.x - va.pt.x));
    println!("m2: {:.4} ", (vd.pt.y - vc.pt.y) / (vd.pt.x - vc.pt.x));

    _ = bind_pool.add_bind_parallel((&va, &vb), (&vc, &vd));
    _ = bind_pool.add_bind_vertical((&vc, &vd));
    _ = bind_pool.add_bind_fixed(&va);
    _ = bind_pool.add_bind_fixed(&vc);
    _ = bind_pool.add_bind_fixed_y(&vb);
    _ = bind_pool.add_bind_fixed_y(&vd);

    let mut cst = Eq2DConstraints::new(&mut bind_pool, &mut v_pool);
    cst.solve(&mut v_pool)?;

    let va = v_pool.get(&va.id).unwrap();
    let vb = v_pool.get(&vb.id).unwrap();
    let vc = v_pool.get(&vc.id).unwrap();
    let vd = v_pool.get(&vd.id).unwrap();

    println!("va: ({:.2},{:.2}) ", va.pt.x, va.pt.y);
    println!("vb: ({:.2},{:.2})", vb.pt.x, vb.pt.y);
    println!("vc: ({:.2},{:.2}) ", vc.pt.x, vc.pt.y);
    println!("vd: ({:.2},{:.2})", vd.pt.x, vd.pt.y);

    println!("m1: {:.4} ", (vb.pt.y - va.pt.y) / (vb.pt.x - va.pt.x));
    println!("m2: {:.4} ", (vd.pt.y - vc.pt.y) / (vd.pt.x - vc.pt.x));

    Ok(())
}
