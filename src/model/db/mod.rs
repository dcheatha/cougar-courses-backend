mod courses;
mod grades;

pub use courses::ActiveModel as CoursesActiveModel;
pub use courses::Column as CoursesColumn;
pub use courses::Entity as Courses;
pub use courses::Model as CoursesModel;

pub use grades::ActiveModel as GradesActiveModel;
pub use grades::Column as GradesColumn;
pub use grades::Entity as Grades;
pub use grades::Model as GradesModel;
