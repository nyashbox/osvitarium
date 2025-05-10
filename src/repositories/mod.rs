use course::CourseRepository;
use user::UserRepository;

pub mod course;
pub mod user;

pub trait RepositoryTrait: UserRepository + CourseRepository {}
impl<T> RepositoryTrait for T where T: UserRepository + CourseRepository {}
