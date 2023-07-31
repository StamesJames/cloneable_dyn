# Disclaimer
I'm quite new to rust and even more so to publishing crates so this crate should be used with caution and feedback is welcome :-)
# cloneable_dyn
With this crate you can make your traits cloneablr, your structs/enums cloneable as given cloneable traits and derive clone on structs/enums that contain dyn objects of your traits.
Make a trait cloneable by attaching the attribute #[dyn_cloneable]. This will generate a new supertrait that contains a function `fn clone_dyn(&self) -> Box<dyn #trait_ident>`
Here an Example:
```
use cloneable_dyn::dyn_cloneable;
#[dyn_cloneable]
trait TestTrait {}
// will generate
// pub trait __TestTrait__DynCloneAutoDerive__ {
//    fn clone_dyn(&self) -> Box<dyn TestTrait>;
// }
// and make it a supertrait of TestTrait
```
