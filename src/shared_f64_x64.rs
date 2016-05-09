/* Copyright 2016 Joshua Gentry
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */
use std::mem::transmute;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

//*************************************************************************************************
/// Internal data structure that identifies how we are accessing the data.
enum Data
{
    //---------------------------------------------------------------------------------------------
    /// There is only 1 instance of the element.
    Single(f64),

    //---------------------------------------------------------------------------------------------
    /// There are or were multiple instances of the element.
    Multiple(Arc<AtomicUsize>)
}

//*************************************************************************************************
/// Shareable f64 data element.
///
/// If only 1 instance of the element is needed then that data is just saved as a normal memory
/// location.   If multiple instances are needed then the value is saved in an AtomicUsize if the
/// application is 64 bit or a Mutex if the application is 32 bit so it can be safely shared
/// between threads.
///
/// # Examples
///
/// ```
/// use shareable::SharedF64;
///
/// // Single thread, no expensive structures used.
/// let mut value1 = SharedF64::new(63.23);
///
/// println!("Value: {}", value1.get());
///
/// value1.set(78.3);
///
/// println!("Value: {}", value1.get());
/// ```
///
/// ```
/// use std::sync::mpsc;
/// use std::thread;
/// use shareable::SharedF64;
///
/// // Multiple threads, atomic values are used.
/// let mut value1 = SharedF64::new(63.23);
/// let mut value2 = value1.dup();
///
/// let (tx, rx) = mpsc::channel();
///
/// let thread = thread::spawn(move || {
///     rx.recv();
///     assert_eq!(value2.get(), 31.83);
/// });
///
/// value1.set(31.83);
///
/// tx.send(());
/// thread.join().unwrap();
/// ```
pub struct SharedF64
{
    //---------------------------------------------------------------------------------------------
    /// The internal data element.
    data : Data
}

impl SharedF64
{
    //********************************************************************************************
    /// Construct a new instance of the object.
    pub fn new(
        value : f64
        ) -> SharedF64
    {
        SharedF64 {
            data : Data::Single(value)
        }
    }

    //********************************************************************************************
    /// Set the value of the object.
    pub fn set(
        &mut self,
        val : f64
        )
    {
        match self.data
        {
            Data::Single(_)         => self.data = Data::Single(val),
            Data::Multiple(ref mem) => unsafe { mem.store(transmute(val), Ordering::Relaxed) }
        }
    }

    //********************************************************************************************
    /// Returns the value of the object.
    pub fn get(&self) -> f64
    {
        match self.data
        {
            Data::Single(val)       => val,
            Data::Multiple(ref mem) => unsafe { transmute(mem.load(Ordering::Relaxed)) }
        }
    }

    //********************************************************************************************
    /// Clones the object.  After this call all access to the data will be done via an
    /// AtomicIsize element.
    pub fn dup(&mut self) -> SharedF64
    {
        match self.data
        {
            Data::Single(val) => {
                let data = unsafe { Arc::new(AtomicUsize::new(transmute(val))) };
                self.data = Data::Multiple(data.clone());

                SharedF64 { data : Data::Multiple(data) }
            },
            Data::Multiple(ref val) => {
                SharedF64 { data : Data::Multiple(val.clone()) }
            }
        }
    }
}

use std::fmt::{Debug, Display, Formatter, Error};

impl Debug for SharedF64
{
    //*********************************************************************************************
    /// Implementation of Debug.
    fn fmt(
        &self,
        f : &mut Formatter
        ) -> Result<(), Error>
    {
        write!(f, "{:?}", self.get())
    }
}

impl Display for SharedF64
{
    //*********************************************************************************************
    /// Implementation of Display.
    fn fmt(
        &self,
        f : &mut Formatter
        ) -> Result<(), Error>
    {
        write!(f, "{}", self.get())
    }
}

#[cfg(test)]
mod tests
{

    //*********************************************************************************************
    /// Test that get/set work with only 1 instance.
    #[test]
    fn test_single()
    {
        let mut test = super::SharedF64::new(-79.23);

        assert_eq!(test.get(), -79.23);
        test.set(41.78);
        assert_eq!(test.get(), 41.78);
    }

    //*********************************************************************************************
    /// Test that get/set work with multiple instances.
    #[test]
    fn test_multiple()
    {
        let mut test1 = super::SharedF64::new(-79.6);
        let mut test2 = test1.dup();
        let mut test3 = test2.dup();

        assert_eq!(test1.get(), -79.6);
        assert_eq!(test2.get(), -79.6);
        assert_eq!(test3.get(), -79.6);

        test1.set(51.98);

        assert_eq!(test1.get(), 51.98);
        assert_eq!(test2.get(), 51.98);
        assert_eq!(test3.get(), 51.98);

        test2.set(31.77);

        assert_eq!(test1.get(), 31.77);
        assert_eq!(test2.get(), 31.77);
        assert_eq!(test3.get(), 31.77);

        test3.set(-11.101);

        assert_eq!(test1.get(), -11.101);
        assert_eq!(test2.get(), -11.101);
        assert_eq!(test3.get(), -11.101);
    }
}
