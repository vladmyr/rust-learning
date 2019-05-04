extern crate crossbeam;

use prs_mandelbrot::*;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 5 {
    eprintln!("Usage: prs_mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
    eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
    std::process::exit(1);
  }

  let bounds = parse_pair::<usize>(&args[2], 'x')
    .expect("error parsing image dimensions");
  let upper_left = parse_complex(&args[3])
    .expect("error parsing upper left corner point");
  let lower_right = parse_complex(&args[4])
    .expect("error parsing lower right corner point");

  let mut pixels = vec![0; bounds.0 * bounds.1];

  // render(&mut pixels, bounds, upper_left, lower_right);
  let threads = 8;
  let rows_per_band = bounds.1 / threads + 1;
  // round the count upward to make sure entire image is covered eveb uf the
  // height is not a multiple of `threads`
  
  {
    let bands: Vec<&mut [u8]> = pixels
      .chunks_mut(rows_per_band * bounds.0)
      .collect();

    crossbeam::scope(|spawner| {
      for (i, band) in bands.into_iter().enumerate() {
        let top = rows_per_band * i;
        let height = band.len() / bounds.0;
        let band_bounds = (bounds.0, height);
        let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

        spawner.spawn(move || {
          render(band, band_bounds, band_upper_left, band_lower_right);
        });
      }
    })
  }

  write_image(&args[1], &pixels, bounds)
    .expect("error writing PNG file");
}
