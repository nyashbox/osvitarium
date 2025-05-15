use course::CourseRepository;
use meeting::MeetingRepository;
use user::UserRepository;

pub mod course;
pub mod meeting;
pub mod user;

pub trait RepositoryTrait: UserRepository + CourseRepository + MeetingRepository {}
impl<T> RepositoryTrait for T where T: UserRepository + CourseRepository + MeetingRepository {}
