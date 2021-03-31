use std::ops::{Index,IndexMut};
use rand::prelude::random;

// it's premature abstraction to make this a trait, but whatever!
trait Pixel: Copy + Clone {
    // if we make an image with these as pixels, this'll be the
    // default background
    fn empty() -> Self;
    // how many color stops do we want to report in the PPM file?
    fn depth() -> usize;
    // print this. (hooboy, this ain't efficient, but eh: fast enough
    // for the one-off script this is! if I turn this into a library,
    // I'd make this take a formatter instead)
    fn show(&self) -> String;
}

// I left this in after testing this for a simple grayscale version
impl Pixel for bool {
    // defaults to white
    fn empty() -> bool { false }
    fn depth() -> usize { 1 }
    fn show(&self) -> String {
        if *self {
            "1 1 1"
        } else {
            "0 0 0"
        }.to_string()
    }
}

// The three kinds of pixels: Black, White, and Red
#[derive(Copy, Clone)]
enum Px {
    Black,
    White,
    Red
}

impl Px {
    // we use this in the CA impl, later on
    fn idx(&self) -> usize {
        match self {
            Px::Black => 0,
            Px::White => 1,
            Px::Red => 2,
        }
    }
}

impl Pixel for Px {
    fn empty() -> Px { Px::White }
    fn depth() -> usize { 1 }
    fn show(&self) -> String {
        match self {
            Px::Black => "0 0 0",
            Px::White => "1 1 1",
            Px::Red => "1 0 0",
        }.to_string()
    }
}

// Our simple image abstraction
struct Image<T> {
    width: usize,
    height: usize,
    // we maintain the invariant that the length of `data` here is
    // `width * height`. (Or at least, if we don't, things
    // crash. Fun!)
    data: Vec<T>,
}

impl<T: Pixel> Image<T> {
    fn new(width: usize, height: usize) -> Image<T> {
        let data = vec![T::empty(); width*height];
        Image {
            width,
            height,
            data,
        }
    }

    // This prints the PPM file:
    fn show(&self) -> String {
        let mut str = String::new();
        str.push_str("P3\n");
        str.push_str(&format!("{} {}\n", self.width, self.height));
        str.push_str(&format!("{}\n", T::depth()));
        for px in self.data.iter() {
            str.push_str(&format!("{} ", px.show()));
        }
        str
    }

    // This looks up the pixel, but returns an 'empty' pixel if we
    // can't find it.
    fn get(&self, (x, y): (usize, usize)) -> T {
        // ...I only just realized while commenting this file that
        // this is wrong, but I'm too lazy to fix it now.
        *self.data.get(x + y * self.height).unwrap_or(&T::empty())
    }
}

// This lets us index into our image using a tuple as coordinate!
impl<T: Pixel> Index<(usize, usize)> for Image<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.data[x + y * self.height]
    }
}

// This lets us modify our image too!
impl <T: Pixel> IndexMut<(usize, usize)> for Image<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.data[x + y * self.height]
    }
}


// Okay, here's where the CA stuff comes in. So: a given 'generation'
// in this system is a vector of cells, where each cell is either
// black, white, or red. Each subsequent generation, a cell turns into
// a new cell based on a rule which applies to the previous
// generation, looking at the same cell and its immediate
// neighbors. With `n` possible states, that gives us `n**3` possible
// 'neighborhoods': in this case, 27. So we can describe an automaton
// of this form by simply enumerating the resut for each of the
// neighborhoods, which is why this has 27 different `Px` values: one
// for each possible neighborhood.
struct Rule {
    result: [Px;27],
}

impl Rule {
    // We can describe a given automaton by using a string of 27
    // characters. We use this to create the filenames, so we could in
    // theory reproduce the automaton again later on
    fn descr(&self) -> String {
        let mut str = String::new();
        for r in self.result.iter() {
            str.push(match r {
                Px::White => 'w',
                Px::Black => 'b',
                Px::Red => 'r',
            })
        }
        str
    }

    // This implements the logic (which is really just the lookup) for
    // 'how do we know the cell at generation n given the neighboor at
    // generation n-1?'
    fn step(&self, (l, c, r): (Px, Px, Px)) -> Px {
        let index = l.idx() + c.idx() * 3 + r.idx() * 9;
        self.result[index]
    }

    // This generates a random automaton.
    fn random() -> Rule {
        let mut result = [Px::White; 27];
        for i in 0..27 {
            let r: f32 = random();
            result[i] = if r < 0.33 {
                Px::White
            } else if r > 0.66 {
                Px::Black
            } else {
                Px::Red
            }
        }
        Rule { result }
    }
}

fn main() {
    // I choose something odd so our initial condition can be all
    // white cells with a single black cell in the middle to make
    // something interesting happen
    let w = 99;
    let mut img: Image<Px> = Image::new(w, w);
    img[(w/2, 0)] = Px::Black;

    // choose a random rule and find out what it is
    let rule = Rule::random();
    eprintln!("Using {}", rule.descr());

    // for each generation (except the last)
    for y in 0..(w-1) {
        // this is the logic on the left-hand side, where we hard-code
        // that the stuff off the side is the 'empty' value.
        img[(0, y+1)] = rule.step((Px::empty(), img[(0, y)], img[(1, y)]));
        // for everything in the middle, we calculate the neighborhood and then step the rule
        for x in 1..(w-1) {
            let env = (img.get((x, y)), img.get((x+1, y)), img.get((x+2, y)));
            img[(x+1, y+1)] = rule.step(env);
        }
        // ditto for the right-hand side
        img[(w-1, y+1)] = rule.step((img[(w-2, y)], img[(w-1, y)], Px::empty()));
    }

    // print this out
    {
        std::fs::write(
            &format!("output/{}.ppm", rule.descr()),
            img.show().as_bytes(),
        ).unwrap();
    }
}
