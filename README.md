minitmpl
========

Dead Simple templating engine.

Usage:
```rust
use minitmpl::minitmpl_fn;

minitmpl_fn("Color: {{color}}", |x| match x {
	"color" => Some("red"),
	_ => None,
});
```
