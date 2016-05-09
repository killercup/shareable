/* Copyright 2016 Joshua Gentry
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */
use std::sync::{Mutex, Arc};

//*************************************************************************************************
/// Internal data structure that identifies how we are accessing the data.
enum Data<T>
{
    //---------------------------------------------------------------------------------------------
    /// There is only 1 instance of the element.
    Single(Arc<T>),

    //---------------------------------------------------------------------------------------------
    /// There are or were multiple instances of the element.
    Multiple(Arc<Mutex<Arc<T>>>)
}

//*************************************************************************************************
/// Shareable object data element.
///
/// If only 1 instance of the element is needed then that data is just saved as a normal memory
/// location.  If multiple instances are needed then the value is saved in an Mutex so it
/// can be safely shared between threads.
///
/// This object can only store read only data structures.  There is nothing implemented to provide
/// read/write access to objects.
///
/// # Examples
///
/// ```
/// use shareable::SharedObject;
///
/// // Single thread, no expensive structures used.
/// let mut value1 = SharedObject::new(String::from("abc"));
///
/// println!("Value: {}", value1.get());
///
/// value1.set(String::from("xyz"));
///
/// println!("Value: {}", value1.get());
/// ```
///
/// ```
/// use std::sync::mpsc;
/// use std::thread;
/// use shareable::SharedObject;
///
/// // Multiple threads, atomic values are used.
/// let mut value1 = SharedObject::new(String::from("abc"));
/// let mut value2 = value1.dup();
///
/// let (tx, rx) = mpsc::channel();
///
/// let thread = thread::spawn(move || {
///     rx.recv();
///     assert_eq!(*value2.get(), "xyz");
/// });
///
/// value1.set(String::from("xyz"));
///
/// tx.send(());
/// thread.join().unwrap();
/// ```
pub struct SharedObject<T>
{
    //---------------------------------------------------------------------------------------------
    /// The internal data element.
    data : Data<T>
}

impl<T> SharedObject<T>
{
    //********************************************************************************************
    /// Construct a new instance of the object.
    pub fn new(
        value : T
        ) -> SharedObject<T>
    {
        SharedObject {
            data : Data::Single(Arc::new(value))
        }
    }

    //********************************************************************************************
    /// Set the value of the object.
    pub fn set(
        &mut self,
        val : T
        )
    {
        match self.data
        {
            Data::Single(_)         => self.data = Data::Single(Arc::new(val)),
            Data::Multiple(ref mem) => {
                let mut lock = mem.lock().unwrap();

                *lock = Arc::new(val);
            }
        }
    }

    //********************************************************************************************
    /// Returns the value of the object.
    pub fn get(&self) -> Arc<T>
    {
        match self.data
        {
            Data::Single(ref val)   => val.clone(),
            Data::Multiple(ref mem) => {
                let lock = mem.lock().unwrap();

                lock.clone()
            }
        }
    }

    //********************************************************************************************
    /// Clones the object.  After this call all access to the data will be done via an
    /// AtomicIsize element.
    pub fn dup(&mut self) -> SharedObject<T>
    {
        let data = match self.data
        {
            Data::Single(ref val)   => Arc::new(Mutex::new(val.clone())),
            Data::Multiple(ref val) => val.clone()
        };

        self.data = Data::Multiple(data.clone());

        SharedObject { data : Data::Multiple(data) }
    }
}

use std::fmt::{Debug, Display, Formatter, Error};

impl<T : Debug> Debug for SharedObject<T>
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

impl<T : Display> Display for SharedObject<T>
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
    fn single()
    {
        let mut test = super::SharedObject::new(String::from("abc"));

        assert_eq!(*test.get(), "abc");
        test.set(String::from("xyz"));
        assert_eq!(*test.get(), "xyz");
    }

    //*********************************************************************************************
    /// Test that get/set work with multiple instances.
    #[test]
    fn multiple()
    {
        let mut test1 = super::SharedObject::new(String::from("abc"));
        let mut test2 = test1.dup();
        let mut test3 = test2.dup();

        assert_eq!(*test1.get(), "abc");
        assert_eq!(*test2.get(), "abc");
        assert_eq!(*test3.get(), "abc");

        test1.set(String::from("xyz"));

        assert_eq!(*test1.get(), "xyz");
        assert_eq!(*test2.get(), "xyz");
        assert_eq!(*test3.get(), "xyz");

        test2.set(String::from("mno"));

        assert_eq!(*test1.get(), "mno");
        assert_eq!(*test2.get(), "mno");
        assert_eq!(*test3.get(), "mno");

        test3.set(String::from("123"));

        assert_eq!(*test1.get(), "123");
        assert_eq!(*test2.get(), "123");
        assert_eq!(*test3.get(), "123");
    }
}
