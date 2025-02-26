#let _p = plugin("mandelbrot_plugin.wasm")

#let mandelbrot(res: 512, max_iterations: 1500, x_translation: 0, y_translation: 0, zoom: 1) = {
  image(
    _p.mandelbrot(
      bytes(json.encode(res)),
      bytes(json.encode(max_iterations)),
      bytes(json.encode(x_translation)),
      bytes(json.encode(y_translation)),
      bytes(json.encode(zoom)),
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
