create table courses(
  id                serial      primary key,
  year              int2        not null,
  semester          text        not null,
  campus            text        not null,
  academic_group    text        not null,
  subject           text        not null,
  catalog_no        int4        not null,
  section           text        not null,
  course_no         int4        not null,
  title             text        not null,
  instructor        text,
  headcount         int4        not null,
  dropped           int4        not null,
  meeting_times     text
)