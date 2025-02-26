#{
  let p = plugin("mandelbrot_plugin.wasm")

  let res = 512

  set page(
    margin: (x: 0pt, y: 0pt),
    width: res * 1pt,
    height: res * 1pt,
  )

  image(
    p.mandelbrot(bytes(str(res))),
    format: (
      encoding: "rgb8",
      width: res,
      height: res,
    ),
    width: (res * 1pt),
    scaling: "pixelated",
  )
}
