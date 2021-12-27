create index ix_courses_year
	on courses (year);

create index ix_courses_semester
	on courses (semester);

create index ix_courses_campus
	on courses (campus);

create index ix_courses_academic_group
	on courses (academic_group);

create index ix_courses_subject
	on courses (subject);

create index ix_courses_catalog_no
	on courses (catalog_no);

create index ix_courses_section
	on courses (section);

create index ix_courses_course_no
	on courses (course_no);

create index ix_courses_title
	on courses (title);

create index ix_courses_instructor
	on courses (instructor);

create index ix_courses_headcount
	on courses (headcount);

create index ix_courses_dropped
	on courses (dropped);

create index ix_courses_meeting_times
	on courses (meeting_times);
