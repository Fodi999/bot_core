use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Goal {
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub status: GoalStatus,
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoalStatus {
    Active,
    InProgress,
    Completed,
    Abandoned,
}

#[derive(Debug, Clone)]
pub struct PlanStep {
    pub action: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct Planner {
    pub goals: Vec<Goal>,
}

impl Planner {
    pub fn new() -> Self {
        Self { goals: Vec::new() }
    }

    pub fn add_goal(&mut self, description: &str, steps: Vec<&str>) {
        let goal = Goal {
            description: description.to_string(),
            created_at: Utc::now(),
            status: GoalStatus::Active,
            steps: steps
                .into_iter()
                .map(|s| PlanStep {
                    action: s.to_string(),
                    done: false,
                })
                .collect(),
        };
        self.goals.push(goal);
    }

    pub fn mark_step_done(&mut self, goal_idx: usize, step_idx: usize) {
        if let Some(goal) = self.goals.get_mut(goal_idx) {
            if let Some(step) = goal.steps.get_mut(step_idx) {
                step.done = true;
            }
            if goal.steps.iter().all(|s| s.done) {
                goal.status = GoalStatus::Completed;
            } else {
                goal.status = GoalStatus::InProgress;
            }
        }
    }

    pub fn active_goals(&self) -> Vec<&Goal> {
        self.goals
            .iter()
            .filter(|g| g.status == GoalStatus::Active || g.status == GoalStatus::InProgress)
            .collect()
    }
}
