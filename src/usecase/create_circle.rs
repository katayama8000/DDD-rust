use anyhow::Result;
use serde::Deserialize;

use crate::domain::{
    aggregate::{
        circle::Circle,
        member::Member,
        value_object::{grade::Grade, major::Major},
    },
    interface::circle_repository_interface::CircleRepositoryInterface,
};

#[derive(Debug, Deserialize)]
pub struct CreateCircleInput {
    pub circle_name: String,
    pub capacity: i32,
    pub owner_name: String,
    pub owner_age: i32,
    pub owner_grade: i32,
    pub owner_major: String,
}

impl CreateCircleInput {
    pub fn new(
        circle_name: String,
        capacity: i32,
        owner_name: String,
        owner_age: i32,
        owner_grade: i32,
        owner_major: String,
    ) -> Self {
        CreateCircleInput {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCircleOutput {
    pub circle_id: i32,
    pub owner_id: i32,
}

pub struct CreateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> CreateCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        CreateCircleUsecase { circle_repository }
    }

    pub async fn execute(
        &mut self,
        circle_circle_input: CreateCircleInput,
    ) -> Result<CreateCircleOutput> {
        let grade = Grade::try_from(circle_circle_input.owner_grade)?;

        let major = Major::from(circle_circle_input.owner_major.as_str());

        let owner = Member::new(
            circle_circle_input.owner_name,
            circle_circle_input.owner_age,
            grade,
            major,
        );
        let owner_id = owner.id;
        let circle = Circle::new(
            circle_circle_input.circle_name,
            owner,
            circle_circle_input.capacity,
        )?;
        self.circle_repository
            .create(&circle)
            .await
            .map(|_| CreateCircleOutput {
                circle_id: i32::from(circle.id),
                owner_id: i32::from(owner_id),
            })
    }
}
