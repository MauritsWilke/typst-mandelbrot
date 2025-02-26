#import "/src/lib.typ": mandelbrot, render_options



// #set page(
//   margin: (x: 0pt, y: 0pt),
//   width: res * 1pt,
//   height: res * 1pt,
// )


// #for (k, option) in render_options {
//   box(
//     mandelbrot(
//       res: res,
//       max_iterations: 300,
//       x_translation: -1.047824885,
//       y_translation: -0.248258497,
//       zoom: 0.005,
//       render_mode: option,
//     ),
//     height: 100pt,
//     width: 100pt,
//     inset: 0pt,
//     outset: 0pt,
//   )
// }

#let res = 512
#set page(margin: (x: 0pt, y: 0pt), width: (res * 2 * 1pt), height: (res * 3 * 1pt))

#let mandels = (
  render_options
    .values()
    .map(option => {
      box(
        width: (res * 1pt),
        height: (res * 1pt),
        [#mandelbrot(
            res: res,
            max_iterations: 300,
            x_translation: -1.047824885,
            y_translation: -0.248258497,
            zoom: 0.005,
            render_mode: option,
          )],
      )
    })
)

#table(
  columns: 2,
  rows: 3,
  gutter: 0pt,
  inset: 0pt,
  [#table.cell(colspan: 2, [#mandels.at(2)], align: center)], [#mandels.at(4)],
  [#mandels.at(0)], [#mandels.at(3)],
  [#mandels.at(1)],
)

// #mandelbrot(res: res, max_iterations: 300)
