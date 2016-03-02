use std::any::Any;
use std::boxed::Box;

use engine::{EvalError, Engine, Arity0, Arity1, Arity2, Arity3, Arity4, Arity5, Arity6};

pub trait FnRegister {
    fn register(self, engine: &mut Engine, name: &str);
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone, X: Any+Clone, Y: Any+Clone, Z: Any+Clone> FnRegister for fn(&mut T, U, V, W, X, Y)->Z {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>, 
            &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>, arg4: &mut Box<Any>,
                    arg5: &mut Box<Any>, arg6: &mut Box<Any>| {
                    
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    let inside4 = (*arg4).downcast_mut() as Option<&mut W>;
                    let inside5 = (*arg5).downcast_mut() as Option<&mut X>;
                    let inside6 = (*arg6).downcast_mut() as Option<&mut Y>;
                    
                    match (inside1, inside2, inside3, inside4, inside5, inside6) {
                        (Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) => Ok(Box::new(self(b, c.clone(), d.clone(), 
                            e.clone(), f.clone(), g.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_6.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity6::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone, X: Any+Clone, Y: Any+Clone, Z: Any+Clone> FnRegister for fn(T, U, V, W, X, Y)->Z {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>, 
            &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>, arg4: &mut Box<Any>,
                    arg5: &mut Box<Any>, arg6: &mut Box<Any>| {

                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    let inside4 = (*arg4).downcast_mut() as Option<&mut W>;
                    let inside5 = (*arg5).downcast_mut() as Option<&mut X>;
                    let inside6 = (*arg6).downcast_mut() as Option<&mut Y>;
                    
                    match (inside1, inside2, inside3, inside4, inside5, inside6) {
                        (Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) => Ok(Box::new(self(b.clone(), c.clone(), d.clone(), 
                            e.clone(), f.clone(), g.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_6.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity6::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone, X: Any+Clone, Y: Any+Clone> FnRegister for fn(&mut T, U, V, W, X)->Y {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>, arg4: &mut Box<Any>,
                    arg5: &mut Box<Any>| {

                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    let inside4 = (*arg4).downcast_mut() as Option<&mut W>;
                    let inside5 = (*arg5).downcast_mut() as Option<&mut X>;
                    
                    match (inside1, inside2, inside3, inside4, inside5) {
                        (Some(b), Some(c), Some(d), Some(e), Some(f)) => Ok(Box::new(self(b, c.clone(), d.clone(), 
                            e.clone(), f.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_5.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity5::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone, X: Any+Clone, Y: Any+Clone> FnRegister for fn(T, U, V, W, X)->Y {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>, arg4: &mut Box<Any>,
                    arg5: &mut Box<Any>| {

                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    let inside4 = (*arg4).downcast_mut() as Option<&mut W>;
                    let inside5 = (*arg5).downcast_mut() as Option<&mut X>;
                    
                    match (inside1, inside2, inside3, inside4, inside5) {
                        (Some(b), Some(c), Some(d), Some(e), Some(f)) => Ok(Box::new(self(b.clone(), c.clone(), d.clone(), 
                            e.clone(), f.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_5.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity5::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone, X: Any+Clone> FnRegister for fn(&mut T, U, V, W)->X {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>, arg4: &mut Box<Any>| {
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    let inside4 = (*arg4).downcast_mut() as Option<&mut W>;
                    
                    match (inside1, inside2, inside3, inside4) {
                        (Some(b), Some(c), Some(d), Some(e)) => Ok(Box::new(self(b, c.clone(), d.clone(), e.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_4.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity4::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone, X: Any+Clone> FnRegister for fn(T, U, V, W)->X {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>, arg4: &mut Box<Any>| {
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    let inside4 = (*arg4).downcast_mut() as Option<&mut W>;
                    
                    match (inside1, inside2, inside3, inside4) {
                        (Some(b), Some(c), Some(d), Some(e)) => Ok(Box::new(self(b.clone(), c.clone(), d.clone(), e.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_4.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity4::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone> FnRegister for fn(&mut T, U, V)->W {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>| {
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    
                    match (inside1, inside2, inside3) {
                        (Some(b), Some(c), Some(d)) => Ok(Box::new(self(b, c.clone(), d.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_3.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity3::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone, W: Any+Clone> FnRegister for fn(T, U, V)->W {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>, arg3: &mut Box<Any>| {
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    let inside3 = (*arg3).downcast_mut() as Option<&mut V>;
                    
                    match (inside1, inside2, inside3) {
                        (Some(b), Some(c), Some(d)) => Ok(Box::new(self(b.clone(), c.clone(), d.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_3.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity3::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone> FnRegister for fn(&mut T, U)->V {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>| {
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    
                    match (inside1, inside2) {
                        (Some(b), Some(c)) => Ok(Box::new(self(b, c.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_2.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity2::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone, V: Any+Clone> FnRegister for fn(T, U)->V {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>, &mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg1: &mut Box<Any>, arg2: &mut Box<Any>| {
                    let inside1 = (*arg1).downcast_mut() as Option<&mut T>;
                    let inside2 = (*arg2).downcast_mut() as Option<&mut U>;
                    
                    match (inside1, inside2) {
                        (Some(b), Some(c)) => Ok(Box::new(self(b.clone(), c.clone())) as Box<Any>),
                        _ => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_2.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity2::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone> FnRegister for fn(&mut T)->U {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn(&mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg: &mut Box<Any>| {
                    let inside = (*arg).downcast_mut() as Option<&mut T>;
                    
                    match inside {
                        Some(b) => Ok(Box::new(self(b)) as Box<Any>),
                        None => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_1.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity1::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone, U: Any+Clone> FnRegister for fn(T)->U {
    fn register(self, engine: &mut Engine, name: &str) {        
        let wrapped : Box<Fn(&mut Box<Any>)->Result<Box<Any>, EvalError>> = 
            Box::new(
                move |arg: &mut Box<Any>| {
                    let inside = (*arg).downcast_mut() as Option<&mut T>;
                    match inside {
                        Some(b) => Ok(Box::new(self(b.clone())) as Box<Any>),
                        None => Err(EvalError::FunctionArgMismatch)
                    }
                }
            );

        let ent = engine.fns_arity_1.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity1::ExternalFn(wrapped));
    }
}

impl<T: Any+Clone> FnRegister for fn()->T {
    fn register(self, engine: &mut Engine, name: &str) {
        let wrapped : Box<Fn()->Result<Box<Any>, EvalError>> = 
            Box::new(
                move || { Ok(Box::new(self()) as Box<Any>) }
            );

        let ent = engine.fns_arity_0.entry(name.to_string()).or_insert(Vec::new());
        (*ent).push(Arity0::ExternalFn(wrapped));
    }
}
