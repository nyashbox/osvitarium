use activity::ActivityRepository;
use course::CourseRepository;
use meeting::MeetingRepository;
use user::UserRepository;

pub mod activity;
pub mod course;
pub mod meeting;
pub mod user;

pub trait RepositoryTrait:
    UserRepository + CourseRepository + MeetingRepository + ActivityRepository
{
}
impl<T> RepositoryTrait for T where
    T: UserRepository + CourseRepository + MeetingRepository + ActivityRepository
{
}
