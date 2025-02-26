#import "/src/lib.typ": mandelbrot

#let res = 1024

#set page(
  margin: (x: 0pt, y: 0pt),
  width: res * 1pt,
  height: res * 1pt,
)

#mandelbrot(
  res: res,
  max_iterations: 100,
)
