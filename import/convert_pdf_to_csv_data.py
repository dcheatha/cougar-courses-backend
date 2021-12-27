import tabula
import pandas as pd
import re
from operator import is_not
from functools import partial 
import json

def load_data():
  course_data = tabula.read_pdf('import/20-319_Installment_No_Redactions.pdf', pages='all', pandas_options={'header': None})
  course_data = pd.concat(course_data)
  course_data.to_csv(f"import/data.csv", header = True)
  return course_data

term_matcher = re.compile('(?P<year>[0-9]{4}) (?P<term>[A-z]+)')
def parse_term(term):
  matches = re.search(term_matcher, term)

  if not matches:
    return (None, None)

  term, year = matches.group('term'), matches.group('year')

  return term, int(year)


grade_mapping = [ 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.0 ]
def parse_grades(row):
  grades = map(lambda idx: (grade_mapping[idx[0]], None if pd.isna(row[idx[1]]) else int(row[idx[1]])), enumerate(range(9, 20)))
  return list(filter(lambda grade: not pd.isna(grade[1]) and not grade[1] == 0, grades))

def parse_col(col):
  return None if pd.isna(col) else col


def parse_int(value):
  if value:
    return int(value)
  else:
    return 0
def parse_row(row):
  if not should_parse(row):
    return None

  term, year = parse_term(row[0])
  grades = parse_grades(row)

  return {
    'year':               year,
    'semester':           term,
    'campus':             parse_col(row[1]),
    'academic_group':     parse_col(row[2]),
    'subject':            parse_col(row[3]),
    'catalog_no':         parse_int(parse_col(row[4])),
    'section':            str(parse_col(row[5])),
    'course_no':          parse_int(parse_col(row[6])),
    'title':              parse_col(row[7]),
    'meeting_times':      parse_col(row[21]),
    'instructor':         parse_col(row[22]),
    'headcount':          parse_int(parse_col(row[8])),
    'dropped':            parse_int(parse_col(row[20])),
    'grades':             grades
  }

def should_parse(row):
  if not isinstance(row[0], str):
    return False

  term, year = parse_term(row[0])

  if not term or not year:
    return False

  return True

def main():
  course_data = map(lambda row: parse_row(row[1]), load_data().iterrows())
  course_data = filter(partial(is_not, None), course_data)

  with open('import/data.json', 'w', encoding='utf-8') as dump_file:
    for datum in course_data:
      json.dump(datum, dump_file)
      dump_file.write("\n")

    dump_file.close()


main()
