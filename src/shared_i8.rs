/* Copyright 2016 Joshua Gentry
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */
use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};

//*************************************************************************************************
/// Internal data structure that identifies how we are accessing the data.
enum Data
{
    //---------------------------------------------------------------------------------------------
    /// There is only 1 instance of the element.
    Single(i8),

    //---------------------------------------------------------------------------------------------
    /// There are or were multiple instances of the element.
    Multiple(Arc<AtomicIsize>)
}

//*************************************************************************************************
/// Shareable i8 data element.
///
/// If only 1 instance of the element is needed then that data is just saved as a normal memory
/// location.  If multiple instances are needed then the value is saved in an AtomicIsize so it
/// can be safely shared between threads.
///
/// # Examples
///
/// ```
/// use shareable::SharedI8;
///
/// // Single thread, no expensive structures used.
/// let mut value1 = SharedI8::new(63);
///
/// println!("Value: {}", value1.get());
///
/// value1.set(31);
///
/// println!("Value: {}", value1.get());
/// ```
///
/// ```
/// use std::sync::mpsc;
/// use std::thread;
/// use shareable::SharedI8;
///
/// // Multiple threads, atomic values are used.
/// let mut value1 = SharedI8::new(63);
/// let mut value2 = value1.dup();
///
/// let (tx, rx) = mpsc::channel();
///
/// let thread = thread::spawn(move || {
///     rx.recv();
///     assert_eq!(value2.get(), 31);
/// });
///
/// value1.set(31);
///
/// tx.send(());
/// thread.join().unwrap();
/// ```
pub struct SharedI8
{
    //---------------------------------------------------------------------------------------------
    /// The internal data element.
    data : Data
}

impl SharedI8
{
    //********************************************************************************************
    /// Construct a new instance of the object.
    pub fn new(
        value : i8
        ) -> SharedI8
    {
        SharedI8 {
            data : Data::Single(value)
        }
    }

    //********************************************************************************************
    /// Set the value of the object.
    pub fn set(
        &mut self,
        val : i8
        )
    {
        match self.data
        {
            Data::Single(_)         => self.data = Data::Single(val),
            Data::Multiple(ref mem) => mem.store(val as isize, Ordering::Relaxed)
        }
    }

    //********************************************************************************************
    /// Returns the value of the object.
    pub fn get(&self) -> i8
    {
        match self.data
        {
            Data::Single(val)       => val,
            Data::Multiple(ref mem) => mem.load(Ordering::Relaxed) as i8
        }
    }

    //********************************************************************************************
    /// Clones the object.  After this call all access to the data will be done via an
    /// AtomicUsize element.
    pub fn dup(&mut self) -> SharedI8
    {
        match self.data
        {
            Data::Single(val) => {
                let data = Arc::new(AtomicIsize::new(val as isize));
                self.data = Data::Multiple(data.clone());

                SharedI8 { data : Data::Multiple(data) }
            },
            Data::Multiple(ref val) => {
                SharedI8 { data : Data::Multiple(val.clone()) }
            }
        }
    }
}

use std::fmt::{Debug, Display, Formatter, Error};

impl Debug for SharedI8
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

impl Display for SharedI8
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
        let mut test = super::SharedI8::new(79);

        assert_eq!(test.get(), 79);
        test.set(41);
        assert_eq!(test.get(), 41);
    }

    //*********************************************************************************************
    /// Test that get/set work with multiple instances.
    #[test]
    fn test_multiple()
    {
        let mut test1 = super::SharedI8::new(-79);
        let mut test2 = test1.dup();
        let mut test3 = test2.dup();

        assert_eq!(test1.get(), -79);
        assert_eq!(test2.get(), -79);
        assert_eq!(test3.get(), -79);

        test1.set(-51);

        assert_eq!(test1.get(), -51);
        assert_eq!(test2.get(), -51);
        assert_eq!(test3.get(), -51);

        test2.set(-31);

        assert_eq!(test1.get(), -31);
        assert_eq!(test2.get(), -31);
        assert_eq!(test3.get(), -31);

        test3.set(11);

        assert_eq!(test1.get(), 11);
        assert_eq!(test2.get(), 11);
        assert_eq!(test3.get(), 11);
    }
}
