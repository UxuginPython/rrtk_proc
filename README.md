# RRTK Procedural Macros
[HIGHLY EXPERIMENTAL] Procedural `math!` macro making the RRTK math streams easier to use. Requires RRTK 0.6 with the `alloc` feature enabled.

Do this:
```
let pid = math!(kp * error + ki * int + kd * drv);
```
instead of this:
```
let p = make_rc_ref_cell_reference(ProductStream::new([kp, error]);
let i = make_rc_ref_cell_reference(ProductStream::new([ki, int]));
let d = make_rc_ref_cell_reference(ProductStream::new([kd, drv]));
let pid = make_rc_ref_cell_reference(SumStream::new([p, i, d]));
```
The macro currently only supports `Rc<RefCell>`-variant `Reference`s for output, but it can still take any `Reference` variant for its terms.

## License: BSD 3-Clause
This basically means that you can do whatever you want as long as you give me attribution and you don't remove the license notices or use my name to endorse stuff I don't. Read the actual license for details though.
