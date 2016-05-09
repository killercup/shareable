/* Copyright 2016 Joshua Gentry
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */
use std::sync::{Arc, Mutex};

//*************************************************************************************************
/// Internal data structure that identifies how we are accessing the data.
enum Data
{
    //---------------------------------------------------------------------------------------------
    /// There is only 1 instance of the element.
    Single(f64),

    //---------------------------------------------------------------------------------------------
    /// There are or were multiple instances of the element.
    Multiple(Arc<Mutex<f64>>)
}

//*************************************************************************************************
/// Shareable f64 data element.
///
/// If only 1 instance of the element is needed then that data is just saved as a normal memory
/// location.  If multiple instances are needed then the value is saved in an AtomicIsize if the
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
/// value1.set(31.432);
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
/// let mut value1 = SharedF64::new(63.32);
/// let mut value2 = value1.dup();
///
/// let (tx, rx) = mpsc::channel();
///
/// let thread = thread::spawn(move || {
///     rx.recv();
///     assert_eq!(value2.get(), 31.78);
/// });
///
/// value1.set(31.78);
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
            Data::Multiple(ref mem) => {
                let mut data = mem.lock().unwrap();

                *data = val
            }
        }
    }

    //********************************************************************************************
    /// Returns the value of the object.
    pub fn get(&self) -> f64
    {
        match self.data
        {
            Data::Single(val)       => val,
            Data::Multiple(ref mem) => {
                let data = mem.lock().unwrap();

                *data
            }
        }
    }

    //********************************************************************************************
    /// Clones the object.  After this call all access to the data will be done via an
    /// AtomicUsize element.
    pub fn dup(&mut self) -> SharedF64
    {
        match self.data
        {
            Data::Single(val) => {
                let data = Arc::new(Mutex::new(val));
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
        let mut test = super::SharedF64::new(79.89);

        assert_eq!(test.get(), 79.89);
        test.set(41.12);
        assert_eq!(test.get(), 41.12);
    }

    //*********************************************************************************************
    /// Test that get/set work with multiple instances.
    #[test]
    fn test_multiple()
    {
        let mut test1 = super::SharedF64::new(79.97);
        let mut test2 = test1.dup();
        let mut test3 = test2.dup();

        assert_eq!(test1.get(), 79.97);
        assert_eq!(test2.get(), 79.97);
        assert_eq!(test3.get(), 79.97);

        test1.set(-51.15);

        assert_eq!(test1.get(), -51.15);
        assert_eq!(test2.get(), -51.15);
        assert_eq!(test3.get(), -51.15);

        test2.set(31.31);

        assert_eq!(test1.get(), 31.31);
        assert_eq!(test2.get(), 31.31);
        assert_eq!(test3.get(), 31.31);

        test3.set(11.87);

        assert_eq!(test1.get(), 11.87);
        assert_eq!(test2.get(), 11.87);
        assert_eq!(test3.get(), 11.87);
    }
}
