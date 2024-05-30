use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

use crate::{
    constraints::{
        angle_between_points::AngleBetweenPoints,
        distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints,
        fix_point::FixPoint, lines::perpendicular_lines::PerpendicularLines, ConstraintCell,
    },
    primitives::{line::Line, point2::Point2, PrimitiveCell},
    sketch::Sketch,
};

pub struct RotatedRectangleDemo {
    pub sketch: Rc<RefCell<Sketch>>,
    pub point_a: Rc<RefCell<Point2>>,
    pub point_b: Rc<RefCell<Point2>>,
    pub point_c: Rc<RefCell<Point2>>,
    pub point_d: Rc<RefCell<Point2>>,
    pub point_reference: Rc<RefCell<Point2>>,
}

impl RotatedRectangleDemo {
    pub fn new() -> Self {
        let sketch = Rc::new(RefCell::new(Sketch::new()));

        // This time we have to choose some random start points to break the symmetry
        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.1)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.3, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(0.3, 0.3)));
        let point_d = Rc::new(RefCell::new(Point2::new(0.1, 0.3)));

        let point_reference = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));

        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_a.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_b.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_c.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_d.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_reference.clone()))
            .unwrap();

        let line_a = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
        let line_b = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
        let line_c = Rc::new(RefCell::new(Line::new(point_c.clone(), point_d.clone())));
        let line_d = Rc::new(RefCell::new(Line::new(point_d.clone(), point_a.clone())));

        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_a.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_b.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_c.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_d.clone()))
            .unwrap();

        // Fix point a to origin
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                FixPoint::new(point_a.clone(), Vector2::new(0.0, 0.0)),
            ))))
            .unwrap();

        // Constrain line_a and line_b to be perpendicular
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::PerpendicularLines(Rc::new(RefCell::new(
                PerpendicularLines::new(line_a.clone(), line_b.clone()),
            ))))
            .unwrap();

        // Constrain line_b and line_c to be perpendicular
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::PerpendicularLines(Rc::new(RefCell::new(
                PerpendicularLines::new(line_b.clone(), line_c.clone()),
            ))))
            .unwrap();

        // Constrain line_c and line_d to be perpendicular
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::PerpendicularLines(Rc::new(RefCell::new(
                PerpendicularLines::new(line_c.clone(), line_d.clone()),
            ))))
            .unwrap();

        // // Constrain line_d and line_a to be perpendicular
        // sketch.borrow_mut().add_constraint(Rc::new(RefCell::new(PerpendicularLines::new(
        //     line_d.clone(),
        //     line_a.clone(),
        // ))));

        // Constrain the length of line_a to 2
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::EuclideanDistance(Rc::new(RefCell::new(
                EuclidianDistanceBetweenPoints::new(point_a.clone(), point_b.clone(), 2.0),
            ))))
            .unwrap();

        // Constrain the length of line_b to 3
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::EuclideanDistance(Rc::new(RefCell::new(
                EuclidianDistanceBetweenPoints::new(point_a.clone(), point_d.clone(), 3.0),
            ))))
            .unwrap();

        // Fix reference point
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                FixPoint::new(point_reference.clone(), Vector2::new(1.0, 0.0)),
            ))))
            .unwrap();

        // Constrain rotation of line_a to 45 degrees
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::AngleBetweenPoints(Rc::new(RefCell::new(
                AngleBetweenPoints::new(
                    point_reference.clone(),
                    point_b.clone(),
                    point_a.clone(),
                    f64::to_radians(45.0),
                ),
            ))))
            .unwrap();

        Self {
            sketch,
            point_a,
            point_b,
            point_c,
            point_d,
            point_reference,
        }
    }

    pub fn check(&self, eps: f64) -> Result<(), Box<dyn Error>> {
        let point_a = self.point_a.as_ref().borrow().data();
        let point_b = self.point_b.as_ref().borrow().data();
        let point_c = self.point_c.as_ref().borrow().data();
        let point_d = self.point_d.as_ref().borrow().data();
        let point_reference = self.point_reference.as_ref().borrow().data();

        let s2 = f64::sqrt(2.0);
        let s22 = s2 / 2.;

        if (point_reference - Vector2::new(1.0, 0.0)).norm() >= eps {
            return Err(format!("point_reference not solved: {:?}", point_reference).into());
        }

        if (point_a - Vector2::new(0.0, 0.0)).norm() >= eps {
            return Err(format!("point_a not solved: {:?}", point_a).into());
        }

        // Problem is under-constrained, look for b above or below the x-axis
        if point_b[1] < 0. {
            if (point_b - Vector2::new(s2, -s2)).norm() >= eps {
                return Err(format!("point_b (below) not solved: {:?}", point_b).into());
            }
            // Point c can either be up-and-right of b or down-and-left of b
            if point_c[1] < point_b[1] {
                // Point c is down-and-left of b
                if (point_c - Vector2::new(-s22, -5. * s22)).norm() >= eps {
                    return Err(format!("point_c (down,left) not solved: {:?}", point_c).into());
                }

                if (point_d - Vector2::new(-3. * s22, -3. * s22)).norm() >= eps {
                    return Err(format!("point_d (down,left) not solved: {:?}", point_d).into());
                }
            } else {
                // Point c is up-and-right of b
                if (point_c - Vector2::new(5. * s22, s22)).norm() >= eps {
                    return Err(format!("point_c (up,right) not solved: {:?}", point_c).into());
                }

                if (point_d - Vector2::new(3. * s22, 3. * s22)).norm() >= eps {
                    return Err(format!("point_d (up,right) not solved: {:?}", point_d).into());
                }
            }
        } else {
            if (point_b - Vector2::new(s2, s2)).norm() >= eps {
                return Err(format!("point_b (above) not solved: {:?}", point_b).into());
            }
            // Point c can either be up-and-left of b or down-and-right of b
            if point_c[1] > point_b[1] {
                // Point c is up-and-left of b
                if (point_c - Vector2::new(-s22, 5. * s22)).norm() >= eps {
                    return Err(format!("point_c (up,left) not solved: {:?}", point_c).into());
                }

                if (point_d - Vector2::new(-3. * s22, 3. * s22)).norm() >= eps {
                    return Err(format!("point_d (up,left) not solved: {:?}", point_d).into());
                }
            } else {
                // Point c is down-and-right of b
                if (point_c - Vector2::new(5. * s22, -s22)).norm() >= eps {
                    return Err(format!("point_c (down,right) not solved: {:?}", point_c).into());
                }

                if (point_d - Vector2::new(3. * s22, -3. * s22)).norm() >= eps {
                    return Err(format!("point_d (down,right) not solved: {:?}", point_d).into());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        examples::test_rectangle_rotated::RotatedRectangleDemo,
        solvers::{bfgs_solver::BFGSSolver, Solver},
    };

    #[test]
    pub fn test_rectangle_rotated() -> Result<(), Box<dyn Error>> {
        let rectangle = RotatedRectangleDemo::new();

        // Now solve the sketch
        let solver = BFGSSolver::new();
        solver.solve(rectangle.sketch.clone()).unwrap();

        println!("point_a: {:?}", rectangle.point_a.as_ref().borrow());
        println!("point_b: {:?}", rectangle.point_b.as_ref().borrow());
        println!("point_c: {:?}", rectangle.point_c.as_ref().borrow());
        println!("point_d: {:?}", rectangle.point_d.as_ref().borrow());
        println!(
            "point_reference: {:?}",
            rectangle.point_reference.as_ref().borrow()
        );

        rectangle.check(1e-5)
    }
}
