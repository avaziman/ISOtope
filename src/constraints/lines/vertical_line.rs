use std::{cell::RefCell, rc::Rc};

use nalgebra::SMatrix;
use serde::{Deserialize, Serialize};

use crate::{constraints::Constraint, primitives::line::Line};

// This is a sketch constraint that makes the end point of an arc coincident with a point.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct VerticalLine {
    line: Rc<RefCell<Line>>,
}

impl VerticalLine {
    pub fn new(line: Rc<RefCell<Line>>) -> Self {
        Self { line }
    }

    pub fn line(&self) -> Rc<RefCell<Line>> {
        self.line.clone()
    }

    pub fn set_line(&mut self, line: Rc<RefCell<Line>>) {
        self.line = line;
    }
}

impl Constraint for VerticalLine {
    fn references(&self) -> Vec<Rc<RefCell<dyn crate::primitives::Parametric>>> {
        vec![self.line.clone()]
    }

    fn loss_value(&self) -> f64 {
        let start = self.line.borrow().start().borrow().data();
        let end = self.line.borrow().end().borrow().data();
        let dx = end.x - start.x;
        0.5 * dx * dx
    }

    fn update_gradient(&mut self) {
        let start = self.line.borrow().start().borrow().data();
        let end = self.line.borrow().end().borrow().data();
        let dx = end.x - start.x;

        let gradient_constraint = SMatrix::<f64, 1, 2>::from_row_slice(&[dx, 0.0]);

        let grad_start = self.line.borrow().start_gradient();
        let grad_end = self.line.borrow().end_gradient();

        self.line
            .borrow_mut()
            .add_to_gradient((-gradient_constraint * grad_start).as_view());
        self.line
            .borrow_mut()
            .add_to_gradient((gradient_constraint * grad_end).as_view());
    }

    fn get_type(&self) -> crate::constraints::ConstraintType {
        crate::constraints::ConstraintType::VerticalLine(self.clone())
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        constraints::{lines::vertical_line::VerticalLine, ConstraintCell},
        primitives::{line::Line, point2::Point2, ParametricCell},
        sketch::Sketch,
        solvers::gradient_based_solver::GradientBasedSolver,
    };

    #[test]
    fn test_vertical_line() {
        let sketch = Rc::new(RefCell::new(Sketch::new()));

        let line_start = Rc::new(RefCell::new(Point2::new(3.0, 4.0)));
        let line_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line = Rc::new(RefCell::new(Line::new(
            line_start.clone(),
            line_end.clone(),
        )));
        sketch
            .borrow_mut()
            .add_primitive(ParametricCell(line_start.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(ParametricCell(line_end.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(ParametricCell(line.clone()))
            .unwrap();

        let constr1 = Rc::new(RefCell::new(VerticalLine::new(line.clone())));
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell(constr1.clone()))
            .unwrap();

        sketch
            .borrow_mut()
            .check_gradients(1e-6, constr1.clone(), 1e-6);
        let solver = GradientBasedSolver::new(sketch.clone());
        solver.solve();

        println!("line: {:?}", line.as_ref().borrow());

        assert!(
            (line.as_ref().borrow().end().borrow().data().x
                - line.as_ref().borrow().start().borrow().data().x)
                .abs()
                < 1e-6
        );
    }
}
