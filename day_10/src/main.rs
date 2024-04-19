use gif::{Encoder, Decoder, Frame};
use nom::{
    bytes::complete::{is_not, take_until},
    character::complete::char,
    combinator::iterator,
    sequence::{delimited, tuple},
    IResult,
};
use clap::{
    Parser
};
use std::borrow::Cow;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Part number
    #[arg(short, long)]
    part: u8,

    /// Name of input file
    #[arg(short, long)]
    input: String,

    /// ID of correct gif found in part 1. Use only when --part is set to 2
    #[arg(long, default_value_t = -1)]
    id: isize,
}

fn tags(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((
        take_until("<"),
        delimited(char('<'), is_not(">"), char('>')),
    ))(input)
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = Args::parse();

    let untrimmed = fs::read_to_string(args.input.as_str())?;
    let input = untrimmed.trim();

    let mut canvas = Canvas(Vec::with_capacity(input.lines().count()));

    for line in input.lines() {
        let mut it = iterator(line, tags);
        let mut inside_tags = Vec::with_capacity(2);
        for parsed in &mut it {
            inside_tags.push(parsed.1);
        }
        let point = Point {
            pos: Position::from(inside_tags[0]),
            vel: Velocity::from(inside_tags[1]),
        };

        canvas.0.push(point);
    }

    match fs::create_dir("results") {
        Ok(_) => (),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(Box::new(e))
    }

    if args.part == 1 {
        part1(canvas.clone()).unwrap();
    } else if args.part == 2 {
        if args.id == -1 {
            eprintln!("for part 2, ID must be one of the image number corresponding to any image present in results directory");
            std::process::exit(1);
        }
        part2(canvas.clone(), args.id as usize).unwrap();
    } else {
        eprintln!("part number must be either 1 or 2");
        std::process::exit(1);
    }

    Ok(())
}

fn part1(mut canvas: Canvas) -> Result<()> {
    // RGB colormap for gif...[white, black]
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];

    // The points huge distances in terms of pixels to form the message. The image size can be too
    // big if we try to capture each instance. Only those instances are captured where the maximum
    // vertical and horizontal distance between the pixels is less than the threshold.
    // A value of few hundred works fine.
    let threshold = 100;

    // image count to use it in the image name
    let mut image_number = 0;

    // twenty thousand is an overkill...but who cares
    for _ in 0..20000 {
        let extremes = canvas.extremes();

        // instantaneous width and height based on extremes
        let (width, height) = (
            (extremes[1].1 - extremes[1].0 + 1) as u16,
            (extremes[0].1 - extremes[0].0 + 1) as u16,
        );

        // if image is too big then just do forward and skip to next instant
        if width > threshold && height > threshold {
            canvas.forward();
            continue;
        }

        // create name of the gif file
        let name = format!("results/{}.gif", image_number);
        let mut gif = fs::File::create(name.as_str()).unwrap();
        let mut encoder = Encoder::new(&mut gif, width, height, color_map).unwrap();

        // origin based on extremes
        let origin: (i64, i64) = (-1 * extremes[1].0, -1 * extremes[0].0);

        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;

        // 0/1 is the index of color in the color_map for the pixels in the gif
        let mut buffer = vec![0u8; (width as usize * height as usize).into()];

        // skip edge case...unlikely to occur but it can
        let len = buffer.len();
        if len == 0 {
            continue;
        }

        // make pixels black at the position of the points, leave others at default white
        canvas.positions().iter().for_each(|pos| {
            let index = (origin.1 + pos.y) as i64 * width as i64 + (origin.0 + pos.x) as i64;
            match index {
                0.. => {
                    *buffer.get_mut(index as usize % len).unwrap() = 1;
                }
                _ => (),
            };
        });
        frame.buffer = Cow::Borrowed(&buffer);
        encoder.write_frame(&frame).unwrap();
        image_number += 1;
        canvas.forward();
    }

    println!("No. of images written: {}", image_number);
    println!("Go through the images in 'result' directory and get the id of the correct one for part 2");

    Ok(())
}

fn part2(mut canvas: Canvas, img_number: usize) -> Result<()> {
    let filename = format!("results/{img_number}.gif");
    let file = fs::File::open(filename.as_str()).unwrap();
    let decoder = Decoder::new(file).unwrap();

    // Get width and height of the image instance that contains the correct message.
    // The idea is that whenever an image instance is exactly the same size as this, that will
    // be the time instance when the message appeared.
    // It may not work but thats highly unlikely.
    let (w, h) = (decoder.width(), decoder.height());

    for time in 0..20000 {
        let extremes = canvas.extremes();

        // instantaneous width and height based on extremes
        let (width, height) = (
            (extremes[1].1 - extremes[1].0 + 1) as u16,
            (extremes[0].1 - extremes[0].0 + 1) as u16,
        );

        if width == w && height == h {
            println!("Image of size {}x{} appeared @ time {}s", w, h, time);
        }

        canvas.forward();
    }

    Ok(())
}

/// Point position
#[derive(Clone)]
pub struct Position {
    x: i64,
    y: i64,
}

/// Point velocity
#[derive(Clone)]
pub struct Velocity {
    vx: i64,
    vy: i64,
}

#[derive(Clone)]
pub struct Point {
    pos: Position,
    vel: Velocity,
}

/// extracting velocity from strings of type "vx, vy"
impl From<&str> for Velocity {
    fn from(value: &str) -> Self {
        let numbers = value
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Velocity {
            vx: numbers[0],
            vy: numbers[1],
        }
    }
}

/// extracting position from strings of type "x, y"
impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let numbers = value
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Position {
            x: numbers[0],
            y: numbers[1],
        }
    }
}

#[derive(Clone)]
pub struct Canvas(Vec<Point>);

impl Canvas {
    /// number of points on the canvas
    fn count(&self) -> usize {
        self.0.len()
    }

    /// forward time 1 second and update the position of each point
    fn forward(&mut self) {
        for point in &mut self.0 {
            point.pos.x += point.vel.vx;
            point.pos.y += point.vel.vy;
        }
    }

    /// position of all the points at a time
    fn positions(&self) -> Vec<&Position> {
        let mut positions = Vec::with_capacity(self.count());
        for point in &self.0 {
            positions.push(&point.pos);
        }
        positions
    }

    /// get vertical and horizontal extremes [(v_low, v_high), (h_low, h_high)]
    fn extremes(&self) -> [(i64, i64); 2] {
        let positions = self.positions();
        let (mut v_low, mut v_high, mut h_low, mut h_high) =
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
        for pos in positions {
            if pos.y >= v_high {
                v_high = pos.y;
            }

            if pos.y <= v_low {
                v_low = pos.y
            }

            if pos.x >= h_high {
                h_high = pos.x;
            }

            if pos.x <= h_low {
                h_low = pos.x;
            }
        }
        [(v_low, v_high), (h_low, h_high)]
    }
}
