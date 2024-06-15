use std::str::FromStr;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use domain::{
    aggregate::{circle::Circle, value_object::circle_id::CircleId},
    interface::circle_repository_interface::CircleRepositoryInterface,
};

#[derive(Debug, Deserialize)]
pub struct FetchCircleInput {
    pub id: String,
}

impl FetchCircleInput {
    pub fn new(id: String) -> Self {
        FetchCircleInput { id }
    }
}

#[derive(Debug)]
pub struct FetchCircleOutput {
    pub circle_id: String,
    pub circle_name: String,
    pub capacity: i16,
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberOutput {
    pub id: String,
    pub name: String,
    pub age: i16,
    pub grade: i16,
    pub major: String,
}
pub struct FetchCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    circle_repository: T,
}

impl<T> FetchCircleUsecase<T>
where
    T: CircleRepositoryInterface,
{
    pub fn new(circle_repository: T) -> Self {
        FetchCircleUsecase { circle_repository }
    }

    pub async fn execute(
        &self,
        fetch_circle_input: FetchCircleInput,
    ) -> Result<FetchCircleOutput, Error> {
        let circle_id = CircleId::from_str(fetch_circle_input.id.as_str())?;
        self.circle_repository
            .find_by_id(&circle_id)
            .await
            .map(|circle: Circle| FetchCircleOutput {
                circle_id: circle.id.into(),
                circle_name: circle.name,
                capacity: circle.capacity as i16,
                owner: MemberOutput {
                    id: String::from(circle.owner.id),
                    name: circle.owner.name,
                    age: circle.owner.age,
                    grade: i16::from(circle.owner.grade),
                    major: String::from(circle.owner.major),
                },
                members: circle
                    .members
                    .iter()
                    .map(|member| MemberOutput {
                        id: member.id.clone().into(),
                        name: member.name.clone(),
                        age: member.age,
                        grade: i16::from(member.grade),
                        major: String::from(member.major),
                    })
                    .collect(),
            })
    }
}
