#let _p = plugin("mandelbrot_plugin.wasm")

#let mandelbrot(res: 128, max_iterations: 100) = {
  image(
    _p.mandelbrot(
      bytes(str(res)),
      bytes(str(max_iterations)),
    ),
    format: (
      encoding: "rgb8",
      width: res,
      height: res,
    ),
    width: (res * 1pt),
    scaling: "pixelated",
  )
}
