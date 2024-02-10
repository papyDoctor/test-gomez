use kurbo::Point;
mod bindings;
mod math;
use bindings::Eq2DConstraints;
use math::*;

fn main() -> Result<(), String> {
    let mut bind_pool = BindingsPool::new();
    let mut v_pool = VerticesPool::new();

    let va = v_pool.add(Point::new(-27., 30.));
    let vb = v_pool.add(Point::new(120., 20.));
    let vc = v_pool.add(Point::new(-10., 60.));
    let vd = v_pool.add(Point::new(0., 52.));

    println!("va: ({:.2},{:.2}) ", va.pt.x, va.pt.y);
    println!("vb: ({:.2},{:.2})", vb.pt.x, vb.pt.y);
    println!("vc: ({:.2},{:.2}) ", vc.pt.x, vc.pt.y);
    println!("vd: ({:.2},{:.2})", vd.pt.x, vd.pt.y);

    println!("m1: {:.4} ", (vb.pt.y - va.pt.y) / (vb.pt.x - va.pt.x));
    println!("m2: {:.4} ", (vd.pt.y - vc.pt.y) / (vd.pt.x - vc.pt.x));
    println!("dist(va,vd): {:.4} ", va.dist_sq(&vd));

    // 8 DOF and 8 Eq => Determined system
    // Undertermined system sometimes work (converge) sometimes no
    _ = bind_pool.add_bind_parallel((&va, &vb), (&vc, &vd)); // 1 eq
    _ = bind_pool.add_bind_vertical((&vc, &vd)); // 1 eq
    _ = bind_pool.add_bind_fixed(&va); // 2 eq
    _ = bind_pool.add_bind_fixed(&vc); // 2 eq
    _ = bind_pool.add_bind_fixed_y(&vb); // 1 eq
    _ = bind_pool.add_bind_distance((&va, &vd)); // 1 eq

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
    println!("dist(va,vd): {:.4} ", va.dist_sq(&vd));

    Ok(())
}
