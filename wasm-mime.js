export default function (req, res, next) {
  if (req.url.endsWith(".wasm")) {
    res.setHeader("Content-Type", "application/wasm");
  }
  next();
}
