create table grades(
  id          serial                                      primary key,
  course_id   int4      not null    references courses,
  grade       float8    not null,
  headcount   int4      not null
)