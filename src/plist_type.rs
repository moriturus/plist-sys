/*
The MIT License (MIT)

Copyright (c) 2017 Henrique Sasaki Yuya

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

//! Plist node types.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Plist node types.
pub enum PlistType {
    /// Boolean, scalar type.
    Boolean,
    /// Unsigned integer, scalar type.
    UInt,
    /// Real, scalar type.
    Real,
    /// ASCII string, scalar type.
    String,
    /// Ordered array, structured type.
    Array,
    /// Unordered dictionary (key/value pair), structured type.
    Dictionary,
    /// Date, scalar type.
    Date,
    /// Binary data, scalar type.
    Data,
    /// Key in dictionaries (ASCII String), scalar type.
    Key,
    /// Special type used for 'keyed encoding'.
    Uid,
    /// No type.
    None,
}

impl PlistType {
    /// Whether or not `self` is `Boolean` type.
    pub fn is_boolean(self) -> bool {
        self == PlistType::Boolean
    }

    /// Whether or not `self` is `UInt` type.
    pub fn is_uint(self) -> bool {
        self == PlistType::UInt
    }

    /// Whether or not `self` is `Real` type.
    pub fn is_real(self) -> bool {
        self == PlistType::Real
    }

    /// Whether or not `self` is `String` type.
    pub fn is_string(self) -> bool {
        self == PlistType::String
    }

    /// Whether or not `self` is `Array` type.
    pub fn is_array(self) -> bool {
        self == PlistType::Array
    }

    /// Whether or not `self` is `Dictionary` type.
    pub fn is_dictionary(self) -> bool {
        self == PlistType::Dictionary
    }

    /// Whether or not `self` is `Date` type.
    pub fn is_date(self) -> bool {
        self == PlistType::Date
    }

    /// Whether or not `self` is `Data` type.
    pub fn is_data(self) -> bool {
        self == PlistType::Data
    }

    /// Whether or not `self` is `Key` type.
    pub fn is_key(self) -> bool {
        self == PlistType::Key
    }

    /// Whether or not `self` is `Uid` type.
    pub fn is_uid(self) -> bool {
        self == PlistType::Uid
    }
}

#[test]
fn test_plist_types() {
    let boolean = PlistType::Boolean;
    assert!(boolean.is_boolean());
    let uint = PlistType::UInt;
    assert!(uint.is_uint());
    let real = PlistType::Real;
    assert!(real.is_real());
    let string = PlistType::String;
    assert!(string.is_string());
    let array = PlistType::Array;
    assert!(array.is_array());
    let dic = PlistType::Dictionary;
    assert!(dic.is_dictionary());
    let date = PlistType::Date;
    assert!(date.is_date());
    let data = PlistType::Data;
    assert!(data.is_data());
    let key = PlistType::Key;
    assert!(key.is_key());
    let uid = PlistType::Uid;
    assert!(uid.is_uid());
}
