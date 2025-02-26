#import "/src/lib.typ": mandelbrot

#let res = 1024

#set page(
  margin: (x: 0pt, y: 0pt),
  width: res * 1pt,
  height: res * 1pt,
)

// #mandelbrot(
//   res: res,
//   max_iterations: 1500,
//   x_translation: -1.047824885,
//   y_translation: -0.248258497,
//   zoom: 0.005,
// )

#mandelbrot(res: res, max_iterations: 300)
