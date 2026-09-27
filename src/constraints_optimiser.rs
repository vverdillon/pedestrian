use crate::constraints::Constraint;

#[derive(Debug, Clone, Copy)]
pub struct GradientDescent {
    pub learning_rate: f32,
    pub iterations_max: usize,
    pub stopping_condition: f32,
}

impl Default for GradientDescent {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            iterations_max: 4096,
            stopping_condition: 0.01,
        }
    }
}

#[derive(Debug)]
pub struct SolveResult {
    pub variables: Vec<f32>,
    pub converged: bool,
}

impl GradientDescent {
    pub fn solve(&self, initial: &[f32], constraints: &[&dyn Constraint]) -> SolveResult {
        let mut variables = initial.to_vec();

        for _ in 0..self.iterations_max {
            let score = total_score(&variables, constraints);
            let gradient = total_gradient(&variables, constraints);
            let gradient_norm = gradient.iter().map(|value| value * value).sum::<f32>();

            if !gradient_norm.is_finite() {
                return SolveResult {
                    variables,
                    converged: false,
                };
            }

            if gradient_norm <= self.stopping_condition {
                return SolveResult {
                    variables,
                    converged: true,
                };
            }

            let mut step = self.learning_rate;
            let mut improved = false;
            let reduction_coeff = 0.5;

            for _ in 0..12 {
                let candidate = variables
                    .iter()
                    .zip(&gradient)
                    .map(|(value, derivative)| value - step * derivative)
                    .collect::<Vec<_>>();

                if total_score(&candidate, constraints) < score {
                    variables = candidate;
                    improved = true;
                    break;
                }

                if step < 0.000001 {
                    break;
                }

                step *= reduction_coeff;
            }

            if !improved {
                let current_score = total_score(&variables, constraints);
                return SolveResult {
                    variables,
                    converged: current_score <= self.stopping_condition,
                };
            }
        }

        let final_score = total_score(&variables, constraints);

        SolveResult {
            variables,
            converged: final_score <= self.stopping_condition,
        }
    }
}

fn total_score(variables: &[f32], constraints: &[&dyn Constraint]) -> f32 {
    constraints
        .iter()
        .map(|constraint| constraint.score(variables))
        .sum()
}

fn total_gradient(variables: &[f32], constraints: &[&dyn Constraint]) -> Vec<f32> {
    let mut gradient = vec![0.0; variables.len()];

    for constraint in constraints {
        for (total, contribution) in gradient.iter_mut().zip(constraint.gradient(variables)) {
            *total += contribution;
        }
    }

    gradient
}

#[cfg(test)]
mod tests {
    use super::GradientDescent;
    use crate::constraints::{Constraint, ConstraintDistance, ConstraintMouse};
    use crate::sketch::Point;

    #[test]
    fn solves_mouse_position() {
        let mouse = ConstraintMouse::new(100.0, 0, Point { x: 20.0, y: 30.0 });
        let constraints: [&dyn Constraint; 1] = [&mouse];
        let solved = GradientDescent::default().solve(&[0.0, 0.0], &constraints);

        assert!(solved.converged);
        assert!((solved.variables[0] - 20.0).abs() < 0.01, "{solved:?}");
        assert!((solved.variables[1] - 30.0).abs() < 0.01, "{solved:?}");
    }

    #[test]
    fn preserves_segment_length_while_moving_a_point() {
        let distance = ConstraintDistance::new(1.0, 10.0, 0, 1);
        let mouse = ConstraintMouse::new(100.0, 0, Point { x: 20.0, y: 0.0 });
        let constraints: [&dyn Constraint; 2] = [&distance, &mouse];
        let solved = GradientDescent::default().solve(&[0.0, 0.0, 10.0, 0.0], &constraints);

        assert!(solved.converged);
        assert!((solved.variables[0] - 20.0).abs() < 0.1, "{solved:?}");
        let length = (solved.variables[0] - solved.variables[2]).abs();
        assert!((length - 10.0).abs() < 0.1, "{solved:?}");
    }
}
