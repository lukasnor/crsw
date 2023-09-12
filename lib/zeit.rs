use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum Direction {
    #[serde(rename = "v")]
    Vertical,
    #[serde(rename = "h")]
    Horizontal,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct MyChar(char);
impl Default for MyChar {
    fn default() -> Self {
        MyChar('.')
    }
}

impl From<char> for MyChar {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl Into<char> for MyChar {
    fn into(self) -> char {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct GridCell {
    nr: Option<usize>,
    in_horizontal: bool,
    in_vertical: bool,
    thick_top: bool,
    thick_side: bool,
    b: MyChar,
}

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // possible/usual output "|[14][ftl]X\t"
        let mut s = String::with_capacity(13);
        s.push('|');
        s.push('[');
        if let Some(i) = self.nr {
            s.push_str(i.to_string().as_ref());
        }
        s.push(']');
        s.push_str("[f");
        if self.thick_top {
            s.push('t');
        }
        if self.thick_side {
            s.push('l');
        }
        s.push(']');
        s.push(self.b.into());
        s.push(' ');
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Option<GridCell>>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let grid = &self.0;
        // capacity = rows * (columns + 1) for the \n
        let mut s = String::with_capacity(grid.len() * (grid[0].len() + 1));
        for row in grid.iter() {
            for cell in row.iter() {
                match cell {
                    None => {
                        s.push('.');
                    }
                    Some(cell) => {
                        s.push(cell.b.into());
                    }
                }
            }
            s.push('\n');
        }
        // Remove last
        //s.pop();
        write!(f, "{}", s)
    }
}

impl Grid {
    fn latex(&self) -> String {
        let mut latex = String::new();

        latex.push_str(
            &("\\begin{Puzzle}{".to_owned()
                + &self.0[0].len().to_string()
                + "}{"
                + &self.0.len().to_string()
                + "}\n")
        );
        for row in self.0.iter() {
            for cell in row {
                match cell {
                    None => {
                        latex.push_str("|{} ");
                    }
                    Some(cell) => {
                        latex.push_str(&cell.to_string());
                    }
                }
            }
            latex.push_str("|.\n")
        }
        latex.push_str("\\end{Puzzle}");
        latex
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Question {
    id: usize,
    game_id: usize,
    nr: usize,
    question: String,
    answer: String,
    xc: usize,
    yc: usize,
    direction: Direction,
    description: String,
    length: usize,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frage {}: {}\nAntwort: {}\nErklärung: {}", self.nr ,self.question, self.answer, self.description)
    }
}

impl Question {
    fn latex(&self) -> String {
        let mut s = String::new();
        s.push_str(&("\\Clue{".to_owned()+&self.nr.to_string()+"}{}{"+&self.question+"}"));
        s
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(from = "ShadowGame")]
pub struct Game {
    id: usize,
    name: String,
    #[serde(rename = "gameNr")]
    game_nr: String,
    #[serde(rename = "isContest")]
    is_contest: bool,
    #[serde(rename = "releaseDate")]
    release_date: String,
    #[serde(rename = "additionalInfo")]
    additional_info: String,
    pub questions: Vec<Question>,
    grid: Grid,
}

impl Game {
    pub fn latex(&self) -> String{

        let mut s = String::from(&("\\fancyhead[LO]{Um die Ecke Gedacht Nr. ".to_owned()+ &self.game_nr.to_string()+"}\n\n"));

        let grid = self.grid.latex();
        s.push_str(&grid);
        s.push_str("\n\n");

        let horizontal_qs :Vec<&Question> = self.questions.iter().filter(|q| q.direction == Direction::Horizontal).collect();
        let mut hqs = String::from("\\begin{PuzzleClues}{\\sffamily\\textbf{Waagerecht}}\n");
        for q in horizontal_qs {
            hqs.push_str(&q.latex());
            hqs.push('\n');
        }
        hqs.push_str(&"\\end{PuzzleClues}\n");
        s.push_str(&hqs);

        let mut vqs = String::from("\\begin{PuzzleClues}{\\sffamily\\textbf{Senkrecht}}\n");
        let vertical_qs: Vec<&Question> = self.questions.iter().filter(|q| q.direction == Direction::Vertical).collect();
        for q in vertical_qs {
            vqs.push_str(&q.latex());
            vqs.push('\n');
        }
        vqs.push_str(&"\\end{PuzzleClues}\n");
        s.push_str(&vqs);

        s
    }
}

#[derive(Deserialize)]
struct ShadowGame {
    id: usize,
    name: String,
    #[serde(rename = "gameNr")]
    game_nr: String,
    #[serde(rename = "isContest")]
    is_contest: bool,
    #[serde(rename = "releaseDate")]
    release_date: String,
    #[serde(rename = "additionalInfo")]
    additional_info: String,
    questions: Vec<Question>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl From<ShadowGame> for Game {
    fn from(value: ShadowGame) -> Self {
        let grid = value.construct_grid();
        Game {
            id: value.id,
            name: value.name,
            game_nr: value.game_nr,
            is_contest: value.is_contest,
            release_date: value.release_date,
            additional_info: value.additional_info,
            questions: value.questions,
            grid,
        }
    }
}

impl ShadowGame {
    pub fn construct_grid(&self) -> Grid {
        // early return if grid is computed
        let rows = self
            .questions
            .iter()
            .filter(|q| q.direction == Direction::Vertical)
            .map(|q| q.yc - 1 + q.length)
            .chain(
                self.questions
                    .iter()
                    .filter(|q| q.direction == Direction::Horizontal)
                    .map(|q| q.yc - 1),
            )
            .reduce(std::cmp::max)
            .expect("The grid is nonempty.");
        let columns = self
            .questions
            .iter()
            .filter(|q| q.direction == Direction::Horizontal)
            .map(|q| q.xc - 1 + q.length)
            .chain(
                self.questions
                    .iter()
                    .filter(|q| q.direction == Direction::Vertical)
                    .map(|q| q.xc - 1),
            )
            .reduce(std::cmp::max)
            .expect("The grid is nonempty.");

        let mut grid: Vec<Vec<Option<GridCell>>> = Vec::with_capacity(rows);
        let mut row = Vec::with_capacity(columns);
        for _ in 0..columns {
            row.push(Some(Default::default()));
        }
        for _ in 0..rows {
            grid.push(row.clone());
        }

        // Change cells, that are in a horizontal word
        for q in self
            .questions
            .iter()
            .filter(|q| q.direction == Direction::Horizontal)
        {
            let cell = &mut grid[q.yc - 1][q.xc - 1];
            // Set number and initial thick line
            *cell = cell.take().and_then(|mut c| {
                c.nr = Some(q.nr);
                c.thick_side = true;
                Some(c)
            });
            // Set the horizontal flag
            for i in 0..q.length {
                let cell = &mut grid[q.yc - 1][q.xc - 1 + i];
                *cell = cell.take().and_then(|mut c| {
                    c.in_horizontal = true;
                    c.b = q.answer.chars().nth(i).expect("i < q.length").into();
                    Some(c)
                });
            }
        }

        // Change cells, that are in a vertical word
        for q in self
            .questions
            .iter()
            .filter(|q| q.direction == Direction::Vertical)
        {
            let cell = &mut grid[q.yc - 1][q.xc - 1];
            // Set number and initial thick line
            *cell = cell.take().and_then(|mut c| {
                c.nr = Some(q.nr);
                c.thick_top = true;
                Some(c)
            });
            // Set the vertical flag
            for i in 0..q.length {
                let cell = &mut grid[q.yc - 1 + i][q.xc - 1];
                *cell = cell.take().and_then(|mut c| {
                    c.in_vertical = true;
                    c.b = q.answer.chars().nth(i).expect("i < q.length").into();
                    Some(c)
                });
            }
        }

        // Set cells which are neither in hor not in vert to None
        for cell in grid.iter_mut().flatten().filter(|c| match c {
            Some(c) => !c.in_vertical && !c.in_horizontal,
            _ => false,
        }) {
            *cell = None;
        }

        // Set thick walls for cells, which are in a word of one direction and not in a word of
        // another
        for cell in grid
            .iter_mut()
            .flatten()
            .map(Option::as_mut)
            .filter(|c| match c {
                Some(c) => c.in_vertical ^ c.in_horizontal,
                _ => false,
            })
            .flatten()
        // Option<T> is IntoIterator, and flattening Iterator<Item=Option<T>> means retaining and unwrapping the Some(T) variants, nice!
        {
            if cell.in_horizontal {
                cell.thick_top = true;
            } else {
                cell.thick_side = true;
            }
        }

        Grid(grid)
    }
}
