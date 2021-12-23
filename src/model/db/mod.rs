mod courses;
mod grades;

pub use courses::Entity as Courses;
pub use courses::Model as CoursesModel;
pub use courses::Column as CoursesColumn;

pub use grades::Entity as Grades;
pub use grades::Model as GradesModel;
pub use grades::Column as GradesColumn;
